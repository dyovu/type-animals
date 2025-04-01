use std::sync::{Arc, Mutex};

mod listen_keybord;
use listen_keybord::AppState;
mod get_directories;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_|{
            let _ = get_directories::get_directory();
            Ok(())
        })
        .manage(AppState { // AppStateを手動で初期化する必要がある、そのためArcやMutexを使っている
            listener_process: Arc::new(Mutex::new(None)),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![listen_keybord::start_listening, listen_keybord::stop_listening])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
