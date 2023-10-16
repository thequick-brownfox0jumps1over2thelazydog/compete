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

const UPPER_LARGE_PRIME: usize = 1e9 as usize + 7;
const LOWER_LARGE_PRIME: usize = 998_244_353;

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
            A: usize,
            mut B: usize,
        }

        if B == 0 {
            println!("0");
            return;
        }

        let B_is_odd = B % 2 == 1;
        let B_half = B / 2;

        let prime_factors = prime_factorize(A);
        debug!(prime_factors);
        let exponent_options = prime_factors
            .iter()
            .map(|(_, c)| {
                modulo_sum(
                    modulo_product(*c, B, LOWER_LARGE_PRIME),
                    1,
                    LOWER_LARGE_PRIME,
                )
            })
            .collect::<Vec<usize>>();
        debug!(exponent_options);

        let mut is_squared = true;
        for i in 0..exponent_options.len() {
            if prime_factors[i].1 % 2 == 1 && B_is_odd {
                is_squared = false;
                break;
            }
        }
        is_squared = is_squared && B_is_odd;

        let n_divisors = exponent_options.iter().fold(1, |cumulative_product, &e| {
            modulo_product(cumulative_product, e, LOWER_LARGE_PRIME)
        });
        debug!(n_divisors);
        debug!(A, B, is_squared);

        let mut result = modulo_product(
            if is_squared {
                n_divisors + LOWER_LARGE_PRIME - 1
            } else {
                n_divisors
            },
            B,
            LOWER_LARGE_PRIME,
        );
        result = modulo_divide(result, 2, LOWER_LARGE_PRIME);

        if is_squared {
            result = modulo_sum(result, B_half, LOWER_LARGE_PRIME);
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
