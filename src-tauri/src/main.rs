// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use async_stream;
use reqwest::blocking::Client;
// use reqwest::multipart;
// use reqwest::Client;
use reqwest::blocking::multipart;
use reqwest::Body;
use std::io::Read;
use std::path::Path;
use tauri::{Manager, State};
use tokio::task;
use tokio::{fs::File, sync::mpsc::channel};
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;
//where is async stream
fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn upload_file(file_path: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let path_clone = file_path.clone();

    let path = Path::new(&file_path);
    // let path_clone = path.clone();
    let file = File::open(path).await.unwrap();

    // let file_name = path.file_name().unwrap().to_str().unwrap();

    let total_size = file.metadata().await.unwrap().len();
    let mut reader_stream = ReaderStream::new(file);

    let (progress_tx, mut progress_rx) = channel::<usize>(100);
    let app_handle = app_handle.clone();

    tokio::task::spawn(async move {
        while let Some(progress) = progress_rx.recv().await {
            let percentage = (progress as f64 / total_size as f64) * 100.0;
            app_handle.emit_all("upload-progress", percentage).unwrap();
        }
    });

    task::spawn_blocking(move || {
        let async_stream = async_stream::stream! {
            let mut progress = 0;
            while let Some(chunk) = reader_stream.next().await {
                if let Ok(chunk) = &chunk {
                    progress += chunk.len();
                    progress_tx.send(progress).await.unwrap();
                    // progress += chunk.len();
                    // progress_tx.send(progress).await.unwrap();
                    // let percentage = (progress as f64 / total_size as f64) * 100.0;

                    // app_handle
                        // .emit_all("upload-progress", percentage)
                        // .unwrap();
                }

                yield chunk;
            }
        };

        // let path_clone = path.clone();
        let path = Path::new(&path_clone);
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // let file_name = path_clone.file_name().unwrap().to_str().unwrap();

        let form = multipart::Form::new().part(
            "image",
            multipart::Part::reader(Body::wrap_stream(async_stream))
                .file_name(file_name.to_string())
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        println!("Uploading file from rs: {}", file_name);
        let client = Client::new();
        let response = client
            .post("http://192.168.86.81:8000/api/upload")
            .multipart(form)
            .send();

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Server returned status code: {}", res.status()))
                }
            }
            Err(err) => Err(format!("Error: {}", err)),
        }
        // let response = .send().await.unwrap();
        // let response = response.unwrap();
        // let status = response.status();
        // if !status.is_success() {
        //     return Err(format!("Server returned status code: {}", status));
        // } else {
        //     let body = response.text().unwrap();
        //     Ok(body)
        // }
        // match response {
        //     Ok(res) => {
        //         let status = res.status();
        //         if !status.is_success() {
        //             return Err(format!("Server returned status code: {}", status));
        //         } else {
        //             let body = res.text().unwrap();
        //             Ok(body)
        //         }
        //     }
        //     Err(err) => Err(format!("Error: {}", err)),
        // }
    })
    .await
    .unwrap()
    // .unwrap();

    // Ok("Uploaded".to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
