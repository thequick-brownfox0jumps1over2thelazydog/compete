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
            A: [isize; N - 1],
            B: [isize; N - 2],
        }
        debug!(N, A, B);

        let mut dp = vec![0; N];
        for i in 1..N {
            if i == 1 {
                dp[i] = dp[i - 1] + A[i - 1];
                continue;
            }

            dp[i] = (dp[i - 1] + A[i - 1]).min(dp[i - 2] + B[i - 2]);
        }

        let mut reversed_route = vec![N];
        let mut index = N - 1;
        while index > 0 {
            if dp[index] == dp[index - 1] + A[index - 1] {
                reversed_route.push(index);
                index -= 1;
            } else {
                reversed_route.push(index - 1);
                index -= 2;
            }
        }

        let route = reversed_route
            .iter()
            .rev()
            .map(|n| n.to_string())
            .collect::<Vec<String>>();
        println!("{}", route.len());
        println!("{}", route.join(" "));
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
