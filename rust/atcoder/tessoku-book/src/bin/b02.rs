#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        A: isize,
        B: isize,
    }

    for i in A..=B {
        if 100 % i == 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
