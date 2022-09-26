#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;


struct Notebook {
    filepath: String,
    filename: String,
    cells: Vec<Cell>,
}

struct MarkdownCell {
}

struct QueryCell {
    code: String,
    output: String,
}


#[derive(Default)]
struct Connection(Mutex<Option<Client>>);
struct Client;
impl Client {}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
