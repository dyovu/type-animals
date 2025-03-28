use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};



pub struct AppState {
    pub listener_process: Mutex<Option<std::process::Child>>, // マウス位置取得プロセス
}


fn start_listening(){
    let listen_process_path = "bin/listen_keybord";
    let listener_process : Child = Command::new(listen_process_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute listener process");
    listener_process


}