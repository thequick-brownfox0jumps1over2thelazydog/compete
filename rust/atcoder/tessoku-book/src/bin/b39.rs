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
    fmt::{Binary, Debug},
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
        D: usize,
        mut XY: [(Usize1, usize); N],
    }
    debug!(N, D, XY);

    XY.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    let group: BTreeMap<usize, Vec<usize>> =
        XY.into_iter()
            .fold(BTreeMap::new(), |mut m, (day, reward)| {
                m.entry(day)
                    .and_modify(|v| v.push(reward))
                    .or_insert(vec![reward]);
                m
            });
    debug!(group);

    let mut result = 0;
    let mut priority_queues = BinaryHeap::new();
    let empty_vec: Vec<usize> = vec![];
    for i in 0..D {
        let rewards = group.get(&i).unwrap_or(&empty_vec);
        for r in rewards {
            priority_queues.push(r);
        }

        result += priority_queues.pop().unwrap_or(&0);
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
