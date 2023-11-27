#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_attributes)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::precedence)]

use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    iter::FromIterator,
    thread::Builder,
};

use itertools::Itertools;
use num::integer::{gcd, lcm};
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

#[fastout]
fn solve() {
    input! {
        N: usize,
        P: [f64; N],
    }

    let mut max_rate = f64::MIN;
    let mut dp = vec![vec![0.0; N + 1]; N + 1];
    let mut denominator = 0.0;
    for k in 1..=N {
        denominator = 0.9 * denominator + 1.0;
        let last_term = 1200.0 / (k as f64).sqrt();

        for j in 1..=N {
            let composite = 0.9 * dp[k - 1][j - 1] + P[j - 1];

            match k.cmp(&j) {
                Ordering::Greater => {}
                Ordering::Equal => {
                    dp[k][j] = composite;
                }
                Ordering::Less => {
                    dp[k][j] = if dp[k][j - 1] < composite {
                        composite
                    } else {
                        dp[k][j - 1]
                    };
                }
            }
        }
        debug!(k, dp[k][N]);

        let rate = dp[k][N] / denominator - last_term;
        if max_rate < rate {
            max_rate = rate;
        }
    }
    debug!(dp);

    println!("{max_rate}");
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
