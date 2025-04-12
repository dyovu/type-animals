use std::path::{PathBuf};
use std::process::{Child};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;


use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};


pub struct AppState {
    pub listener_process: Arc<Mutex<Option<Child>>>, // マウス位置取得のバイナリクレートのプロセス
}


pub mod app_paths{
    use super::*; // app_pathsモジュールの外側でArc, Mutex, Lazy等をuseしているから、このモジュール内にimprtする必要がある

    pub static APP_DATA_PATH: Lazy<Arc<Mutex<Option<PathBuf>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
    pub static FILE_NAME:&str = "list.json";
    pub static IMAGE_DIR:&str = "images";
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub spell: String,
    pub path: String,
}


