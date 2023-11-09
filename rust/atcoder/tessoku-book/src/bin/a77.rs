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
        L: usize,
        K: usize,
        A: [usize; N],
    }
    debug!(N, L, K, A);

    let mut parts = vec![A[0]; N + 1];
    for i in 1..N {
        parts[i] = A[i] - A[i - 1];
    }
    parts[N] = L - A[N - 1];

    let min_part = *parts.iter().min().unwrap();

    let mut cumulative_length = vec![0; N + 2];
    for i in 0..=N {
        cumulative_length[i + 1] = cumulative_length[i] + parts[i] as isize;
    }
    debug!(parts, cumulative_length, min_part);

    let mut left = min_part + 1;
    let mut right = L / (K + 1) + 1;

    while left < right {
        let mut updated_flag = false;
        let midium = (left + right) >> 1;
        debug!(left, midium, right);

        let mut index = 0;
        for k in 0..K {
            let mut binary_search: BinarySearch<isize> = BinarySearch {
                data: &cumulative_length,
                left: index as isize + 1,
                right: N as isize + 1,
                target: cumulative_length[index] + midium as isize,
                strict_equal: false,
                ..BinarySearch::default()
            };
            index = binary_search.lower_bound() as usize;

            if index > N + 1
                || (k == K - 1
                    && cumulative_length[N + 1] - cumulative_length[index] < midium as isize)
            {
                right = midium;
                updated_flag = true;
                break;
            }
        }

        if !updated_flag {
            left = midium + 1;
        }
    }

    println!("{}", left - 1);
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
