use crate::mini_multipart::Multipart;
use futures_util::stream::StreamExt;
use std::env;
use std::path::Path;
use tauri::Manager;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::channel;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tauri::command]
pub async fn file_upload(
    file_path: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let addr = env::var("TAURI_UPLOAD_SERVER").expect("TAURI_UPLOAD_SERVER not set");
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
            let time_remaining = (total_size as f64 - progress as f64) / (speed * 1024.0);

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
    let mut reader = FramedRead::with_capacity(file, BytesCodec::new(), 100 * 1024 * 1024);

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
