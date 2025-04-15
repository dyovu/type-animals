use std::collections::{HashMap, VecDeque};

use crate::config::key_counts::{get_keycount, pop_and_refill};


pub fn count_keys(typed_key: char){
    let mut key_counts = get_keycount();


    for (key, dq) in key_counts.iter_mut() {
        if let Some(&front) = dq.front() {
            println!("'{}' の先頭は '{}' です", key, front);
            if typed_key == front {
                let finished_typed:bool = pop_and_refill(key);






                dq.pop_front();
                println!("'{}' matched front of '{}' → updated dequeue: {:?}", typed_key, key, dq);
                // println!(dq);

                if dq.is_empty() {
                    println!("'{}' の入力が完了しました", key);

                    dq.extend(key.chars())
                    // *dq = &key.chars().collect::<VecDeque<char>>()
                }
            } else {
                println!("'{}' は '{}' の先頭ではない", typed_key, key);
            }
        }
    }
}



pub fn check_key(key: String) -> Option<char>{
    if &key[0..3] ==  "Key" {
        return Some(key.chars().nth(3).unwrap().to_lowercase().next().unwrap());
    }else{
        return None;
    }
}

    