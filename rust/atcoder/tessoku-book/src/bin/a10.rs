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
        A: [isize; N],
        D: usize,
        LR: [(Usize1, Usize1); D],
    }

    let mut left_cumulative_maxima = vec![0; N + 1];
    for i in 0..N {
        left_cumulative_maxima[i + 1] = A[i].max(left_cumulative_maxima[i])
    }

    let mut right_cumulative_maxima = vec![0; N + 1];
    for i in (0..N).rev() {
        right_cumulative_maxima[i] = A[i].max(right_cumulative_maxima[i + 1])
    }

    for lr in LR.iter() {
        let l = lr.0;
        let r = lr.1;

        println!(
            "{}",
            left_cumulative_maxima[l].max(right_cumulative_maxima[r + 1])
        );
    }
}
