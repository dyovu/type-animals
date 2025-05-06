mod tmp{
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



    pub struct GifWindow<'a> {
        window: Window,
        canvas: sdl2::render::Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
        texture: Texture<'a>,
        created_at: Instant,
        duration: Duration,
    }
    
    impl<'a> GifWindow<'a> {
        pub fn new(
            video_subsystem: &sdl2::VideoSubsystem,
            path: String,
            duration_secs: u64,
        ) -> Result<Self, String> {
            // ランダムな位置を設定
            let mut rng = rand::thread_rng();
            let screen_width = 1920; // 画面サイズは適宜調整
            let screen_height = 1080;

            // ウィンドウサイズと位置の設定
            let x = rng.gen_range(0..screen_width - query.width);
            let y = rng.gen_range(0..screen_height - query.height);
            

            // まずウィンドウを作成
            let mut window = video_subsystem
                .window("gif", 0, 0) // サイズは後で設定
                .position_centered()
                .borderless()
                .always_on_top()
                .build()
                .map_err(|e| e.to_string())?;
                
            // canvasを作成
            let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
            
            // texture_creatorを取得
            let texture_creator = canvas.texture_creator();
            
            // テクスチャの読み込み
            let texture = texture_creator.load_texture(path)?;
            let query = texture.query();
            
            
            
            let mut window = canvas.window_mut();
            window.set_size(query.width, query.height)?;
            window.set_position(
                WindowPos::Positioned(x as i32), 
                WindowPos::Positioned(y as i32)
            );
            
            // 透過設定
            canvas.set_blend_mode(BlendMode::Blend);
            
            Ok(GifWindow {
                window: canvas.window_mut().clone(),
                canvas,
                texture_creator: unsafe {
                    // 安全ではありませんが、TextureCreatorの参照をウィンドウのライフタイムに拡張
                    std::mem::transmute(&texture_creator)
                },
                texture,
                created_at: Instant::now(),
                duration: Duration::from_secs(duration_secs),
            })
        }
        
        fn render(&mut self) -> Result<(), String> {
            self.canvas.clear();
            self.canvas.copy(&self.texture, None, None)?;
            self.canvas.present();
            Ok(())
        }
    
        fn is_expired(&self) -> bool {
            Instant::now().duration_since(self.created_at) > self.duration
        }
    }
    
    // GifManagerはダミーウィンドウを持たず、各GifWindowが独自のtexture_creatorを管理
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
    
        pub fn add_gif(&mut self, path: String, duration_secs: u64) -> Result<u32, String> {
            let gif_window = GifWindow::new(
                &self.video_subsystem,
                path,
                duration_secs,
            )?;
            
            let id = self.next_id;
            self.next_id += 1;
            self.gif_windows.insert(id, gif_window);
            
            Ok(id)
        }
        
        pub fn update(&mut self) {
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