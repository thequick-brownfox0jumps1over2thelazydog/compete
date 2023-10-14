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
            H: usize,
            W: usize,
            C: [Chars; H],
        }
        debug!(H, W, C);

        let mut dp: Vec<Vec<usize>> = vec![vec![0; W]; H];
        dp[0][0] = 1;
        for i in 0..H {
            for j in 0..W {
                if C[i][j] == '#' || dp[i][j] == 0 {
                    continue;
                }

                if i < H - 1 && C[i + 1][j] == '.' {
                    dp[i + 1][j] += dp[i][j];
                }
                if j < W - 1 && C[i][j + 1] == '.' {
                    dp[i][j + 1] += dp[i][j];
                }
            }
        }
        debug!(dp);

        println!("{}", dp[H - 1][W - 1]);
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
