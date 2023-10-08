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
    default,
    fmt::Binary,
    hash::Hash,
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

/**

2 8
1000 1000 500 500 500 500 500 500
xxoooooo
ooxxxxxx

 */

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [isize; M], // 配点。500 <= 100n <= 2500
        S: [Chars; N], // 正解しているか否か
    }

    let mut sorted_A = A.clone();
    sorted_A.sort();

    let mut max_score = 0;
    let mut scores = vec![0; N];
    let mut default_score_map = HashMap::new();
    for i in 0..21 {
        default_score_map.insert(500 + i * 100, 0);
    }
    let mut gained_points: Vec<HashMap<isize, isize>> = vec![default_score_map; N];

    for i in 0..N {
        for j in 0..M {
            if S[i][j] == 'o' {
                scores[i] += A[j];
                match gained_points[i].get_mut(&A[j]) {
                    Some(x) => *x += 1,
                    None => _ = gained_points[i].insert(A[j], 1),
                }
            }
        }

        scores[i] += i as isize + 1;

        if max_score < scores[i] {
            max_score = scores[i];
        }
    }

    for i in 0..N {
        let mut diff = max_score - scores[i];
        if diff == 0 {
            println!("0");
            continue;
        }

        let mut counter = 0;
        for a in sorted_A.iter().rev() {
            if *gained_points[i].get(a).unwrap_or(&0) > 0 {
                match gained_points[i].get_mut(a) {
                    Some(x) => *x -= 1,
                    None => _ = gained_points[i].insert(*a, 0),
                }
                continue;
            }

            diff -= a;
            counter += 1;
            if diff < 0 {
                println!("{counter}");
                break;
            }
        }
    }
}
