#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{cmp::Ordering, collections::HashSet, str::MatchIndices};

use ndarray::Order;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

fn lower_bound(
    f: fn(usize, isize, &[isize]) -> Ordering,
    mut left: usize,
    mut right: usize,
    quota: isize,
    slice: &[isize],
) -> usize {
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
    f: fn(usize, isize, &[isize]) -> Ordering,
    mut left: usize,
    mut right: usize,
    quota: isize,
    slice: &[isize],
) -> usize {
    while left < right {
        let midium = (left + right) / 2;
        match f(midium, quota, slice) {
            Ordering::Less | Ordering::Equal => left = midium + 1,
            _ => right = midium,
        }
    }

    left
}

fn check(query: usize, quota: isize, slice: &[isize]) -> Ordering {
    slice[query].cmp(&quota)
}

#[fastout]
fn main() {
    input! {
        N: usize,
        X: isize,
        A: [isize; N],
    }

    /**
    let mut left = 0;
    let mut right = N;
    while true {
        let midium = (left + right) / 2;
        match X - A[midium] {
            1.. => left = midium + 1,
            0 => {
                println!("{}", midium + 1);
                return;
            }
            _ => right = midium,
        }
    }
    */

    //println!("{}", A.binary_search(&X).expect("Item X not found.") + 1);

    println!("{}", lower_bound(check, 0, N, X, &A) + 1);
}
