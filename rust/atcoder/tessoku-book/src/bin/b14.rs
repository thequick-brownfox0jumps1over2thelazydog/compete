#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_doc_comments)]
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
    }

    let left_power = N / 2;
    let right_power = N - left_power;

    let left = 2_isize.pow(left_power as u32);
    let right = 2_isize.pow(right_power as u32);

    let mut L: HashSet<isize> = HashSet::new();
    for i in 0..left {
        let mut left_total = 0;
        for (k, c) in format!("{:0digits$b}", i, digits = left_power)
            .chars()
            .enumerate()
        {
            if c == '1' {
                left_total += A[k]
            }
        }

        L.insert(left_total);
    }

    let mut R: HashSet<isize> = HashSet::new();
    for j in 0..right {
        let mut right_total = 0;
        for (m, c) in format!("{:0digits$b}", j, digits = right_power)
            .chars()
            .enumerate()
        {
            if c == '1' {
                right_total += A[left_power + m]
            }
        }

        R.insert(right_total);
    }

    let mut Rv: Vec<_> = R.into_iter().collect();
    Rv.sort();

    for l in L.iter() {
        let index = lower_bound(check, 0, Rv.len() as isize, K - l, &Rv, true);
        if index >= 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
