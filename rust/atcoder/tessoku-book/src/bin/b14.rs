#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_doc_comments)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{cmp::Ordering, collections::HashSet, fmt::Binary, str::MatchIndices};

use ndarray::Order;
use num_traits::real;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use rand_core::le;

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

#[fastout]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }

    let left_power = N / 2;
    let right_power = N - left_power;

    const TWO: usize = 2;
    let left = TWO.pow(left_power as u32);
    let right = TWO.pow(right_power as u32);

    let mut L: HashSet<isize> = HashSet::new();
    for i in 0..left {
        format!("{:0digits$b}", i, digits = left_power);
    }

    let mut R: HashSet<isize> = HashSet::new();
    for j in 0..right {
        format!("{:0digits$b}", j, digits = right_power);
    }
}
