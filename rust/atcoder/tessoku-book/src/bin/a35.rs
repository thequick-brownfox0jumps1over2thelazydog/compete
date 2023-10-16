#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_attributes)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::precedence)]

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    thread::Builder,
};

use num::integer::{gcd, lcm};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! debug {
    ($($var:expr),+) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($(stringify!($var), "={:?}  "),+), $(&$var),+);
    };
}

#[derive(Default)]
struct Solver {}

impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            A: [usize; N],
        }
        debug!(N, A);

        let mut expected_scores = vec![vec![]; N];
        expected_scores[N - 1] = A.clone();

        for i in (0..N - 1).rev() {
            let last_row = expected_scores[i + 1].clone();
            debug!(i, 0..i, last_row);
            for j in 0..i + 1 {
                expected_scores[i].push(if i % 2 == 0 {
                    max(last_row[j], last_row[j + 1])
                } else {
                    min(last_row[j], last_row[j + 1])
                });
            }
        }
        debug!(expected_scores);

        let mut j = 0;
        for i in 0..N - 1 {
            if i % 2 == 0 {
                if expected_scores[i + 1][j] < expected_scores[i + 1][j + 1] {
                    j += 1;
                }
                continue;
            }

            if expected_scores[i + 1][j] > expected_scores[i + 1][j + 1] {
                j += 1;
            }
        }

        println!("{}", A[j]);
    }
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
