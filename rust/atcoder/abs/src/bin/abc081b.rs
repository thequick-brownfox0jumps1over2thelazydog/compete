#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: u8,
        A: [u32; N],
    }

    let mut min_n_factors: usize = 30;
    for a in A.iter() {
        let n_factors = format!("{:b}", a)
            .chars()
            .rev()
            .position(|c| c == '1')
            .expect("Any a must be greater than 0");
        if min_n_factors > n_factors {
            min_n_factors = n_factors;
        }
    }

    println!("{min_n_factors}");
}
