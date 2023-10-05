#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{cmp::Ordering, collections::HashSet, fmt::Binary, str::MatchIndices};

use ndarray::Order;
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

/**
fn check(aas: &[isize], seconds: isize, quota: isize) -> bool {
    aas.iter().map(|a| seconds / a).sum::<isize>() >= quota
}
*/
fn check(query: isize, quota: isize, slice: &[isize]) -> Ordering {
    slice.iter().map(|a| query / a).sum::<isize>().cmp(&quota)
}

#[fastout]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }

    const TEN: isize = 10;
    const L: isize = TEN.pow(9);

    /**
    let mut left = 1;
    let mut right = L;

    while right - left > 1 {
        let midium = (left + right) / 2;
        if check(&A, midium, K) {
            right = midium;
        } else {
            left = midium;
        };
    }

    println!("{right}");
    */
    println!("{}", lower_bound(check, 1, L, K, &A));
}
