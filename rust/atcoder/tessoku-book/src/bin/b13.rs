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

#[fastout]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }

    let mut cumulative_sums = vec![0; N + 1];
    for i in 1..=N {
        cumulative_sums[i] = cumulative_sums[i - 1] + A[i - 1];
    }

    let mut result = 0;
    let mut index = 0;
    for i in 0..N {
        for j in index.max(i + 1)..=N {
            if cumulative_sums[j] - cumulative_sums[i] > K {
                result += j - i - 1;
                index = j;
                break;
            }
            if j == N {
                result += j - i;
                index = j;
                break;
            }
        }
    }

    println!("{result}");
}
