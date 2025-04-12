use std::collections::{HashMap, VecDeque};


pub fn count_keys(typed_key :char){
    let mut key_counts: HashMap< String, VecDeque<char>> = HashMap::from([
        ("cat".to_string(), "cat".chars().collect::<VecDeque<char>>())
    ]);


    for (key, value) in key_counts{
        if &typed_key == value.front().unwrap(){
            let key_c: char = *value.pop_front().unwrap();

            
        }else{

        }
    }




}

    