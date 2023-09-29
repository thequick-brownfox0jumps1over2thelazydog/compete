#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: isize,
        mut D: [isize; N],
    }

    let ds: HashSet<isize> = D.into_iter().collect();

    println!("{}", ds.len());
}
