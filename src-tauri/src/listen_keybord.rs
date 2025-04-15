use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

use crate::config::APP_STATE;
use crate::config::key_counts::{initialize_key_count, reset_key_count};
use crate::count_keys::{count_keys, check_key};

#[tauri::command]
pub fn start_listening() {
    // /bin/にキーボードの位置取得のバイナリクレートを入れる
    let listen_process_path: &str = "bin/listen-keytype";

    // プロセスが起動しているときは何もしない
    {
        let state = APP_STATE.get_listener_process().lock().expect("Failed to lock mutex");
        if state.is_some() {
            println!("listener process is already running");
            return;
        }
    }

    // key＿cおウントをリセット
    initialize_key_count();

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

                    if let Some(key) = check_key(line.clone()){
                        count_keys(key);
                    }
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
    let mut state = APP_STATE.get_listener_process().lock().expect("Failed to lock mutex");
    *state = Some(listener_process);
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
