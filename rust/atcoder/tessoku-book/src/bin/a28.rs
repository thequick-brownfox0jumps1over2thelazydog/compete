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

fn modulo_sum(a: usize, b: usize, modulo: usize) -> usize {
    (a % modulo + b % modulo) % modulo
}
fn modulo_product(a: usize, b: usize, modulo: usize) -> usize {
    (a % modulo) * (b % modulo) % modulo
}
fn modulo_factorial(left: usize, right: usize, modulo: usize) -> usize {
    let mut result = 1;
    for i in left..=right {
        result = modulo_product(result, i, modulo);
    }
    result
}
fn modulo_power(number: usize, exponent: usize, modulo: usize) -> usize {
    let n = number % modulo;
    let digits = format!("{:b}", exponent).len();
    let mut factor = 1;
    let mut result = 1;
    for i in 0..digits {
        factor = factor * if factor == 1 { n } else { factor } % modulo;
        if exponent & 1 << i > 0 {
            result = result * factor % modulo;
        }
    }
    result
}
fn modulo_divide(numerator: usize, denominator: usize, modulo: usize) -> usize {
    let inverse = modulo_power(denominator, modulo - 2, modulo);
    modulo_product(numerator, inverse, modulo)
}

#[derive(Default)]
struct Solver {}

impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            TA: [(char, usize); N],
        }
        debug!(N, TA);

        const FACTOR: usize = 1e4 as usize;
        const FACTOR_isize: isize = FACTOR as isize;

        let mut result = FACTOR;
        for (t, a) in TA.into_iter() {
            match t {
                '+' => result = modulo_sum(result, a, FACTOR),
                '-' => {
                    result = if result < a {
                        result + FACTOR - a
                    } else {
                        result - a
                    }
                }
                '*' => result = modulo_product(result, a, FACTOR),
                _ => unreachable!(),
            }

            println!("{}", result % FACTOR);
        }
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
