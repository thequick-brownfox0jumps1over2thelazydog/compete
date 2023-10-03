#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        mut D: [isize; N],
    }

    let ds: HashSet<_> = D.into_iter().collect();

    println!("{}", ds.len());
}
