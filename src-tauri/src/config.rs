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
    
    pub struct Textures<'a>{
        textures: HashMap<u32, Texture<'a>>,
    }

    impl<'a> Textures<'a>{
        pub fn new() -> Self {
            Textures {
                textures: HashMap::new(),
            }
        }

        pub fn load_texture(&mut self, next_id: u32, texture_creator: &'a TextureCreator<WindowContext>, path: String) -> Result<(), String> {
            let texture = texture_creator.load_texture(path).unwrap();
            self.textures.insert(next_id-1, texture);
            Ok(())
        }

        pub fn add_loaded_texture(&mut self, id: u32, texture: Texture<'a>) -> Result<(), String> {
            self.textures.insert(id, texture);
            Ok(())
        }
    }
    

    pub struct GifWindow {
        window: Window,
        canvas: sdl2::render::Canvas<Window>,
        texture_creator: TextureCreator<WindowContext>,
        created_at: Instant,
        duration: Duration,
    }
    
    impl GifWindow {
        pub fn new(
            video_subsystem: &sdl2::VideoSubsystem,
            duration_secs: u64,
        ) -> Result<Self, String> {
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
            
            Ok(GifWindow {
                window: canvas.window_mut().clone(),
                canvas,
                texture_creator,
                created_at: Instant::now(),
                duration: Duration::from_secs(duration_secs),
            })
        }
        
        fn render(&mut self, texture: &Texture) -> Result<(), String> {
            self.canvas.clear();
            self.canvas.copy(texture, None, None)?;
            self.canvas.present();
            Ok(())
        }
    
        fn is_expired(&self) -> bool {
            Instant::now().duration_since(self.created_at) > self.duration
        }
    }
    
    // GifManagerはダミーウィンドウを持たず、各GifWindowが独自のtexture_creatorを管理
    pub struct GifManager {
        video_subsystem: sdl2::VideoSubsystem,
        gif_windows: HashMap<u32, GifWindow>,
        next_id: u32,
        // _phantom: std::marker::PhantomData<&'a ()>,
    }
    
    impl GifManager {
        pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
            let video_subsystem = sdl_context.video()?;
            
            Ok(GifManager {
                video_subsystem,
                gif_windows: HashMap::new(),
                next_id: 0,
            })
        }
    
        pub fn add_gif(&mut self, duration_secs: u64) -> Result<u32, String> {
            let gif_window = GifWindow::new(
                &self.video_subsystem,
                duration_secs,
            )?;
            
            let id = self.next_id;
            self.next_id += 1;
            self.gif_windows.insert(id, gif_window);
            Ok(id)
        }
        
        pub fn get_texture_creator(& self) -> (u32, &TextureCreator<WindowContext>) {
            let id = self.next_id - 1;
            let window: & GifWindow = self.gif_windows.get(&id).unwrap();
            let texture_creator: &TextureCreator<WindowContext>  = &window.texture_creator;
            (id, &texture_creator)
        }



        pub fn get_texture_info(&self) -> (u32, /* 必要な情報 */) {
            (self.next_id - 1, /* 必要な情報 */)
        }
        
        // TextureCreatorへのアクセスを提供するメソッド
        pub fn with_texture_creator<F, R>(&self, id: u32, f: F) -> Result<R, String>
        where
            F: FnOnce(&TextureCreator<WindowContext>) -> R,
        {
            if let Some(window) = self.gif_windows.get(&id) {
                Ok(f(&window.texture_creator))
            } else {
                Err("Window not found".to_string())
            }
        }




        // 最新のものだけ変更巣売るように変えたほうがいいかも
        pub fn update_window_position(&mut self, textures: &Textures) -> Result<(), String> {
            for (id, window) in &mut self.gif_windows {
                let texture = textures.textures.get(id).unwrap();
                let query = texture.query();
                
                // ランダムな位置を設定
                let mut rng = rand::thread_rng();
                let screen_width = 1920; // 画面サイズは適宜調整
                let screen_height = 1080;
                
                let x = rng.gen_range(0..screen_width - query.width);
                let y = rng.gen_range(0..screen_height - query.height);
                
                window.window.set_position(
                    WindowPos::Positioned(x as i32), 
                    WindowPos::Positioned(y as i32)
                );
            }
            Ok(())
        }
        
        pub fn update(&mut self, textures: &mut Textures) {
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
                textures.textures.remove(&id);
            }
            
            // 残りのGIFをレンダリング
            for (_, window) in &mut self.gif_windows {
                let texture = textures.textures.get(&(self.next_id - 1)).unwrap();
                if let Err(e) = window.render(texture) {
                    eprintln!("Error rendering GIF: {}", e);
                }
            }
        }
        
        pub fn is_empty(&self) -> bool {
            self.gif_windows.is_empty()
        }
    }
}









/*
======================================================
======================================================
======================================================

*/



// pub mod sdl_v2{
//     use super::*;
//     use rand::Rng;
//     use std::time::Instant;
//     use std::time::Duration;
//     use std::sync::mpsc::Sender;

//     use sdl2::Sdl;
//     use sdl2::video::{WindowContext, Window,WindowPos};
//     use sdl2::image::{InitFlag, Sdl2ImageContext, LoadTexture};
//     use sdl2::render::{Texture, TextureCreator, BlendMode};
//     use sdl2::event::Event;
//     use sdl2::keyboard::Keycode;



//     pub struct SdlContext {
//         pub _context: Sdl,
//         pub _image_context: Sdl2ImageContext,
//         pub event_pump: sdl2::EventPump,
//     }
    
