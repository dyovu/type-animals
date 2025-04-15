use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;

use dirs;

use crate::config::app_paths;


pub fn get_directory(){
    
    if let Some(data_dir) = dirs::data_dir() {
        let app_dir = data_dir.join("type-animal");
        fs::create_dir(&app_dir).unwrap_or_else(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("すでにディレクトリが存在しています");
            } else {
                panic!("ディレクトリの作成に失敗しました: {}", e);
            }
        });

        let json_path:PathBuf = app_dir.join(app_paths::JSON_FILE);
        let image_dir:PathBuf = app_dir.join(app_paths::IMAGE_DIR);
        let mut image_path = image_dir.clone();  // image_dir: PathBuf
        image_path.push("cat_1.jpeg");

        match fs::exists(&json_path){
            Ok(true) =>{
                println!("すでにファイルが存在しています");
            }
            Ok(false) =>{ // JSONファイルをアプリケーション独自のパスに作成、同じ階層に画像ファイルを保存するディレクトリも作成
                println!("ファイルが存在していません、新規作成します");

                let json_str:&str = include_str!("../default_data/sample.json");
                let mut data: HashMap<String, String> = serde_json::from_str(json_str).expect("JSONのパースに失敗しました");
                if let Some(entry) = data.get_mut("cat") {
                    *entry = image_path.to_str().expect("文字列の変換に失敗しました").to_string();  
                }
                let json_str = serde_json::to_string_pretty(&data).expect("JSONのシリアライズに失敗しました");
                fs::write(json_path, json_str).expect("JSONファイルの書き込みに失敗しました");
                println!("デフォルトデータは{:?}", data);


                // 画像ファイルを保存するディレクトリを作成
                let image_bytes = include_bytes!("../default_data/cat_1.jpeg");
                match fs::exists(&image_dir) {
                    Ok(true) => {
                        println!("画像保存用のディレクトリはすでに存在しています");
                    }
                    Ok(false) => {
                        fs::create_dir(&image_dir).expect("画像保存用のディレクトリの作成に失敗しました");
                        fs::write(image_path, image_bytes).expect("画像ファイルの書き込みに失敗しました");
                        println!("画像保存用のディレクトリを作成しました");
                    }
                    Err(_) => {
                        println!("権限不足によりディレクトリの確認に失敗しました");
                    }
                }
            }
            Err(_) => {
                println!("ファイルの存在確認に失敗しました");
            }
        }
        app_paths::initialize_app_data_path(app_dir);
    }

}




