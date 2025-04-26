// SDL2というC/C++のライブラリをRustから使うためのラッパーをしようして作成する

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
use std::time::Duration;
use sdl2::sys;

fn display_images(path: &str) -> Result<(), String> {
    // sdlライブラリを使うには必ずsdl2::init()で初期化する必要がある。
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // sdl_imageの初期化
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    
    // 画像を読み込んで寸法を取得する（実際のコードではこの部分の処理が必要）
    let image_width:u32 = 200;
    let image_height:u32 = 200;
    
    // ボーダーレスウィンドウを作成
    let mut window = video_subsystem.window("Floating Image", image_width, image_height)
        .position_centered()
        .borderless() // ウィンドウの装飾を除去
        .always_on_top() // 常に最前面表示
        .build()
        .map_err(|e| e.to_string())?;

    // ウィンドウ作成後に設定
    unsafe {
        sys::SDL_SetWindowOpacity(window.raw(), 0.5);
    }
    
    // ランダムな位置に配置
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let display_bounds = video_subsystem.display_bounds(0)?;
    let random_x = rng.gen_range(0..display_bounds.width() - image_width);
    let random_y = rng.gen_range(0..display_bounds.height() - image_height);
    window.set_position(
        sdl2::video::WindowPos::Positioned(random_x as i32),
        sdl2::video::WindowPos::Positioned(random_y as i32)
    );
    
    
    
    // レンダラーの設定
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    
    // テクスチャの作成と画像の読み込み
    let texture_creator = canvas.texture_creator();
    

    // SDLのイメージ拡張を使用
    // let image: = sdl2::image::ImageRWops::from_file(Path::new(path))?.load()?;
    // let texture = texture_creator.create_texture_from_surface(&image)
    //     .map_err(|e| e.to_string())?;

    let texture = texture_creator.load_texture(Path::new(path))?;
    
    // 背景を透明に設定
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
    
    // イベントループ
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        
        // 透明な背景でクリア
        canvas.clear();
        
        // 画像を描画
        canvas.copy(&texture, None, None)?;
        
        // 画面を更新
        canvas.present();
        
        // 少し待つ
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}