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
            W: usize,
            WV: [(usize, isize); N],
        }
        debug!(N, W, WV);

        let mut dp = vec![vec![-1; W + 1]; N + 1];
        let mut max_value = 0;
        for i in 0..=N {
            let (w, v) = if i > 0 { WV[i - 1] } else { (0, 0) };
            for j in 0..=W {
                if i == 0 {
                    if j == 0 {
                        dp[i][j] = 0;
                    }
                    continue;
                }

                if dp[i - 1][j] >= 0 || (j >= w && dp[i - 1][j - w] >= 0) {
                    dp[i][j] = dp[i - 1][j].max(if j >= w { dp[i - 1][j - w] + v } else { -1 });
                    if dp[i][j] > max_value {
                        max_value = dp[i][j];
                    }
                }
            }
        }
        debug!(dp);

        println!("{max_value}");
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
