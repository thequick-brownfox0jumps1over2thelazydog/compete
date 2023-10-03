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
        N: usize,
        Q: usize,
        A: [isize; N],
        LR: [(Usize1, Usize1); Q],
    }

    let mut cumulative_customers: Vec<isize> = vec![0; N + 1];
    for i in 1..=N {
        cumulative_customers[i] = cumulative_customers[i - 1] + A[i - 1];
    }

    for j in 0..Q {
        println!(
            "{}",
            cumulative_customers[LR[j].1 + 1] - cumulative_customers[LR[j].0]
        );
    }
}
