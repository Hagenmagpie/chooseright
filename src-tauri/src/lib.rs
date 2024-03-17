// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn files()  {
    // 定义 assets/flags/ 目录路径
    let assets_flags_dir = "./assets/flags/";

    // 读取 assets/flags/ 目录下的所有文件
    let mut files = fs::read_dir(assets_flags_dir).unwrap();

    // 遍历所有文件并打印文件名
    for file in files {
        let file_name = file.unwrap().file_name();
        println!("{}", file_name.to_string_lossy());
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
