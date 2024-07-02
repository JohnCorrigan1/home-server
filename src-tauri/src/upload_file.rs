use crate::mini_multipart::Multipart;
use futures_util::stream::StreamExt;
use serde::de::IntoDeserializer;
use std::env;
use std::path::Path;
use tauri::Manager;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::channel;
use tokio::task::JoinHandle;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tauri::command]
pub async fn file_upload(
    file_path: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let addr = env::var("TAURI_UPLOAD_SERVER").expect("TAURI_UPLOAD_SERVER not set");
    println!("addr: {}", addr);
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
            let time_remaining = (total_size as f64 - progress as f64) / (speed * 1024.0) / 1000.0;

            app_handle
                .emit_all(
                    "upload-progress",
                    (percentage, speed, elapsed, time_remaining),
                )
                .unwrap();
        }
    });

    let multipart = Multipart::new(&file_name, total_size);

    let endpoint = "/api/upload";
    let mut stream = TcpStream::connect(&addr).await.map_err(|e| e.to_string())?;

    let request_header = multipart.request_header(&addr, endpoint);

    stream
        .write_all(&request_header)
        .await
        .map_err(|e| e.to_string())?;

    stream
        .write_all(&multipart.header)
        .await
        .map_err(|e| e.to_string())?;

    let mut total_written = multipart.header.len();
    progress_tx.send(total_written).await.unwrap();

    //read by 100mb chunks
    let mut reader = FramedRead::with_capacity(file, BytesCodec::new(), 4 * 1024 * 1024);
    // let mut reader = FramedRead::new(file, BytesCodec::new());

    while let Some(chunk) = reader.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        stream.write_all(&chunk).await.map_err(|e| e.to_string())?;
        total_written += chunk.len();
        progress_tx.send(total_written).await.unwrap();
    }

    stream
        .write_all(&multipart.footer)
        .await
        .map_err(|e| e.to_string())?;

    total_written += multipart.footer.len();
    progress_tx.send(total_written).await.unwrap();

    stream.flush().await.unwrap();

    let mut response = vec![];
    stream.read_to_end(&mut response).await.unwrap();

    let response = String::from_utf8_lossy(&response).to_string();
    match response.starts_with("HTTP/1.1 200 OK") {
        true => Ok(format!("Uploaded: {}", file_name)),
        false => Err(format!("Failed to upload: {}", response)),
    }
}

#[tauri::command]
pub async fn file_stream(
    file_paths: Vec<String>,
    file_type: u16,
    show_name: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let addr = env::var("FILE_UPLOAD_SERVER").expect("FILE_UPLOAD_SERVER not set");
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    println!("filetype: {}", &file_type);

    // let show_name_clone = show_name.clone();
    for (index, path) in file_paths.iter().enumerate() {
        let addr = addr.clone();
        let path = path.clone();
        let app_handle = app_handle.clone();
        let show_name_clone = show_name.clone();
        let handle = tokio::spawn(async move {
            let mut stream = TcpStream::connect(addr).await.unwrap();
            stream.set_nodelay(true).unwrap();
            let path = Path::new(&path);
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let file = File::open(path).await.unwrap();
            let filename_size = file_name.len() as u16;
            // Send the filename size and filename
            stream
                .write_all(&filename_size.to_be_bytes())
                .await
                .unwrap();

            stream.write_all(file_name.as_bytes()).await.unwrap();

            stream.write_all(&file_type.to_be_bytes()).await.unwrap();

            let file_type = FileType::from_u16(file_type);

            file_type.stream_type(&mut stream, show_name_clone).await;

            let total_size = file.metadata().await.unwrap().len() as usize;
            let (progress_tx, mut progress_rx) = channel::<usize>(100);

            let _progress_handle = tokio::spawn(async move {
                let start_time = std::time::Instant::now();
                while let Some(progress) = progress_rx.recv().await {
                    let percentage = (progress as f64 / total_size as f64) * 100.0;
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = (progress as f64 / (1024.0 * 1024.0)) / elapsed;
                    let time_remaining =
                        (total_size as f64 - progress as f64) / (speed * 1024.0) / 1000.0;
                    // println!("Progress handle emit: {}", index);
                    //only update every 1 second
                    if (elapsed * 10.0) as i64 % 10 == 0 || percentage == 100.0 {
                        app_handle
                            .emit_all(
                                &format!("upload-progress-{}", index),
                                (percentage, speed, elapsed, time_remaining),
                            )
                            .unwrap();
                    }
                }
            });

            let mut total_written = 0;
            //let mut buf = vec![0; 1024 * 1024 * 4];
            let mut reader = FramedRead::with_capacity(file, BytesCodec::new(), 1024 * 1024 * 4);
            while let Some(chunk) = reader.next().await {
                let chunk = chunk.unwrap();
                stream.write_all(&chunk).await.unwrap();
                total_written += chunk.len();
                progress_tx.send(total_written).await.unwrap();
                if total_written == total_size {
                    println!("breaking");
                    break;
                }
            }
            // println!("Progress handle await");
            // progress_handle.await.unwrap();
            // println!("Progress handle done");
        });
        handles.push(handle);
    }
    println!("Handles await");
    for handle in handles {
        handle.await.unwrap();
        // handle.await.unwrap();
    }
    println!("Handles done");
    Ok("".to_string())
}

enum FileType {
    Movie,
    Show,
    Image,
    Document,
}

impl FileType {
    fn from_u16(value: u16) -> Self {
        match value {
            1 => Self::Movie,
            2 => Self::Show,
            3 => Self::Image,
            4 => Self::Document,
            _ => panic!("Invalid file type"),
        }
    }

    async fn stream_type(&self, stream: &mut TcpStream, show_name: String) {
        match self {
            Self::Show => {
                let show_name_size: u16 = show_name.len() as u16;
                stream
                    .write_all(&show_name_size.to_be_bytes())
                    .await
                    .unwrap();
                stream.write_all(show_name.as_bytes()).await.unwrap();
            }
            _ => {}
        }
    }
}
