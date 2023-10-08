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

#[fastout]
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let mut vs: Vec<(isize, isize)> = vec![(0, 0); N];

    for i in 0..N {
        let wins = S[i].iter().filter(|c| **c == 'o').count();
        vs[i] = (i as isize + 1, wins as isize);
    }

    vs.sort_by(|a, b| (-a.1).cmp(&(-b.1)));
    let mut result: Vec<String> = vec![String::from(""); N];
    for (index, v) in vs.iter().enumerate() {
        result[index] = v.0.to_string();
    }

    println!("{}", result.join(" "));
}
