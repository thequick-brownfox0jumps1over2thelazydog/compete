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
        M: usize,
        A: [isize; M],
        S: [Chars; N],
    }

    let mut sorted_A = A.clone();
    sorted_A.sort();

    let mut scores = vec![0; N];
    let mut gained_point_maps = vec![HashMap::new(); N];
    let mut max_score = 0;
    for i in 0..N {
        for j in 0..M {
            if S[i][j] == 'o' {
                scores[i] += A[j];
                gained_point_maps[i]
                    .entry(A[j])
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }

        scores[i] += i as isize + 1;

        if max_score < scores[i] {
            max_score = scores[i];
        }
    }

    for i in 0..N {
        if scores[i] == max_score {
            println!("0");
            continue;
        }

        let mut short_points = max_score - scores[i];
        let mut n_questions = 0;
        for a in sorted_A.iter().rev() {
            if *gained_point_maps[i].get(a).unwrap_or(&0) > 0 {
                gained_point_maps[i].entry(*a).and_modify(|n| *n -= 1);
                continue;
            }

            short_points -= a;
            n_questions += 1;
            if short_points < 0 {
                println!("{n_questions}");
                break;
            }
        }
    }
}
