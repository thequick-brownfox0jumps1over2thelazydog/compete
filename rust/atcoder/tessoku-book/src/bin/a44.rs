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
        Q: usize,
    }
    debug!(N, Q);

    let mut A = (1..=N).collect::<Vec<usize>>();
    let mut is_reversed = false;

    for q in 0..Q {
        input! {
            query_type: usize,
        }

        match query_type {
            1 => {
                input! {
                    i: Usize1,
                    n: usize,
                }

                A[if is_reversed { N - i - 1 } else { i }] = n;
            }
            2 => is_reversed = !is_reversed,
            3 => {
                input! {
                    i: Usize1,
                }

                println!("{}", A[if is_reversed { N - i - 1 } else { i }]);
            }
            _ => unreachable!(),
        }

        debug!(A);
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
