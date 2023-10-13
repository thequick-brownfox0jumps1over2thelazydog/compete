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
            A: [Usize1; N - 1],
            B: [Usize1; N - 1],
        }
        debug!(N, A, B);

        let mut dp = vec![-1; N];
        dp[0] = 0;
        for i in 0..N - 1 {
            if dp[i] < 0 {
                continue;
            }

            let a = A[i];
            let b = B[i];

            dp[a] = max(dp[a], dp[i] + 100);
            dp[b] = max(dp[b], dp[i] + 150);
            debug!(i, a, b, dp[a], dp[b]);
        }
        debug!(dp);

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
