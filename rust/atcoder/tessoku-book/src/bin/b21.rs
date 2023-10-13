#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{
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
            S: Chars,
        }
        debug!(N, S);

        let half = N / 2;

        let mut dp = vec![vec![0; N + 1]; N + 1];
        for i in (0..N).rev() {
            for j in 1..=N {
                if i >= j {
                    dp[i][j] = 0;
                    continue;
                }
                let mut options = vec![1, dp[i + 1][j], dp[i][j - 1]];
                if i != j - 1 && S[i] == S[j - 1] {
                    options.push(dp[i + 1][j - 1] + 2);
                }
                dp[i][j] = options.into_iter().max().unwrap();
                debug!(i, j, S[i..j], dp[i][j]);
            }
        }
        debug!(dp);

        println!("{}", dp[0][N]);
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
