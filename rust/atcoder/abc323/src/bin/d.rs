#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

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
        mut SC: [(isize, isize); N],
    }

    SC.sort_by(|a, b| a.0.cmp(&b.0));

    let mut slimes_map = HashMap::new();
    for sc in SC.iter() {
        let s = sc.0;
        let c = sc.1;

        let mut n_factor = 0;
        for si in format!("{:b}", s).chars().rev() {
            if si == '1' {
                break;
            }

            n_factor += 1;
        }
        let group = s >> n_factor;
        let amount = s * c / group;
        slimes_map
            .entry(group)
            .and_modify(|n| *n += amount)
            .or_insert(amount);
    }

    let mut result = 0;
    for (group, n_slimes) in slimes_map.iter() {
        result += format!("{:b}", n_slimes)
            .chars()
            .filter(|c| *c == '1')
            .count()
    }

    println!("{result}");
}
