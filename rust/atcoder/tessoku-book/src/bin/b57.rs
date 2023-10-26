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
        K: usize,
    }
    debug!(N, K);

    let mut dp = vec![vec![N; 30]; N + 1];
    for j in 0..30 {
        for i in 0..=N {
            if i == 0 {
                dp[i][j] = 0;
                continue;
            }

            if j == 0 {
                dp[i][j] = i - i
                    .to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .sum::<usize>();
                continue;
            }

            dp[i][j] = dp[dp[i][j - 1]][j - 1];
        }
    }
    debug!(dp);

    for i in 1..=N {
        let mut result = i;
        for j in 0..30 {
            if K >> j & 1 == 1 {
                result = dp[result][j];
            }
        }

        println!("{}", result);
    }
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
