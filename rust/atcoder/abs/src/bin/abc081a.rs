#![allow(non_snake_case)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars,
    }

    let result = S.iter().filter(|&&s| s == '1').count();

    println!("{result}");
}
