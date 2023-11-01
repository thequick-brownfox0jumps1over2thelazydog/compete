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
        Q: usize,
        Queries: [(usize, isize); Q],
    }
    debug!(Q, Queries);

    let mut set = BTreeSet::new();
    for (query_type, number) in Queries.into_iter() {
        if query_type == 1 {
            set.insert(number);
            continue;
        }

        if set.is_empty() {
            println!("-1");
            continue;
        }

        let upper = set.range(number..).next().unwrap_or(&2_000_000_000);
        let lower = set.range(..=number).next_back().unwrap_or(&-1_000_000_000);
        println!("{}", (upper - number).min(number - lower));
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
