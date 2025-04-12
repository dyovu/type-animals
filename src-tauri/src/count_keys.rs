use std::collections::{HashMap, VecDeque};


pub fn count_keys(typed_key :char){
    let mut key_counts: HashMap< String, VecDeque<char>> = HashMap::from([
        ("cat".to_string(), "cat".chars().collect::<VecDeque<char>>())
    ]);


    for (key, dq) in key_counts.iter_mut() {
        if let Some(&front) = dq.front() {
            if typed_key == front {
                dq.pop_front();
                println!("'{}' matched front of '{}' → updated queue: {:?}", typed_key, key, dq);

                // すべての文字を入力し終わった場合
                if dq.is_empty() {
                    println!("'{}' 完成！", key);
                }
            } else {
                println!("'{}' は '{}' の先頭ではない", typed_key, key);
            }
        }
    }




}

    