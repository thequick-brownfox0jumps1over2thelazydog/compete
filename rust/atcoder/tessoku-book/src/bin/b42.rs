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
        AB: [(isize, isize); N],
    }
    debug!(N, AB);

    let pp_merged_AB = AB.iter().map(|(a, b)| a + b).collect::<Vec<isize>>();
    let pp_score = pp_merged_AB.iter().filter(|&&n| n > 0).sum::<isize>();

    let pn_merged_AB = AB.iter().map(|(a, b)| a - b).collect::<Vec<isize>>();
    let pn_score = pn_merged_AB.iter().filter(|&&n| n > 0).sum::<isize>();

    let np_merged_AB = AB.iter().map(|(a, b)| -a + b).collect::<Vec<isize>>();
    let np_score = np_merged_AB.iter().filter(|&&n| n > 0).sum::<isize>();

    let nn_merged_AB = AB.iter().map(|(a, b)| -a - b).collect::<Vec<isize>>();
    let nn_score = nn_merged_AB.iter().filter(|&&n| n > 0).sum::<isize>();

    println!("{}", max(pp_score, max(pn_score, max(np_score, nn_score))));
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
