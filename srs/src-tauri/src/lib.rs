#[tauri::command]
fn play_audio(mp3_array: Vec<u8>) -> Result<(), String> {
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![play_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
