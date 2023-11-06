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
        K: usize,
        AB: [(usize, usize); N],
    }
    debug!(N, K);

    let A = AB.iter().map(|(a, b)| *a).collect::<BTreeSet<usize>>();
    let B = AB.iter().map(|(a, b)| *b).collect::<BTreeSet<usize>>();
    debug!(A, B);

    let mut max_n_students = 0;
    for min_a in A.iter() {
        let max_a = &(min_a + K);

        for min_b in B.iter() {
            let max_b = &(min_b + K);

            let mut n_students = 0;
            for (a, b) in AB.iter() {
                if min_a <= a && a <= max_a && min_b <= b && b <= max_b {
                    n_students += 1;
                }
            }

            max_n_students = max(max_n_students, n_students);
        }
    }

    println!("{max_n_students}");
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
