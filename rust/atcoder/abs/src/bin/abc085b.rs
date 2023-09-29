#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: u8,
        mut D: [u8; N],
    }

    let ds: HashSet<u8> = D.into_iter().collect();

    println!("{}", ds.len());
}
