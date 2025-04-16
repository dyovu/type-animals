use std::path::PathBuf;

use crate::config::key_counts::{get_keycount, pop_and_refill};
use crate::config::json_data::JSON_DATA;


pub fn count_keys(typed_key: char) -> Option<PathBuf> {
    let mut key_counts = get_keycount();

    for (key, dq) in key_counts.iter_mut() {
        if let Some(&front) = dq.front() {
            println!("'{}' の先頭は '{}' です", key, front);
            if typed_key == front {
                let finished_typed:bool = pop_and_refill(key);
                if finished_typed{
                    let json = JSON_DATA.lock().unwrap();
                    let path:&str = json.get(key).unwrap();
                    return Some(PathBuf::from(path));
                }
            } else {
                println!("'{}' は '{}' の先頭ではない", typed_key, key);
                return None;
            }
        }
    }
    return None;
}




pub fn check_key(key: String) -> Option<char>{
    if &key[0..3] ==  "Key" {
        return Some(key.chars().nth(3).unwrap().to_lowercase().next().unwrap());
    }else{
        return None;
    }
}

    