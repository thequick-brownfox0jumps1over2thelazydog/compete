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
        S: Chars,
    }
    debug!(N, S);

    let grouped_S = S
        .iter()
        .group_by(|&&c| c == 'A')
        .into_iter()
        .map(|(is_A, group)| (is_A, group.count()))
        .collect::<Vec<(bool, usize)>>();
    let grouped_S_len = grouped_S.len();
    debug!(grouped_S);

    let mut result = vec![1; N];
    result[0] = if grouped_S[0].0 {
        1
    } else {
        grouped_S[0].1 + 1
    };

    let mut j = 1;
    for i in 0..grouped_S_len {
        let (is_A, count) = grouped_S[i];

        let mut c = count as isize;
        if is_A {
            while c > 0 {
                result[j] = if c > 1 || i == grouped_S_len - 1 {
                    result[j - 1] + 1
                } else {
                    (result[j - 1] + 1).max(grouped_S[i + 1].1 + 1)
                };
                j += 1;
                c -= 1;
                debug!("A", i, j - 1, c + 1, result);
            }
        } else {
            while c > 0 {
                result[j] = if c < count as isize {
                    result[j - 1] - 1
                } else {
                    (result[j - 1] - 1).min(grouped_S[i].1)
                };
                j += 1;
                c -= 1;
                debug!("B", i, j - 1, c + 1, result);
            }
        }
    }

    println!("{}", result.iter().sum::<usize>());
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
