use crate::mini_multipart::Multipart;
use futures_util::stream::StreamExt;
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
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for (index, path) in file_paths.iter().enumerate() {
        let path = path.clone();
        let app_handle = app_handle.clone();
        let show_name_clone = show_name.clone();
        let addr = String::from("1.1.1.1:7999");
        let handle = tokio::spawn(async move {
            let mut file_stream = FileStream::new(&path, show_name_clone, addr, file_type).await;
            file_stream.init_stream().await;

            let total_size = file_stream.file_size;
            let (progress_tx, mut progress_rx) = channel::<usize>(100);

            let _progress_handle = tokio::spawn(async move {
                let start_time = std::time::Instant::now();
                while let Some(progress) = progress_rx.recv().await {
                    let percentage = (progress as f64 / total_size as f64) * 100.0;
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = (progress as f64 / (1024.0 * 1024.0)) / elapsed;
                    let time_remaining =
                        (total_size as f64 - progress as f64) / (speed * 1024.0) / 1000.0;

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
            let mut reader =
                FramedRead::with_capacity(file_stream.file, BytesCodec::new(), 1024 * 1024 * 4);
            while let Some(chunk) = reader.next().await {
                let chunk = chunk.unwrap();
                file_stream.stream.write_all(&chunk).await.unwrap();
                total_written += chunk.len();
                progress_tx.send(total_written).await.unwrap();
                if total_written == total_size {
                    println!("breaking");
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok("".to_string())
}

struct FileStream<'a> {
    file_type: FileType,
    stream: TcpStream,
    // from_path: &'a Path,
    file: File,
    to_path: String,
    file_name: &'a str,
    file_size: usize,
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

    fn as_u16(&self) -> u16 {
        match self {
            FileType::Movie => 1,
            FileType::Show => 2,
            FileType::Image => 3,
            FileType::Document => 4,
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

impl FileStream<'_> {
    async fn new(path: &String, dest: String, addr: String, file_type: u16) -> FileStream {
        let file = File::open(path).await.unwrap();
        let from_path = Path::new(path);
        let file_name = from_path.file_name().unwrap().to_str().unwrap();
        let file_size = file.metadata().await.unwrap().len() as usize;
        let stream = TcpStream::connect(addr).await.unwrap();
        stream.set_nodelay(true).unwrap();

        FileStream {
            file,
            file_name,
            file_type: FileType::from_u16(file_type),
            file_size,
            stream,
            to_path: dest,
        }
    }

    async fn init_stream(&mut self) {
        let file_name_size = self.file_name.len() as u16;
        self.stream
            .write_all(&file_name_size.to_be_bytes())
            .await
            .unwrap();
        self.stream
            .write_all(&self.file_name.as_bytes())
            .await
            .unwrap();
        self.stream
            .write_all(&self.file_type.as_u16().to_be_bytes())
            .await
            .unwrap();
        self.file_type
            .stream_type(&mut self.stream, self.to_path.clone())
            .await;
    }
}
