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
    fmt::Binary,
    str::MatchIndices,
};

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
        mut SC: [(isize, isize); N],
    }

    SC.sort_by(|a, b| a.0.cmp(&b.0));

    let mut map = HashMap::new();
    for sc in SC.iter() {
        let s = sc.0;
        let c = sc.1;
        match map.get_mut(&s) {
            Some(n) => *n += c,
            None => _ = map.insert(s, c),
        }

        let binary = format!("{:b}", c);
        let digits = binary.len();
        for i in 0..digits {
            if binary.chars().nth(i).unwrap() == '1' {
                let pow = digits - i - 1;
                let new_size = s * 2_isize.pow(pow as u32);
                match map.get_mut(&new_size) {
                    Some(n) => *n += 1,
                    None => _ = map.insert(new_size, 1),
                }
            }

            if i == digits - 1 {
                match map.get_mut(&s) {
                    Some(n) => {
                        *n = if binary.chars().nth(i).unwrap() == '1' {
                            1
                        } else {
                            0
                        }
                    }
                    None => _ = map.insert(s, 0),
                }
            }
        }
    }

    let xs: Vec<(&isize, &isize)> = map.iter().collect();
    let mut result = 0;
    for x in xs.iter() {
        result += x.1;
    }

    println!("{result}");
}
