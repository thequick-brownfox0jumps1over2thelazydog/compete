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
    // 19 [min]
    // from: 2023-11-17 12:55
    // to: 2023-11-17 13:14
    input! {
        S: Chars,
    }

    let mut stacks = vec![];
    let mut stack_size = 0;
    for i in 0..S.len() {
        stacks.push(S[i]);
        stack_size += 1;

        if stack_size < 3 {
            continue;
        }

        if stacks[stack_size - 3..] == ['A', 'B', 'C'] {
            stacks.pop();
            stacks.pop();
            stacks.pop();
            stack_size -= 3;
        }
    }

    if stack_size > 0 {
        println!("{}", stacks.iter().map(|c| c.to_string()).join(""));
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
