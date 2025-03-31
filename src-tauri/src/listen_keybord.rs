use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio, Child};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct AppState {
    pub listener_process: Arc<Mutex<Option<Child>>>, // マウス位置取得のバイナリクレートのプロセス
}

#[tauri::command]
pub fn start_listening(app_state: tauri::State<AppState>) {
    // /bin/にキーボードの位置取得のバイナリクレートを入れる
    let listen_process_path: &str = "bin/listen-keytype";

    // プロセスが起動しているときは何もしない
    {
        let state = app_state.listener_process.lock().unwrap();
        if state.is_some() {
            println!("listener process is already running");
            return;
        }
    }

    // 新しいプロセスを起動
    let mut listener_process = Command::new(listen_process_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute listener process");
    
    println!("process started successfully");

    if let Some(stdout) = listener_process.stdout.take() { // '=' の返り値がSome()型ならその値をstdoutにいれるということ、
        let reader = BufReader::new(stdout);
        thread::spawn(move||{
            for line in reader.lines() {
                if let Ok(line) = line {
                    println!("pressed key of {}", line);
                    // 押されたキーを取得・保存・表示の処理
                    // ToDo : あとで関数作って処理する
                }
            }
        });
    }

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

    // `listener_process` を AppState に格納
    let mut state = app_state.listener_process.lock().unwrap();
    *state = Some(listener_process);
}



#[tauri::command]
pub fn stop_listening(app_state: tauri::State<AppState>) {
    let listener_process = app_state.listener_process.lock().unwrap().take();
    if let Some(mut listener_process) = listener_process {
        listener_process.kill().expect("failed to kill listener process");
        println!("process killed successfully");
    }else{
        println!("プロセスが起動していません");
        return;
    }
}
