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
            S: usize,
            A: [usize; N],
        }
        debug!(N, S, A);

        let mut dp = vec![vec![false; S + 1]; N + 1];
        for i in 0..=N {
            for j in 0..=S {
                if i == 0 {
                    if j == 0 {
                        dp[i][j] = true;
                    }
                    continue;
                }

                if dp[i - 1][j] || (j >= A[i - 1] && dp[i - 1][j - A[i - 1]]) {
                    dp[i][j] = true;
                }
            }
        }
        debug!(dp);

        if !dp[N][S] {
            println!("-1");
            return;
        }

        let mut reversed_combination = vec![];
        let mut total = S;
        for i in (0..N).rev() {
            debug!(i, total, A[i]);
            if total >= A[i] && dp[i][total - A[i]] {
                reversed_combination.push(i + 1);
                total -= A[i];
                if total == 0 {
                    break;
                }
            }
            debug!(i, reversed_combination);
        }

        let combination = reversed_combination
            .iter()
            .rev()
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        println!("{}", combination.len());
        println!("{}", combination.join(" "));
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
