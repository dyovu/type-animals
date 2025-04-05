use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

use dirs;

use crate::config::{app_paths, Entry};


pub fn get_directory(){
    // アプリケーション独自のパスに"animal_list.json"があるか確認、なければ作成

    
    if let Some(data_dir) = dirs::data_dir() {
        let mut path = app_paths::APP_DATA_PATH.lock().unwrap();
        let full_path:PathBuf = data_dir.join(app_paths::FILE_NAME);

        match fs::exists(&full_path){
            Ok(true) =>{
                println!("すでにファイルが存在しています");
            }
            Ok(false) =>{ // ファイルが存在していない、新規作成する
                println!("ファイルが存在していません、新規作成します");
                let json_str:&str = include_str!("../default_data/sample.json");
                // let json_str = fs::read_to_string("default_data/sample.json").expect("サンプルファイルの読み込みに失敗しました");
                let mut data: HashMap<String, Entry> = serde_json::from_str(json_str).expect("JSONのパースに失敗しました");
            }
            Err(_) => {
                println!("ファイルの存在確認に失敗しました");
            }
        }

        *path = Some(full_path);
        println!("APP_DATA_PATH is: {:?}", path);
    }

}




