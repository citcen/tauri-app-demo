// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{thread, time};
use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}！欢迎使用 Rust！", name)
}


#[tauri::command]
fn show_msg(invoke_message: String) {
    println!("从JS 中调用!，你好 {}", invoke_message);
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window) {
    std::thread::spawn(move || {
        loop {
            window.emit("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, show_msg, init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
