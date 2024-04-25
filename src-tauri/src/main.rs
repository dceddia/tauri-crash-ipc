// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn long_task(number: i32) -> Result<i32, String> {
    tokio::task::spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_secs(20));
        Ok(number)
    })
    .await
    .map_err(|join_error| join_error.to_string())?
    .map_err(|e: String| e)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![long_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
