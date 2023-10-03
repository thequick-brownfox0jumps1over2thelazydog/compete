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

    let mut cumulative_customers = vec![0; N + 1];
    for i in 1..=N {
        cumulative_customers[i] = cumulative_customers[i - 1] + A[i - 1];
    }

    for lr in LR.iter() {
        let l = lr.0;
        let r = lr.1;

        println!("{}", cumulative_customers[r + 1] - cumulative_customers[l]);
    }
}
