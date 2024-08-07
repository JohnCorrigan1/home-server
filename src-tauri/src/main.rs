// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mini_multipart;
mod upload_file;

use std::env;
use upload_file::{file_stream, file_upload};

#[tokio::main]
async fn main() {
    let _ = fix_path_env::fix();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![file_upload, file_stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
