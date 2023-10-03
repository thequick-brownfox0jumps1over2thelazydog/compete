#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::collections::HashSet;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        D: usize,
        LR: [(Usize1, Usize1); D],
    }

    let mut left_cumulative_maxima: Vec<usize> = vec![0; N + 1];
    for i in 0..N {
        left_cumulative_maxima[i + 1] = A[i].max(left_cumulative_maxima[i])
    }

    let mut right_cumulative_maxima: Vec<usize> = vec![0; N + 1];
    for i in (0..N).rev() {
        right_cumulative_maxima[i] = A[i].max(right_cumulative_maxima[i + 1])
    }

    for lr in LR.iter() {
        println!(
            "{}",
            left_cumulative_maxima[lr.0].max(right_cumulative_maxima[lr.1 + 1])
        );
    }
}
