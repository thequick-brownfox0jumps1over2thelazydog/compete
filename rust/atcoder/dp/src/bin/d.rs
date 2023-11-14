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
        W: usize,
        WV: [(usize, usize); N],
    }

    let mut dp = vec![vec![-1; W + 1]; N + 1];
    dp[0][0] = 0;

    let mut max_value = 0;
    for i in 0..N {
        let (weight, value) = WV[i];

        for j in 0..=W {
            if dp[i][j] == -1 {
                continue;
            }

            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);

            let new_weight = j + weight;
            if new_weight <= W {
                dp[i + 1][new_weight] = max(dp[i + 1][new_weight], dp[i][j] + value as isize);
            }

            if i == N - 1 {
                let current_value = if new_weight <= W {
                    max(dp[i + 1][j], dp[i + 1][new_weight])
                } else {
                    dp[i + 1][j]
                };
                max_value = max(max_value, current_value);
            }
        }
    }

    println!("{max_value}");
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
