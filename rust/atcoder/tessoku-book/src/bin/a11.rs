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

    println!("{}", lower_bound(check, 0, N as isize, X, &A) + 1);
}
