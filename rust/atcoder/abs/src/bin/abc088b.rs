#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: u8,
        mut A: [i16; N],
    }

    let mut result: i16 = 0;
    A.sort();

    for (index, a) in A.iter().rev().enumerate() {
        if index % 2 == 0 {
            result += a;
        } else {
            result -= a;
        }
    }

    println!("{result}");
}
