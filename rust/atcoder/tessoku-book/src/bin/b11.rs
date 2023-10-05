#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{cmp::Ordering, collections::HashSet};

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
) -> isize {
    while left < right {
        let midium = (left + right) / 2;
        match f(midium, quota, slice) {
            Ordering::Less => left = midium + 1,
            _ => right = midium,
        }
    }

    left
}

fn upper_bound(
    f: fn(isize, isize, &[isize]) -> Ordering,
    mut left: isize,
    mut right: isize,
    quota: isize,
    slice: &[isize],
) -> isize {
    while left < right {
        let midium = (left + right) / 2;
        match f(midium, quota, slice) {
            Ordering::Less | Ordering::Equal => left = midium + 1,
            _ => right = midium,
        }
    }

    left
}

fn check(query: isize, quota: isize, slice: &[isize]) -> Ordering {
    slice[query as usize].cmp(&quota)
}

#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
        Q: usize,
        X: [isize; Q],
    }

    A.sort();

    for x in X.iter() {
        println!("{}", lower_bound(check, 0, N as isize, *x, &A));
    }
}
