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
    cmp::{max, min, Ordering},
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
        X: usize,
        mut A: Chars,
    }
    debug!(N, X, A);

    let mut queues = VecDeque::new();
    queues.push_back(X);
    A[X - 1] = '@';

    while !queues.is_empty() {
        let blue = queues.pop_front().unwrap();
        if blue > 1 && A[blue - 2] == '.' {
            A[blue - 2] = '@';
            queues.push_back(blue - 1);
        }
        if blue < N && A[blue] == '.' {
            A[blue] = '@';
            queues.push_back(blue + 1);
        }
    }

    println!(
        "{}",
        A.iter()
            .fold(String::from(""), |a, b| a + b.to_string().as_str())
    );
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
