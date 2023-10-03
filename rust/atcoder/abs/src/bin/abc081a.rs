#![allow(non_snake_case)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars,
    }

    println!("{}", S.iter().filter(|&&s| s == '1').count());
}
