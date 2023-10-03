#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        X: isize,
        A: [isize; N],
    }

    for &a in A.iter() {
        if a == X {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
