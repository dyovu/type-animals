use std::io::{BufReader, BufRead, Read};
use std::thread;
use std::sync::mpsc::Sender;

use crate::count_keys::{count_keys, check_key};
use crate::config::sdl::Message;

pub fn listening_key (listener_process: &mut std::process::Child, tx: Sender<Message>) {
    // listener_processのstdoutを受け取るための"パイプ"が作られる
    // 
    if let Some(stdout) = listener_process.stdout.take() { 
        let reader = BufReader::new(stdout);
        thread::spawn(move || {
            process_key_events(reader, tx);
            
        });
    }
}

fn process_key_events(reader: BufReader<impl Read>, tx: Sender<Message>) {
    for line in reader.lines() {
        let Ok(line) = line else { 
            continue; 
        };
        
        println!("pressed key of {}", line);
        if let Some(key) = check_key(line.clone()) {
            if let Some(path) = count_keys(key) {
                // メインスレッドに画像表示を依頼
                println!("path: {:?}", path);
                let _ = tx.send(
                    Message::DisplayGif {
                        path: path.into_os_string().into_string().unwrap(),
                        duration: 3,
                    }
                );
            }
        }

    }
}