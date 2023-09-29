#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        A: usize,
        B: usize,
    }

    println!("{}", A + B);
}
