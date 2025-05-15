use std::io::{BufRead, BufReader, Read, ErrorKind};
use std::process::{Command, Stdio};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::config::APP_STATE;
use crate::config::key_counts::{initialize_key_count, reset_key_count};
use crate::config::sdl::{SdlContext, Message, ImageManager};
use crate::process_key_events::listening_key;

// 停止リクエストフラグ - 静的なグローバル変数として定義
static STOP_REQUESTED: AtomicBool = AtomicBool::new(false);

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

    // 停止フラグをリセット
    STOP_REQUESTED.store(false, Ordering::SeqCst);

    // key_countをリセット
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
    
    // APP_STATEにプロセスを保存
    {
        let mut state = APP_STATE.get_listener_process().lock().expect("Failed to lock mutex");
        *state = Some(listener_process);
    }
    
    

    // SDL2の初期化
    let mut sdl_context:SdlContext = SdlContext::init().expect("Failed to initialize SDL2");
    let mut image_manager: ImageManager = match ImageManager::new(&sdl_context._context) {
        Ok(image_manager) => image_manager,
        Err(e) => {
            eprintln!("Failed to create imageManager: {}", e);
            return;
        }
    };

    'running: loop {
        // 停止リクエストのチェック - 各ループの先頭で確認
        if STOP_REQUESTED.load(Ordering::SeqCst) {
            println!("Stop requested, breaking loop");
            break 'running;
        }

        if !sdl_context.event_pump(sender.clone()) {
            break 'running;
        }

        if let Ok(message) = receiver.try_recv() {
            match message {
                Message::DisplayImage {path, duration } => {
                    let id :u32 = match image_manager.add_image(duration, path) {
                        Ok(id) => id,
                        Err(e) => {
                            eprintln!("Failed to add image: {}", e);
                            continue
                        }
                    };
                    if let Err(e) = image_manager.render_images(&id){
                        eprintln!("Failed to update window position: {}", e);
                        continue
                    }
                },
                Message::Quit => break 'running,
            }
        }

        image_manager.update();

        // ポーリング
        // メッセージの受信と、停止リクエストが来ていないか確認する
        thread::sleep(Duration::from_millis(10));
    }

    
    // ループを抜けた後のクリーンアップ
    // stop_listening()が既に呼ばれている可能性があるため、ここでもプロセスを終了させる
    {
        let mut state = APP_STATE.get_listener_process().lock().expect("Failed to lock mutex");
        if let Some(mut process) = state.take() {
            let _ = process.kill();
        }
    }
    
    // 停止フラグをリセット
    STOP_REQUESTED.store(false, Ordering::SeqCst);
    
    println!("process finished successfully");
}

#[tauri::command]
pub fn stop_listening() {
    // 停止リクエストフラグを設定
    STOP_REQUESTED.store(true, Ordering::SeqCst);
    
    // 既存のプロセス終了コード
    let listener_process = APP_STATE.get_listener_process().lock().expect("Failed to lock mutex").take();
    reset_key_count();
    
    if let Some(mut listener_process) = listener_process {
        listener_process.kill().expect("failed to kill listener process");
        println!("process killed successfully");
    } else {
        println!("プロセスが起動していません");
        return;
    }
}