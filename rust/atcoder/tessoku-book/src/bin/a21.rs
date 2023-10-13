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
            PA: [(Usize1, isize); N],
        }
        debug!(N, PA);

        let mut dp = vec![vec![0; N + 1]; N + 1];
        let mut max_score = 0;
        for i in 0..=N {
            for j in (i..=N).rev() {
                let mut options = vec![];
                if i > 0 {
                    let p = PA[i - 1].0;
                    let local_max_score =
                        dp[i - 1][j] + if i <= p && p < j { PA[i - 1].1 } else { 0 };
                    options.push(local_max_score);
                }
                if j < N {
                    let p = PA[j].0;
                    let local_max_score = dp[i][j + 1] + if i <= p && p < j { PA[j].1 } else { 0 };
                    options.push(local_max_score);
                }

                dp[i][j] = options.into_iter().max().unwrap_or(0);
                if dp[i][j] > max_score {
                    max_score = dp[i][j];
                }
            }
        }

        println!("{max_score}");
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
