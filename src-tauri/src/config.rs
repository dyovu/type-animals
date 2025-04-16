use std::path::PathBuf;
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};

use once_cell::sync::Lazy;

// マウス位置取得のバイナリクレートのプロセス
pub struct AppState {
    pub listener_process: Arc<Mutex<Option<Child>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            listener_process: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_listener_process(&self) -> &Arc<Mutex<Option<Child>>> {
        &self.listener_process
    }
}

pub static APP_STATE: Lazy<AppState> = Lazy::new(|| AppState::new());


// アプリケーション全体のパスと、JSONのファイル名、画像ディレクトリ名
// imiagesディレクトリ内のファイル名についてはJSONファイルのpathを参照する
pub mod app_paths {
    use super::*;

    pub const JSON_FILE: &str = "list.json";  
    pub const IMAGE_DIR: &str = "images";
    pub static APP_DATA_PATH: Lazy<Arc<Mutex<Option<PathBuf>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

    pub fn initialize_app_data_path(app_dir: PathBuf) {
        let mut path = APP_DATA_PATH.lock().unwrap();
        *path = Some(app_dir);
    }

    pub fn get_json_path() -> PathBuf {
        let path = APP_DATA_PATH.lock().unwrap();
        path.as_ref().expect("アプリケーションのパスが初期化されていません").join(JSON_FILE)
    }
}


// start_listeningが呼ばれたらカウントをリセットして、initialize_key_countを呼び出す
pub mod key_counts{
    use super::*;

    static KEY_COUNT: Lazy<Arc<Mutex<HashMap<String, VecDeque<char>>>>> = Lazy::new(||Arc::new(Mutex::new(HashMap::new())));

    pub fn initialize_key_count() {
        let json    = json_data::JSON_DATA.lock().unwrap();
        let mut data = KEY_COUNT.lock().unwrap();

        for (spell, _) in json.iter() {
            let mut vec = VecDeque::new();
            for c in spell.chars() {
                vec.push_back(c);
            }
            data.insert(spell.clone(), vec);
        }
        println!("initialized key_count: {:?}", data);
    }

    pub fn reset_key_count() {
        let mut data = KEY_COUNT.lock().unwrap();
        data.clear();
        println!("key_count reset");
    }

    pub fn get_keycount() -> HashMap<String, VecDeque<char>> {
        let data = KEY_COUNT.lock().unwrap();
        data.clone()
    }

    pub fn pop_and_refill(key:&str) -> bool{
        let mut data = KEY_COUNT.lock().unwrap();
        let dq = data.entry(key.to_string()).or_insert_with(VecDeque::new);
        dq.pop_front();

        if dq.is_empty() {
            dq.extend(key.chars());
            println!("全てのキーが入力されました: {}", key);
            return true;
        }
        return false;
    }
}



pub mod json_data{
    use super::*;

    // アプリケーションのフォルダに保存しているjsonファイルのデータを格納する
    pub static JSON_DATA: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(||Arc::new(Mutex::new(HashMap::new())));

    // jsonファイル作成時(アプリケーション起動時)と値を変更、追加した際に呼び出す
    pub fn initialize_json_data() {
        let json_path: PathBuf = app_paths::get_json_path();

        let mut data = JSON_DATA.lock().unwrap();
        let json_str = std::fs::read_to_string(json_path).expect("JSONファイルの読み込みに失敗しました");
        *data = serde_json::from_str(&json_str).expect("JSONのパースに失敗しました");

        println!("JSONデータの初期化完了: {:?}", data);
    }

    pub fn get_json_data() -> HashMap<String, String> {
        let data = JSON_DATA.lock().unwrap();
        data.clone()
    }
}




