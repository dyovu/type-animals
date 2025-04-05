use std::thread;
use std::path::{self, PathBuf};
use std::fs;
use std::sync::{Arc, Mutex};

use dirs;
use once_cell::sync::Lazy;


pub mod app_paths{
    use super::*; // app_pathsモジュールの外側でArc, Mutex, Lazy等をuseしているから、このモジュール内にimprtする必要がある

    pub static DOWNLOADS_PATH: Lazy<Arc<Mutex<Option<PathBuf>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
    pub static APP_DATA_PATH: Lazy<Arc<Mutex<Option<PathBuf>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
}


pub fn get_directory(){
    // アプリケーション独自のパスに"animal_list.json"があるか確認、なければ作成
    // 名前と動画のリストを保存する
    let file_name:&str = "animal_list.json";
    if let Some(data_dir) = dirs::data_dir() {
        let mut path = app_paths::APP_DATA_PATH.lock().unwrap();
        let full_path:PathBuf = data_dir.join(file_name);
        *path = Some(full_path);
        println!("APP_DATA_PATH is: {:?}", path);
    }

    // ダウンロードディレクトリ
    if let Some(downloads_dir) = dirs::download_dir() {
        let mut path = app_paths::DOWNLOADS_PATH.lock().unwrap();
        *path = Some(downloads_dir);
        println!("DOWNLOADS_PATH is: {:?}", path);
    }
}




