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
        Q: usize,
        A: [Usize1; N],
        XY: [(Usize1, usize); Q],
    }
    debug!(N, Q, A, XY);

    let mut dp = vec![vec![N; 30]; N];
    for j in 0..30 {
        for i in 0..N {
            if j == 0 {
                dp[i][j] = A[i];
                continue;
            }

            dp[i][j] = dp[dp[i][j - 1]][j - 1];
        }
    }
    debug!(dp);

    for (x, y) in XY.into_iter() {
        let mut result = x;
        for j in 0..30 {
            if y >> j & 1 == 1 {
                result = dp[result][j];
                debug!(y, j, result);
            }
        }

        println!("{}", result + 1);
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
