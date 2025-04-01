use std::thread;
use std::path::PathBuf;
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
    // アプリケーションのパス
    if let Some(data_dir) = dirs::data_dir() {
        // println!("App data directory: {:?}", data_dir);
        let mut path = app_paths::APP_DATA_PATH.lock().unwrap();
        *path = Some(data_dir);
        println!("APP_DATA_PATH is: {:?}", path);
    }

    // ダウンロードディレクトリ
    if let Some(downloads_dir) = dirs::download_dir() {
        // println!("Download directory: {:?}", downloads_dir);
        let mut path = app_paths::DOWNLOADS_PATH.lock().unwrap();
        *path = Some(downloads_dir);
        println!("DOWNLOADS_PATH is: {:?}", path);
    }
}




