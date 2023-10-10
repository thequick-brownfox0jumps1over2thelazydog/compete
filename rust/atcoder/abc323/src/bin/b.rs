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
        S: [Chars; N],
    }

    let mut player_and_wins: Vec<(isize, isize)> = vec![(0, 0); N];

    for i in 0..N {
        let n_wins = S[i].iter().filter(|c| **c == 'o').count();
        player_and_wins[i] = (i as isize + 1, n_wins as isize);
    }

    player_and_wins.sort_by(|a, b| (-a.1).cmp(&(-b.1)));

    let mut result = vec![String::from(""); N];
    for (i, pw) in player_and_wins.iter().enumerate() {
        result[i] = pw.0.to_string();
    }

    println!("{}", result.join(" "));
}
