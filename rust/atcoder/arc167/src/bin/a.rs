#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{
    collections::{HashMap, HashSet},
    thread::Builder,
};

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
            M: usize,
            mut A: [usize; N],
        }

        A.sort();
        let double = N - M;

        let mut result = 0;
        for i in 0..N - M {
            result += (A[i] + A[(N - M) * 2 - i - 1]).pow(2);
            debug!(A[i], A[(N - M) * 2 - i - 1], result);
        }
        for j in (N - M) * 2..N {
            result += A[j].pow(2);
            debug!(A[j], result);
        }

        println!("{result}");
    }
}

fn main() {
    Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
