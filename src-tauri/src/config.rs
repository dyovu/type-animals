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





pub mod sdl{
    use super::*;
    use rand::Rng;
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
        DisplayGif { path: String, duration: u64 },
        Quit,
    }

    
    pub struct TextureCreators{
        texture_creator: HashMap<u32, TextureCreator<WindowContext>>,
    }

    impl TextureCreators{
        pub fn new() -> Self {
            TextureCreators {
                texture_creator: HashMap::new(),
            }
        }

        pub fn add_texture_creator(&mut self, id: u32, new_tx: TextureCreator<WindowContext>) {
            self.texture_creator.insert(id, new_tx);
        }

        pub fn load_texture(& self, id: u32, path: String) -> Result<Texture, String> {
            let texture = self.texture_creator.get(&(id))
                .unwrap()
                .load_texture(path)
                .map_err(|e| e.to_string())?;
            Ok(texture)
        }
    }
    

    pub struct GifWindow <'a>{
        window: Window,
        canvas: sdl2::render::Canvas<Window>,
        texture: Option<Texture<'a>>,
        created_at: Instant,
        duration: Duration,
    }
    
    impl<'a> GifWindow<'a>{
        pub fn new(
            video_subsystem: &sdl2::VideoSubsystem,
            duration_secs: u64,
        ) -> Result<(Self,TextureCreator<WindowContext>) , String> {
            // まずウィンドウを作成
            let window = video_subsystem
                .window("gif", 0, 0) 
                .position_centered()
                .borderless()
                .always_on_top()
                .build()
                .map_err(|e| e.to_string())?;
                
            // canvasを作成
            let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
            
            // texture_creatorを取得
            let texture_creator = canvas.texture_creator();
            
            
            // 透過設定
            canvas.set_blend_mode(BlendMode::Blend);
            
            Ok((GifWindow {
                window: canvas.window_mut().clone(),
                canvas,
                texture: None,
                created_at: Instant::now(),
                duration: Duration::from_secs(duration_secs),
            }, texture_creator))
        }
        
        fn render(&mut self) -> Result<(), String> {
            // let texture: Texture = self.texture.take().expect("Texture not set");
            self.canvas.clear();
            if let Some(ref texture) = self.texture {
                self.canvas.copy(texture, None, None)?;
            } else {
                return Err("Texture not set".to_string());
            }
            self.canvas.present();
            Ok(())
        }
    
        fn is_expired(&self) -> bool {
            Instant::now().duration_since(self.created_at) > self.duration
        }
    }
    

    pub struct GifManager<'a> {
        video_subsystem: sdl2::VideoSubsystem,
        gif_windows: HashMap<u32, GifWindow<'a>>,
        next_id: u32,
    }
    
    impl<'a> GifManager<'a> {
        pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
            let video_subsystem = sdl_context.video()?;
            
            Ok(GifManager {
                video_subsystem,
                gif_windows: HashMap::new(),
                next_id: 0,
            })
        }
    
        pub fn add_gif(&mut self, duration_secs: u64, texture_creators: &mut TextureCreators) -> Result<u32, String> {
            let (gif_window, new_tx) = GifWindow::new(
                &self.video_subsystem,
                duration_secs,
            )?;
            
            let id = self.next_id;
            self.next_id += 1;
            self.gif_windows.insert(id, gif_window);
            texture_creators.add_texture_creator(id, new_tx);
            Ok(id)
        }


        // 最新のものだけ変更巣売るように変えたほうがいいかも
        pub fn update_window_position(&mut self, id: u32, texture: Texture<'a>) -> Result<(), String> {
            let gif_window = &mut self.gif_windows.get_mut(&id).unwrap();
            gif_window.texture = Some(texture);
            let texture_now = gif_window.texture.as_ref().unwrap(); 
            let query = texture_now.query();
            
            // ランダムな位置を設定
            let mut rng = rand::thread_rng();
            let screen_width = 1920; // 画面サイズは適宜調整
            let screen_height = 1080;
            
            let x = rng.gen_range(0..screen_width - query.width);
            let y = rng.gen_range(0..screen_height - query.height);
            
            gif_window.window.set_position(
                WindowPos::Positioned(x as i32), 
                WindowPos::Positioned(y as i32)
            );

            Ok(())
        }
        
        pub fn update(&mut self, texture_creators: &mut TextureCreators) {
            let mut expired_ids = Vec::new();
            
            // 期限切れのGIFを特定
            for (id, window) in &self.gif_windows {
                if window.is_expired() {
                    expired_ids.push(*id);
                }
            }
            
            // 期限切れのGIFを削除
            for id in expired_ids {
                self.gif_windows.remove(&id);
                texture_creators.texture_creator.remove(&id);
            }
            
            // 残りのGIFをレンダリング
            for (_, window) in &mut self.gif_windows {
                if let Err(e) = window.render() {
                    eprintln!("Error rendering GIF: {}", e);
                }
            }
        }
        
        pub fn is_empty(&self) -> bool {
            self.gif_windows.is_empty()
        }
    }
}
