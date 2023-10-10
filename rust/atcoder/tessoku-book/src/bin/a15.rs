#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_doc_comments)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

fn lower_bound(
    f: fn(isize, isize, &[isize]) -> Ordering,
    mut left: isize,
    mut right: isize,
    quota: isize,
    slice: &[isize],
    strict_equal: bool,
) -> isize {
    while left < right {
        let midium = (left + right) / 2;
        match f(midium, quota, slice) {
            Ordering::Less => left = midium + 1,
            Ordering::Equal => {
                if strict_equal {
                    return midium;
                } else {
                    right = midium
                }
            }
            _ => right = midium,
        }
    }

    if !strict_equal {
        left
    } else {
        -1
    }
}

fn upper_bound(
    f: fn(isize, isize, &[isize]) -> Ordering,
    mut left: isize,
    mut right: isize,
    quota: isize,
    slice: &[isize],
    strict_equal: bool,
) -> isize {
    while left < right {
        let midium = (left + right) / 2;
        match f(midium, quota, slice) {
            Ordering::Less => left = midium + 1,
            Ordering::Equal => {
                if strict_equal {
                    return midium;
                } else {
                    left = midium + 1
                }
            }
            _ => right = midium,
        }
    }

    if !strict_equal {
        left
    } else {
        -1
    }
}

fn check(query: isize, quota: isize, slice: &[isize]) -> Ordering {
    slice[query as usize].cmp(&quota)
}

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }

    let set_A = A.iter().collect::<HashSet<_>>();
    let mut unique_A = set_A.into_iter().collect::<Vec<_>>();
    unique_A.sort();

    let value_index_map = unique_A
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, i + 1))
        .collect::<HashMap<_, _>>();
    let B = A
        .iter()
        .map(|a| value_index_map[a].to_string())
        .collect::<Vec<_>>();

    println!("{}", B.join(" "));
}
