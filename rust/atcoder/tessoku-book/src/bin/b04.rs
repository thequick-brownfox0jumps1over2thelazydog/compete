#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(clippy::from_str_radix_10)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: String,
    }

    // TODO: `from_str_radix` を使わない（推奨されている `parse` を使う）方法を模索
    //let n: &str = &N;
    //println!("{:?}", n.parse::<u32>().unwrap());
    println!("{}", isize::from_str_radix(&N, 2).unwrap());
}
