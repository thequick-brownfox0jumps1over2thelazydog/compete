#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::precedence)]

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    thread::Builder,
};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! debug {
    ($($var:expr),+) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($(stringify!($var), "={:?}  "),+), $(&$var),+);
    };
}

#[derive(Default)]
struct Solver {}

impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            M: usize,
            A: [[usize; N]; M],
        }
        debug!(N, M, A);

        let n_sets = 2_usize.pow(N as u32);

        let mut dp = vec![vec![-1; n_sets]; M + 1];
        dp[0][0] = 0;
        for i in 0..M {
            let coupon_string = A[i]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .concat();
            let coupon_coverage = usize::from_str_radix(&coupon_string, 2).unwrap();
            debug!(i, coupon_coverage);

            for j in 0..n_sets {
                if dp[i][j] < 0 {
                    continue;
                }

                dp[i + 1][j] = if dp[i + 1][j] >= 0 {
                    min(dp[i + 1][j], dp[i][j])
                } else {
                    dp[i][j]
                };
                dp[i + 1][j | coupon_coverage] = if dp[i + 1][j | coupon_coverage] >= 0 {
                    min(dp[i + 1][j | coupon_coverage], dp[i][j] + 1)
                } else {
                    dp[i][j] + 1
                };
                debug!(
                    i,
                    j,
                    dp[i][j],
                    dp[i + 1][j],
                    j | coupon_coverage,
                    dp[i + 1][j | coupon_coverage]
                );
            }
        }
        debug!(dp);

        println!(
            "{}",
            if dp[M][n_sets - 1] > 0 {
                dp[M][n_sets - 1]
            } else {
                -1
            }
        );
    }
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