//     impl SdlContext {
//         pub fn init() -> Result<Self, String> {
//             let sdl_context = sdl2::init()?;
//             let image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
//             let event_pump = sdl_context.event_pump()?;
            
//             Ok(SdlContext {
//                 _context: sdl_context,
//                 _image_context: image_context,
//                 event_pump,
//             })
//         }

//         pub fn event_pump(&mut self, sender: Sender<Message>) -> bool {
//             for event in self.event_pump.poll_iter() {
//                 match event {
//                     Event::Quit { .. } => {
//                         sender.send(Message::Quit).unwrap();
//                         return false;
//                     },
//                     Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                         sender.send(Message::Quit).unwrap();
//                         return false;
//                     },
//                     _ => {}
//                 }
//             }
//             return true
//         }
//     }



//     pub enum Message {
//         DisplayGif { path: String, duration: u64 },
//         Quit,
//     }

//     pub struct GifWindow<'a> {
//         window: Window,
//         canvas: sdl2::render::Canvas<Window>,
//         texture_creator: &'a TextureCreator<WindowContext>,
//         texture: Texture<'a>,
//         created_at: Instant,
//         duration: Duration,
//     }
    
//     impl<'a> GifWindow<'a> {
//         pub fn new(
//             video_subsystem: &sdl2::VideoSubsystem,
//             texture_creator: &'a TextureCreator<WindowContext>,
//             path: String,
//             duration_secs: u64,
//         ) -> Result<Self, String> {
//             // テクスチャの読み込み
//             let texture = texture_creator.load_texture(path)?;
//             let query = texture.query();
            
//             // ランダムな位置を設定
//             let mut rng = rand::thread_rng();
//             let screen_width = 1920; // 画面サイズは適宜調整
//             let screen_height = 1080;
            
//             let x = rng.gen_range(0..screen_width - query.width);
//             let y = rng.gen_range(0..screen_height - query.height);
            
//             // 透過ウィンドウの作成
//             let mut window = video_subsystem
//                 .window("gif", query.width, query.height)
//                 .position_centered()
//                 .borderless()
//                 .always_on_top()
//                 .build()
//                 .map_err(|e| e.to_string())?;


//             window.set_position(
//                 WindowPos::Positioned(x as i32), 
//                 WindowPos::Positioned(y as i32)
//             );
            
//             let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
            
//             // 透過設定
//             canvas.set_blend_mode(BlendMode::Blend);
            
//             Ok(GifWindow {
//                 window: canvas.window_mut().clone(),
//                 canvas,
//                 texture,
//                 created_at: Instant::now(),
//                 duration: Duration::from_secs(duration_secs),
//             })
//         }
        
//         fn render(&mut self) -> Result<(), String> {
//             self.canvas.clear();
//             self.canvas.copy(&self.texture, None, None)?;
//             self.canvas.present();
//             Ok(())
//         }

//         fn is_expired(&self) -> bool {
//             Instant::now().duration_since(self.created_at) > self.duration
//         }
//     }


//     // ダミーのウィンドウを作成して、GIFを表示するウィンドウを管理する
//     pub struct GifManager<'a> {
//         video_subsystem: sdl2::VideoSubsystem,
//         texture_creator: Arc<TextureCreator<WindowContext>>,
//         gif_windows: HashMap<u32, GifWindow<'a>>,
//         next_id: u32,
//     }
    
//     impl<'a> GifManager<'a> {
//         pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String> {
//             let video_subsystem = sdl_context.video()?;
            
//             // ダミーウィンドウを作成してテクスチャクリエーターを取得
//             let dummy_window = video_subsystem.window("dummy", 1, 1)
//                 .hidden()
//                 .build()
//                 .map_err(|e| e.to_string())?;
            
//             let canvas = dummy_window.into_canvas().build().map_err(|e| e.to_string())?;
//             let texture_creator = canvas.texture_creator();
//             let texture_creator = Arc::new(texture_creator);
            
//             Ok(GifManager {
//                 video_subsystem,
//                 texture_creator,
//                 gif_windows: HashMap::new(),
//                 next_id: 0,
//             })
//         }

//         // 
//         pub fn add_gif(&mut self, path: String, duration_secs: u64) -> Result<u32, String> {

//             // let texture_creator_ref: &'a TextureCreator<WindowContext> = unsafe {
//             //     // 安全ではありませんが、Arcで包まれたTextureCreatorの参照を'aライフタイムにキャスト
//             //     std::mem::transmute::<&TextureCreator<WindowContext>, &'a TextureCreator<WindowContext>>(
//             //         &self.texture_creator
//             //     )
//             // };

//             let gif_window = GifWindow::new(
//                 &self.video_subsystem,
//                 &self.texture_creator,
//                 // texture_creator_ref,
//                 path,
//                 duration_secs,
//             )?;
            
//             let id = self.next_id;
//             self.next_id += 1;
//             self.gif_windows.insert(id, gif_window);
            
//             Ok(id)
//         }
        
//         pub fn update(&mut self) {
//             let mut expired_ids = Vec::new();
            
//             // 期限切れのGIFを特定
//             for (id, window) in &self.gif_windows {
//                 if window.is_expired() {
//                     expired_ids.push(*id);
//                 }
//             }
            
//             // 期限切れのGIFを削除
//             for id in expired_ids {
//                 self.gif_windows.remove(&id);
//             }
            
//             // 残りのGIFをレンダリング
//             for (_, window) in &mut self.gif_windows {
//                 if let Err(e) = window.render() {
//                     eprintln!("Error rendering GIF: {}", e);
//                 }
//             }
//         }
        
//         pub fn is_empty(&self) -> bool {
//             self.gif_windows.is_empty()
//         }
//     }
// }