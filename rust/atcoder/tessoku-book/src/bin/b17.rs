#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
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
            H: [isize; N],
        }
        debug!(N, H);

        let mut dp = vec![0; N];
        for i in (0..N - 1).rev() {
            if i == N - 2 {
                dp[i] = (H[i] - H[i + 1]).abs();
                debug!(i, dp[i], H[i], H[i + 1]);
                continue;
            }

            dp[i] = (dp[i + 1] + (H[i] - H[i + 1]).abs()).min(dp[i + 2] + (H[i] - H[i + 2]).abs());
            debug!(
                i,
                dp[i],
                dp[i + 1],
                H[i],
                H[i + 1],
                dp[i + 2],
                H[i],
                H[i + 2]
            );
        }

        let mut route = vec![1];
        let mut index = 0;
        while index < N - 1 {
            if dp[index] == dp[index + 1] + (H[index] - H[index + 1]).abs() {
                route.push(index + 2);
                index += 1;
            } else {
                route.push(index + 3);
                index += 2;
            }
        }

        let result = route.iter().map(|n| n.to_string()).collect::<Vec<String>>();
        println!("{}", route.len());
        println!("{}", result.join(" "));
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
