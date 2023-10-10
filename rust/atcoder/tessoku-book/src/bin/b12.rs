#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{cmp::Ordering, collections::HashSet, fmt::Binary, str::MatchIndices};

use ndarray::Order;
use num_traits::real;
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
    const TEN: isize = 10;

    let value = query * (query.pow(2) + TEN.pow(6));
    value.cmp(&(quota * TEN.pow(9)))
}

#[fastout]
fn main() {
    input! {
        N: isize,
    }

    const TEN: isize = 10;
    const L: isize = TEN.pow(5);
    const FACTOR: isize = TEN.pow(3);

    println!(
        "{:.6}",
        lower_bound(check, 0, L, N, &[]) as f64 / FACTOR as f64
    );
}
