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
            X: usize,
            Y: usize,
            mut A: [usize; N],
        }
        debug!(N, X, Y, A);

        A.sort();

        let A_len = A.len();
        let mut grundy_numbers = vec![0; A[A_len - 1] + 1];
        for i in 0..=A[A_len - 1] {
            if i < X {
                grundy_numbers[i] = 0;
                continue;
            }

            let mut gs = vec![grundy_numbers[i - X]];
            if i >= Y {
                gs.push(grundy_numbers[i - Y]);
            }

            let mut j = 0;
            while gs.contains(&j) {
                j += 1;
            }

            grundy_numbers[i] = j;
        }
        debug!(grundy_numbers);

        let grundy_xor = A
            .iter()
            .enumerate()
            .fold(0, |a, (i, b)| a ^ grundy_numbers[A[i]]);
        debug!(grundy_xor);

        println!("{}", if grundy_xor > 0 { "First" } else { "Second" });
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
