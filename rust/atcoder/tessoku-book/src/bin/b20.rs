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
            S: Chars,
            T: Chars,
        }
        debug!(S, T);

        let S_len = S.len();
        let T_len = T.len();
        let max_distance = S_len.max(T_len);

        let mut dp = vec![vec![max_distance; T.len() + 1]; S.len() + 1];
        for i in 0..=S_len {
            for j in 0..=T_len {
                if i == 0 || j == 0 {
                    dp[i][j] = i.max(j);
                    continue;
                }

                dp[i][j] = *vec![
                    dp[i - 1][j] + 1,
                    dp[i][j - 1] + 1,
                    dp[i - 1][j - 1] + if S[i - 1] == T[j - 1] { 0 } else { 1 },
                ]
                .iter()
                .min()
                .unwrap();
            }
        }

        println!("{}", dp[S_len][T_len]);
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
