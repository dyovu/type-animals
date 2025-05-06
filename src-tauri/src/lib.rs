use tauri::tray::TrayIconBuilder;


mod listen_keybord;
mod get_directories;
mod config;
mod count_keys;
// mod display_images;
mod process_key_events;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app|{
            get_directories::get_directory();
            config::json_data::initialize_json_data();
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;
            Ok(())
        })
        // .manage()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![listen_keybord::start_process, listen_keybord::stop_listening])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
