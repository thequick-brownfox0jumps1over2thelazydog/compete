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
            W: isize,
            WV: [(isize, usize); N],
        }
        debug!(N, W, WV);

        let V = N * 1000;

        let mut dp = vec![vec![W + 1; V + 1]; N + 1];
        let mut max_value = 0;
        for i in 0..=N {
            let (w, v) = if i > 0 { WV[i - 1] } else { (0, 0) };

            for j in 0..=V {
                if i == 0 {
                    if j == 0 {
                        dp[i][j] = 0;
                    }
                    continue;
                }

                if dp[i - 1][j] <= W || (j >= v && dp[i - 1][j - v] <= W - w) {
                    dp[i][j] = dp[i - 1][j].min(if j >= v { dp[i - 1][j - v] + w } else { W + 1 });
                    if j > max_value {
                        max_value = j;
                    }
                    debug!(i, j, dp[i][j], max_value);
                }
            }
        }

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
