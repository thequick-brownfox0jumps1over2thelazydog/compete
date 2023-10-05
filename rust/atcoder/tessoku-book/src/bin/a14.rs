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
        K: isize,
        A: [isize; N],
        B: [isize; N],
        C: [isize; N],
        D: [isize; N],
    }

    let mut P: HashSet<isize> = HashSet::new();
    for i in 0..N {
        for j in 0..N {
            P.insert(A[i] + B[j]);
        }
    }
    let mut Pv: Vec<_> = P.into_iter().collect();
    Pv.sort();

    let mut Q: HashSet<isize> = HashSet::new();
    for i in 0..N {
        for j in 0..N {
            Q.insert(C[i] + D[j]);
        }
    }
    let q_length = Q.len();
    let mut Qv: Vec<_> = Q.into_iter().collect();
    Qv.sort();

    for p in Pv.iter() {
        if *p < K - Qv[q_length - 1] || *p > K - Qv[0] {
            continue;
        }

        let index = lower_bound(check, 0, Qv.len() as isize - 1, K - p, &Qv, true);
        if index >= 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
