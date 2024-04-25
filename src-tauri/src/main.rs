// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn long_task(number: i32) -> Result<i32, String> {
    // Simulate a long task. If the browser reloads while this is running,
    // then its response will go to the "wrong" browser
    // instance/webview/not sure what to call it and cause a crash.
    std::thread::sleep(std::time::Duration::from_secs(20));
    return Ok(number);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![long_task])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
