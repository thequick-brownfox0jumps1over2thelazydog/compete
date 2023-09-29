#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn get_next_index(string: &str) -> i8 {
    for token in ["dreamer", "dream", "eraser", "erase"] {
        if !string.starts_with(token) {
            continue;
        }

        if token != "dreamer" {
            return token.len() as i8;
        }

        if string.len() == 7 {
            return 7;
        }
        match string.chars().nth(7).unwrap() {
            'a' => {
                return 5;
            }
            'd' | 'e' => {
                return 7;
            }
            _ => {
                return -1;
            }
        }
    }

    -1
}

fn main() {
    input! {
        S: String,
    }

    let s: &str = &S;
    let mut index: usize = 0;

    while true {
        let next_index = get_next_index(&s[index..]);
        if next_index < 0 {
            println!("NO");
            break;
        }

        index += next_index as usize;
        if index == s.len() {
            println!("YES");
            break;
        }
    }
}
