// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use reqwest::blocking::multipart;
use std::path::Path;

fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn upload_file(file_path: Vec<&str>) -> Result<String, String> {
    let path = Path::new(file_path[0]);
    let form = multipart::Form::new().part("image", multipart::Part::file(path).unwrap());

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://192.168.86.81:8000/api/upload")
        .multipart(form)
        .send()
        .map_err(|e| e.to_string())?;

    let status = response.status();
    if !status.is_success() {
        return Err(format!("Server returned status code: {}", status));
    } else {
        let body = response.text().map_err(|e| e.to_string())?;
        Ok(body)
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
