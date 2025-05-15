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




pub mod json_data{
    use super::*;

    // アプリケーションのフォルダに保存しているjsonファイルのデータを格納する
    pub static JSON_DATA: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(||Arc::new(Mutex::new(HashMap::new())));

    // jsonファイル作成時(アプリケーション起動時)と値を変更、追加した際に呼び出す
    pub fn initialize_json_data() {
        let mut data = JSON_DATA.lock().unwrap();
        *data = read_json_data();

        println!("JSONデータの初期化完了: {:?}", data);
    }

    fn read_json_data()-> HashMap<String, String> {
        let json_path: PathBuf = app_paths::get_json_path();
        let json_str = std::fs::read_to_string(json_path).expect("JSONファイルの読み込みに失敗しました");
        let data: HashMap<String, String> = serde_json::from_str(&json_str).expect("JSONのパースに失敗しました");
        data
    }

    // 毎回jsonファイルをすべて変更するのはコスト的によくなさそうだから、現在のJSON_DATAと差分をとってそこのみ編集する形がいいかな？
    fn update_json_data(json_data: HashMap<String, String>)  {
        let json_path: PathBuf = app_paths::get_json_path();
        let json_str = serde_json::to_string_pretty(&json_data).expect("JSONのシリアライズに失敗しました");
        std::fs::write(json_path, json_str).expect("JSONファイルの書き込みに失敗しました");
        let mut data = JSON_DATA.lock().unwrap();
        *data = json_data.clone();
        println!("JSONデータの更新完了: {:?}", data);
    }



    /*
        // 以下フロント側から呼び出す関数
    */
    #[tauri::command]
    pub fn fetch_json_data() -> HashMap<String, String> {
        let data: std::sync::MutexGuard<'_, HashMap<String, String>> = JSON_DATA.lock().unwrap();
        data.clone()
    }

    #[tauri::command]
    pub fn post_json_data(json_data: HashMap<String, String>) {
        update_json_data(json_data.clone());
    }
}



// start_processが呼ばれたらカウントをリセットして、initialize_key_countを呼び出す
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




pub mod sdl{
    use super::*;
    use rand::Rng;
    use sdl2::image;
    use std::time::Instant;
    use std::time::Duration;
    use std::sync::mpsc::Sender;

    use sdl2::Sdl;
    use sdl2::video::{WindowContext, Window,WindowPos};
    use sdl2::image::{InitFlag, Sdl2ImageContext, LoadTexture};
    use sdl2::render::{Texture, TextureCreator, BlendMode};
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;


    pub struct SdlContext {
        pub _context: Sdl,
        pub _image_context: Sdl2ImageContext,
        pub event_pump: sdl2::EventPump,
    }
    
    impl SdlContext {
        pub fn init() -> Result<Self, String> {
            let sdl_context = sdl2::init()?;
            let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
            let event_pump = sdl_context.event_pump()?;
            
            Ok(SdlContext {
                _context: sdl_context,
                _image_context: image_context,
                event_pump,
            })
        }

        pub fn event_pump(&mut self, sender: Sender<Message>) -> bool {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        sender.send(Message::Quit).unwrap();
                        return false;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        sender.send(Message::Quit).unwrap();
                        return false;
                    },
                    _ => {}
                }
            }
            return true
        }
    }

    pub enum Message {
        DisplayImage { path: String, duration: u64 },
        Quit,
    }

    

    pub struct ImageWindow {
        window: Window,
        canvas: sdl2::render::Canvas<Window>,
        image_path: String,
        created_at: Instant,
        duration: Duration,
    }
    
    impl ImageWindow{
        pub fn new(
            video_subsystem: &sdl2::VideoSubsystem,
            duration_secs: u64,
            image_path: String,
        ) -> Result<Self , String> {
            // まずウィンドウを作成
            let window = video_subsystem
                .window("image", 0, 0) 
                .position_centered()
                .borderless()
                .always_on_top()
                .build()
                .map_err(|e| e.to_string())?;
                
            // canvasを作成
            let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
            
            
            // 透過設定
            canvas.set_blend_mode(BlendMode::Blend);
            
            Ok(ImageWindow {
                window: canvas.window_mut().clone(),
                canvas,
                image_path: image_path,
                created_at: Instant::now(),
                duration: Duration::from_secs(duration_secs),
            })
        }
        
        fn render(&mut self) -> Result<(), String> {
            let texture_creator = self.canvas.texture_creator();
            let image_path = PathBuf::from(&self.image_path);
            let texture = texture_creator.load_texture(image_path)?;
            
            // テクスチャのサイズを取得
            let query = texture.query();
            
            // ウィンドウのサイズを設定
            self.canvas.window_mut().set_size(query.width, query.height);
            
            // ランダムな位置を設定
            let mut rng = rand::thread_rng();
            let screen_width = 1920;
            let screen_height = 1080;
            let x = rng.gen_range(0..screen_width - query.width);
            let y = rng.gen_range(0..screen_height - query.height);
            
            self.window.set_position(
                WindowPos::Positioned(x as i32),
                WindowPos::Positioned(y as i32)
            );
            
            // 描画
            self.canvas.clear();
            self.canvas.copy(&texture, None, None)?;
            self.canvas.present();
            Ok(())
        }
    
        fn is_expired(&self) -> bool {
            Instant::now().duration_since(self.created_at) > self.duration
        }
    }
    

    pub struct ImageManager {
        video_subsystem: sdl2::VideoSubsystem,
        image_windows: HashMap<u32, ImageWindow>,
        next_id: u32,
    }
    
    impl ImageManager {
        pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
            let video_subsystem = sdl_context.video()?;
            
            Ok(ImageManager {
                video_subsystem,
                image_windows: HashMap::new(),
                next_id: 0,
            })
        }
    
        pub fn add_image(&mut self, duration_secs: u64, image_path: String) -> Result<u32, String> {
            let image_window = ImageWindow::new(
                &self.video_subsystem,
                duration_secs,
                image_path,
            )?;
            
            let id = self.next_id;
            self.next_id += 1;
            self.image_windows.insert(id, image_window);
            Ok(id)
        }

        
        pub fn update(&mut self) {
            let mut expired_ids = Vec::new();
            
            // 期限切れのimageを特定
            for (id, window) in &self.image_windows {
                if window.is_expired() {
                    expired_ids.push(*id);
                }
            }
            
            // 期限切れのimageを削除
            for id in expired_ids {
                self.image_windows.remove(&id);
            }
        }

        pub fn render_images(&mut self) -> Result<(), String> {
            for (_, window) in &mut self.image_windows {
                if let Err(e) = window.render() {
                    eprintln!("Error rendering image: {}", e);
                    return Err(e);
                }
            }
            Ok(())
        }
        
        pub fn is_empty(&self) -> bool {
            self.image_windows.is_empty()
        }
    }
}
