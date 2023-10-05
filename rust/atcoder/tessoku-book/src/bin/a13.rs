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
fn check(query: isize, quota: isize, slice: &[isize]) -> Ordering {
    slice[query as usize].cmp(&quota)
}
*/

#[fastout]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }

    // 二分探索
    /**
    let mut result = 0;
    for i in 0..N - 1 {
        let les = upper_bound(check, i as isize + 1, N as isize, A[i] + K, &A);
        result += les - (i + 1) as isize;
    }
    */
    // 尺取法
    let mut result = 0;
    let mut index = 0;
    for i in 0..N - 1 {
        for j in index.max(i + 1)..N {
            if A[j] - A[i] > K {
                result += j - i - 1;
                index = j;
                break;
            }

            if j == N - 1 {
                result += j - i;
                index = j;
                break;
            }
        }
    }

    println!("{result}");
}
