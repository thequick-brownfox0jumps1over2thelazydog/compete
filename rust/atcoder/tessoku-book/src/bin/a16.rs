#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

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
            A: [isize; N - 1],
            B: [isize; N - 2],
        }
        debug!(N, A, B);

        /**
        let mut dp = vec![0; N];
        for i in 1..N {
            if i == 1 {
                dp[i] = dp[i - 1] + A[i - 1];
                debug!(i, dp[i - 1], A[i - 1]);
                continue;
            }

            dp[i] = (dp[i - 1] + A[i - 1]).min(dp[i - 2] + B[i - 2]);
            debug!(i, dp[i - 1], A[i - 1], dp[i - 2], B[i - 2]);
        }
        */
        let mut dp = vec![-1; N];
        dp[0] = 0;
        for i in 0..N - 1 {
            dp[i + 1] = if dp[i + 1] < 0 {
                dp[i] + A[i]
            } else {
                min(dp[i + 1], dp[i] + A[i])
            };

            if i == N - 2 {
                continue;
            }

            dp[i + 2] = if dp[i + 2] < 0 {
                dp[i] + B[i]
            } else {
                min(dp[i + 2], dp[i] + B[i])
            };
        }

        println!("{}", dp[N - 1]);
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
