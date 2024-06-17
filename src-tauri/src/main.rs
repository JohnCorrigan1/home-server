// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use reqwest::multipart;
// use reqwest::Body;
// use reqwest::Client;
// use std::path::Path;
// use tauri::Manager;
// use tokio::{fs::File, sync::mpsc::channel};
// use tokio_stream::StreamExt;
// use tokio_util::io::ReaderStream;
// // use reqwest::
// fn main() {
//     let _ = fix_path_env::fix();
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet, upload_file])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// #[tauri::command]
// async fn upload_file(file_path: String, app_handle: tauri::AppHandle) -> Result<String, String> {
//     let path_clone = file_path.clone();

//     let path = Path::new(&file_path);
//     // let path_clone = path.clone();
//     let file = File::open(path).await.unwrap();

//     // let file_name = path.file_name().unwrap().to_str().unwrap();

//     let total_size = file.metadata().await.unwrap().len();
//     // let mut reader_stream = ReaderStream::new(file);

//     let (progress_tx, mut progress_rx) = channel::<usize>(100);
//     let app_handle = app_handle.clone();

//     tokio::task::spawn(async move {
//         while let Some(progress) = progress_rx.recv().await {
//             let percentage = (progress as f64 / total_size as f64) * 100.0;
//             app_handle.emit_all("upload-progress", percentage).unwrap();
//         }
//     });
//     let progress_tx_clone = progress_tx.clone();
//     let async_stream = async_stream::stream! {
//         let mut progress = 0;
//         let mut reader_stream = ReaderStream::new(file);
//         while let Some(chunk) = reader_stream.next().await {
//             if let Ok(chunk) = &chunk {
//                 progress += chunk.len();
//                 progress_tx_clone.send(progress).await.unwrap();
//             }
//             yield chunk;
//         }
//     };

//     let path = Path::new(&path_clone);
//     let file_name = path.file_name().unwrap().to_str().unwrap();
//     let progress_tx_clone = progress_tx.clone();
//     let form = multipart::Form::new().part(
//         "image",
//         multipart::Part::stream(Body::wrap_stream(async_stream.map(move |chunk| {
//             if let Ok(ref chunk) = &chunk {
//                 let chunk_len = chunk.len();
//                 progress_tx_clone.try_send(chunk_len).ok();
//             }
//             chunk
//         })))
//         .file_name(file_name.to_string())
//         .mime_str("image/jpeg")
//         .unwrap(),
//     );

//     println!("Uploading file from rs: {}", file_name);
//     let client = Client::new();
//     let response = client
//         .post("http://192.168.86.224:8000/api/upload")
//         .multipart(form)
//         .send()
//         .await;

//     match response {
//         Ok(_) => Ok(format!("Uploaded: {}", file_name)),
//         Err(e) => Err(e.to_string()),
//     }
// }

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::stream::StreamExt;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::header::CONTENT_DISPOSITION;
use hyper::Request;
use hyper_util::rt::TokioIo;
use std::path::Path;
use tauri::Manager;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Sender};
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() {
    let _ = fix_path_env::fix();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn upload_file(file_path: String, app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = Path::new(&file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let file = File::open(path).await.map_err(|e| e.to_string())?;
    let total_size = file.metadata().await.map_err(|e| e.to_string())?.len() as usize;
    let (progress_tx, mut progress_rx) = channel::<usize>(100);
    let app_handle = app_handle.clone();

    tokio::spawn(async move {
        let start_time = std::time::Instant::now();
        while let Some(progress) = progress_rx.recv().await {
            let percentage = (progress as f64 / total_size as f64) * 100.0;
            let elapsed = start_time.elapsed().as_secs_f64();
            let speed = (progress as f64 / (1024.0 * 1024.0)) / elapsed;

            app_handle
                .emit_all("upload-progress", (percentage, speed))
                .unwrap();
        }
    });

    // Prepare multipart form data headers
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let header = format!(
        "--{}\r\n\
         Content-Disposition: form-data; name=\"image\"; filename=\"{}\"\r\n\
         Content-Type: application/octet-stream\r\n\r\n",
        boundary, file_name
    )
    .into_bytes();
    let footer = format!("\r\n--{}--\r\n", boundary).into_bytes();

    // Calculate content length
    let content_length = header.len() + total_size + footer.len();

    // Connect to the API endpoint
    let addr = "192.168.86.224:8000";
    let mut stream = TcpStream::connect(addr).await.map_err(|e| e.to_string())?;

    // Send HTTP POST request headers
    let request_header = format!(
        "POST /api/upload HTTP/1.1\r\n\
         Host: {}\r\n\
         Content-Type: multipart/form-data; boundary={}\r\n\
         Content-Length: {}\r\n\
         \r\n",
        addr, boundary, content_length
    );
    println!("Sending request header:\n{}", request_header);
    stream
        .write_all(request_header.as_bytes())
        .await
        .map_err(|e| e.to_string())?;

    // Send multipart form data header
    stream.write_all(&header).await.map_err(|e| e.to_string())?;
    let mut total_written = header.len();
    progress_tx.send(total_written).await.unwrap();

    // Read file and send it in chunks
    let mut reader = FramedRead::new(file, BytesCodec::new());
    while let Some(chunk) = reader.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        stream.write_all(&chunk).await.map_err(|e| e.to_string())?;
        total_written += chunk.len();
        progress_tx.send(total_written).await.unwrap();
    }

    // Send the multipart form data footer
    stream.write_all(&footer).await.map_err(|e| e.to_string())?;
    total_written += footer.len();
    progress_tx.send(total_written).await.unwrap();

    // Flush the stream and read the response
    stream.flush().await.map_err(|e| e.to_string())?;
    let mut response = vec![];
    stream
        .read_to_end(&mut response)
        .await
        .map_err(|e| e.to_string())?;

    println!("Received response:\n{}", String::from_utf8_lossy(&response));

    if response.starts_with(b"HTTP/1.1 200") {
        Ok(format!("Uploaded: {}", file_name))
    } else {
        Err(format!(
            "Upload failed: {}",
            String::from_utf8_lossy(&response)
        ))
    }
}

#[tauri::command]
async fn upload_refactor(
    file_path: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let path = Path::new(&file_path);
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let file = File::open(path).await.map_err(|e| e.to_string())?;
    let total_size = file.metadata().await.map_err(|e| e.to_string())?.len() as usize;
    let (progress_tx, mut progress_rx) = channel::<usize>(100);
    let app_handle = app_handle.clone();

    tokio::spawn(async move {
        let start_time = std::time::Instant::now();
        while let Some(progress) = progress_rx.recv().await {
            let percentage = (progress as f64 / total_size as f64) * 100.0;
            let elapsed = start_time.elapsed().as_secs_f64();
            let speed = (progress as f64 / (1024.0 * 1024.0)) / elapsed;

            app_handle
                .emit_all("upload-progress", (percentage, speed))
                .unwrap();
        }
    });

    let addr = "192.168.86.224:8000";
    let mut stream = TcpStream::connect(addr).await.map_err(|e| e.to_string())?;
    let headers = hyper::HeaderMap::new();
    // headers.append(key, value)

    Ok("".to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
