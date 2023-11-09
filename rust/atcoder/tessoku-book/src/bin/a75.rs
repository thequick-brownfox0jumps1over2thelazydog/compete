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
        mut TD: [(usize, usize); N],
    }
    debug!(N);

    TD.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    debug!(TD);

    let mut dp = vec![vec![-1; 144_000]; N + 1];
    dp[0][0] = 0;

    let mut result = 0;
    let mut time_range = 0;
    for i in 0..N {
        let (time, due_time) = TD[i];

        for j in 0..=time_range {
            if dp[i][j] == -1 {
                continue;
            }

            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            if j + time <= due_time {
                dp[i + 1][j + time] = max(dp[i + 1][j + time], dp[i][j] + 1);
            }

            result = max(result, max(dp[i + 1][j], dp[i + 1][j + time]));
        }

        time_range += time;
    }

    println!("{result}");
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
