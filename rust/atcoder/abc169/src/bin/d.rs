#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
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

fn prime_factorize(number: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if number <= 1 {
        return result;
    }
    let square_root = (number as f64).sqrt() as usize;
    let mut n = number;
    for i in 2..=square_root {
        if n % i == 0 {
            let mut count = 0;
            while n % i == 0 {
                n /= i;
                count += 1;
            }
            result.push((i, count));
        }
    }
    if n != 1 {
        result.push((number, 1));
    }
    result
}

#[derive(Default)]
struct Solver {}

impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
        }

        let mut prime_factors = prime_factorize(N);

        let mut result = 0;
        for i in 0..prime_factors.len() {
            let mut j = 1;
            while prime_factors[i].1 >= j {
                prime_factors[i].1 -= j;
                j += 1;
                result += 1;
            }
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
