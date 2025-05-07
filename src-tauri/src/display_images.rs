
use rand::Rng;

// SDL2というC/C++のライブラリをRustから使うためのラッパーをしようして作成する
use sdl2::video::WindowContext;
use sdl2::pixels::Color;
use sdl2::sys;
use sdl2::render::{WindowCanvas, TextureCreator};


use crate::config::sdl::SdlContext;


pub fn create_image_window(sdl_context: & SdlContext) -> Result<(WindowCanvas, TextureCreator<WindowContext>), String> {
    let video_subsystem = &sdl_context.video_subsystem;
    // let sdl = &sdl_context._context;

    /*
     * 画像を読み込んで寸法を取得するように変える
     */
    let image_width:u32 = 200;
    let image_height:u32 = 200;
    
    // ボーダーレスウィンドウを作成
    let mut window = video_subsystem.window("Floating Image", image_width, image_height)
        .position_centered()
        .borderless() // ウィンドウの装飾を除去
        .always_on_top() // 常に最前面表示
        .build()
        .map_err(|e| e.to_string())?;

    unsafe {
        sys::SDL_SetWindowOpacity(window.raw(), 0.5);
    }
    
    // 画像のサイズが収ま流範囲でランダムな画面位置にウィンドウを配置
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
    
    let texture_creator = canvas.texture_creator();


    // 画像を読み込むだけならテクスチャとして読み込む
    // let texture = texture_creator.load_texture(path)?;
    
    // 背景を透明に設定
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));

    // テクスチャ自体を返したかったがライフタイムの関係でtexture_creatorを返す
    Ok((canvas, texture_creator) )
}


pub fn process_sdl_events(sdl_context: &SdlContext) -> Result<bool, String> {
    let sdl = &sdl_context._context;
    let mut event_pump = sdl.event_pump()?;
    
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit {..} => return Ok(true), // 終了イベントの場合はtrueを返す
            _ => {},
        }
    }
    
    Ok(false) // 特に問題がなければfalseを返す
}


