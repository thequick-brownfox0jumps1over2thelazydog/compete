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

use itertools::Itertools;
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

///
/// *  e.g.
/// *  ```
/// *  let mut binary_search: BinarySearch<isize> = BinarySearch {
/// *      data: &[],
/// *      right: 10_isize.pow(9),
/// *      target: 0,
/// *      strict_equal: true,
/// *      ..BinarySearch::default(),
/// *  };
/// *  let index = binary_search.lower_bound();
/// *  ```
///
pub struct BinarySearch<'a, T> {
    data: &'a [isize],
    left: isize,
    right: isize,
    target: isize,
    strict_equal: bool,
    args: T,
}
impl<'a, T> BinarySearch<'a, T> {
    fn is_searchable(&self) -> bool {
        self.left < self.right
    }
    fn midium(&self) -> isize {
        (self.left + self.right) / 2
    }
    fn lower_bound(&mut self) -> isize {
        while self.is_searchable() {
            let midium = self.midium();
            match self.step_function(midium) {
                Ordering::Less => self.left = midium + 1,
                Ordering::Equal => {
                    if self.strict_equal {
                        return midium;
                    } else {
                        self.right = midium
                    }
                }
                _ => self.right = midium,
            }
        }
        if self.strict_equal {
            -1
        } else {
            self.left
        }
    }
    fn upper_bound(&mut self) -> isize {
        while self.is_searchable() {
            let midium = self.midium();
            match self.step_function(midium) {
                Ordering::Less => self.left = midium + 1,
                Ordering::Equal => {
                    if self.strict_equal {
                        return midium;
                    } else {
                        self.left = midium + 1
                    }
                }
                _ => self.right = midium,
            }
        }
        if self.strict_equal {
            -1
        } else {
            self.left
        }
    }
}
impl<'a, T> Default for BinarySearch<'a, T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: &[],
            left: 0,
            right: 10_isize.pow(9),
            target: 0,
            strict_equal: false,
            args: T::default(),
        }
    }
}
impl<'a, T> BinarySearch<'a, T> {
    ///
    ///     * e.g.
    ///     * ```
    ///     * self.data[query as usize].cmp(&self.target)
    ///     * ```
    ///
    fn step_function(&self, query: isize) -> Ordering {
        self.data[query as usize].cmp(&self.target)
    }
}

#[fastout]
fn solve() {
    input! {
        N: usize,
        W: usize,
        L: usize,
        R: usize,
        mut X: [isize; N],
    }
    debug!(N, W, L, R, X);

    let N_extended: usize = N + 2;
    X.insert(0, 0);
    X.push(W as isize);
    debug!(X);

    let mut dp = vec![0; N_extended];
    dp[0] = 1;

    let mut cumulative_sum = vec![0; N_extended + 1];
    cumulative_sum[1] = 1;

    for i in 1..N_extended {
        let mut left_binary_search: BinarySearch<isize> = BinarySearch {
            data: &X,
            right: i as isize,
            target: X[i] - R as isize,
            strict_equal: false,
            ..BinarySearch::default()
        };
        let left_index = left_binary_search.lower_bound() as usize;

        let mut right_binary_search: BinarySearch<isize> = BinarySearch {
            data: &X,
            right: i as isize,
            target: X[i] - L as isize,
            strict_equal: false,
            ..BinarySearch::default()
        };
        let right_index = right_binary_search.upper_bound() as usize;

        dp[i] = modulo_sum(
            cumulative_sum[right_index],
            UPPER_LARGE_PRIME - cumulative_sum[left_index],
            UPPER_LARGE_PRIME,
        );

        cumulative_sum[i + 1] = modulo_sum(cumulative_sum[i], dp[i], UPPER_LARGE_PRIME);
        debug!(i, left_index, right_index, dp, cumulative_sum);
    }

    println!("{}", dp[N_extended - 1]);
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
