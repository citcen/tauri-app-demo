// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}！欢迎使用 Rust！", name)
}


#[tauri::command]
fn show_msg(invoke_message: String) {
    println!("从JS 中调用!，你好 {}", invoke_message);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, show_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
