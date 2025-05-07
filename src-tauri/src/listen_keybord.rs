use std::io::{BufRead, BufReader, Read, ErrorKind};
use std::process::{Command, Stdio};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;


use crate::config::APP_STATE;
use crate::config:: key_counts::{initialize_key_count, reset_key_count};
use crate::config::sdl::{SdlContext, Message, GifManager, TextureCreators,};
use crate::process_key_events::listening_key;



#[tauri::command]
pub fn start_process() {
    // /bin/にキーボードの位置取得のバイナリクレートを入れる
    let listen_process_path: &str = "libs/listen-keytype";

    // プロセスが起動しているときは何もしない
    {
        let state = APP_STATE.get_listener_process().lock().expect("Failed to lock mutex");
        if state.is_some() {
            println!("listener process is already running");
            return;
        }
    }

    // key_countをウントをリセット
    initialize_key_count();

    // キーボードの取得と画像表示のためのスレッド間通信を行うためのチャネル
    let (sender, receiver): (Sender<Message>, Receiver<Message>) = channel();


    // 新しいプロセスを起動
    let mut listener_process = Command::new(listen_process_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute listener process");

    // エラーが出たらエラーを表示
    if let Some(stderr) = listener_process.stderr.take() {
        let reader = BufReader::new(stderr);
        thread::spawn(move||{
            for line in reader.lines() {
                if let Ok(line) = line {
                    eprintln!("error: {}", line);
                }
            }
        });
    }

    println!("process started successfully");
    // listener_processの標準出力を読み取る
    listening_key(&mut listener_process, sender.clone());

    let mut state = APP_STATE.get_listener_process().lock().expect("Failed to lock mutex");
    *state = Some(listener_process);


    // SDL2の初期化
    let mut sdl_context:SdlContext = SdlContext::init().expect("Failed to initialize SDL2");
    let mut gif_manager: GifManager = match GifManager::new(&sdl_context._context) {
        Ok(gif_manager) => gif_manager,
        Err(e) => {
            eprintln!("Failed to create GifManager: {}", e);
            return;
        }
    };
    let mut texture_creators: TextureCreators = TextureCreators::new();

    'running: loop {
        if !sdl_context.event_pump(sender.clone()) {
            break 'running;
        }

        if let Ok(message) = receiver.try_recv() {
            match message {
                Message::DisplayGif {path, duration } => {
                    let id :u32 = match gif_manager.add_gif(duration, &mut texture_creators) {
                        Ok(id) => id,
                        Err(e) => {
                            eprintln!("Failed to add gif: {}", e);
                            continue
                        }
                    };

                    let texture = texture_creators.load_texture(id.clone(), path).unwrap();


                    if let Err(e) = gif_manager.update_window_position(id.clone(), texture){
                        eprintln!("Failed to update window position: {}", e);
                        continue
                    }

                },
                Message::Quit => break 'running,
            }
        }

        gif_manager.update(&mut texture_creators);

        thread::sleep(Duration::from_millis(20));
    }

    println!("process finished successfully");

}



#[tauri::command]
pub fn stop_listening() {
    let listener_process = APP_STATE.get_listener_process().lock().expect("Failed to lock mutex").take();
    reset_key_count();
    if let Some(mut listener_process) = listener_process {
        listener_process.kill().expect("failed to kill listener process");
        println!("process killed successfully");
    }else{
        println!("プロセスが起動していません");
        return;
    }
}
