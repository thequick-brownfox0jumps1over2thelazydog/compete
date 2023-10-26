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
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    iter::FromIterator,
    thread::Builder,
};

use num::integer::{gcd, lcm};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
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

#[fastout]
fn solve() {
    input! {
        N: usize,
        Q: usize,
        S: Chars,
        ABCD: [(Usize1, Usize1, Usize1, Usize1); Q],
    }
    debug!(N, Q, S, ABCD);

    const RADIX: usize = 100;

    let mut hashes = vec![0; N + 1];
    for i in 0..N {
        let x = S[i].to_digit(36).unwrap() as usize;
        hashes[i + 1] = modulo_sum(
            x,
            modulo_product(hashes[i], RADIX, UPPER_LARGE_PRIME),
            UPPER_LARGE_PRIME,
        );
    }
    debug!(hashes);

    for (a, b, c, d) in ABCD.into_iter() {
        let factor = modulo_power(RADIX, b - a + 1, UPPER_LARGE_PRIME);
        let ab = (hashes[b + 1] + UPPER_LARGE_PRIME
            - modulo_product(hashes[a], factor, UPPER_LARGE_PRIME))
            % UPPER_LARGE_PRIME;
        let cd = (hashes[d + 1] + UPPER_LARGE_PRIME
            - modulo_product(hashes[c], factor, UPPER_LARGE_PRIME))
            % UPPER_LARGE_PRIME;

        println!("{}", if ab == cd { "Yes" } else { "No" });
    }
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
