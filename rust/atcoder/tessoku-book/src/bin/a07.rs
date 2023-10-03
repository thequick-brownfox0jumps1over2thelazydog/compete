#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        D: usize,
        N: usize,
        LR: [(Usize1, Usize1); N]
    }

    let mut customer_diffs: Vec<isize> = vec![0; D + 1];
    for lr in LR.iter() {
        customer_diffs[lr.0] += 1;
        customer_diffs[lr.1 + 1] -= 1;
    }

    let mut previous_customers: isize = 0;
    for cd in customer_diffs[..customer_diffs.len() - 1].iter() {
        previous_customers += cd;
        println!("{previous_customers}");
    }
}
