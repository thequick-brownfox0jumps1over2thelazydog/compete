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

#[derive(Debug)]
struct SegmentTree {
    bottom_length: usize,
    initial_value: usize,
    values: Vec<usize>,
}
impl SegmentTree {
    fn new(n_items: usize, initial_value: usize) -> Self {
        let bottom_length = if n_items.count_ones() == 1 {
            n_items
        } else {
            1 << (64 - n_items.leading_zeros())
        };
        Self {
            bottom_length,
            initial_value,
            values: vec![initial_value; bottom_length << 1],
        }
    }
    fn update(&mut self, index: usize, value: usize) {
        let i = self.bottom_length + index;
        self.values[i] = value;
        self.apply(i >> 1)
    }
    fn apply(&mut self, index: usize) {
        let child_index = index << 1;
        self.values[index] = self.rule(self.values[child_index], self.values[child_index + 1]);
        if index > 1 {
            self.apply(index >> 1)
        }
    }
    fn query(&self, left: usize, right: usize) -> usize {
        fn _query(
            tree: &SegmentTree,
            left: usize,
            right: usize,
            search_from: usize,
            search_to: usize,
            search_index: usize,
        ) -> usize {
            if right < search_from || search_to < left {
                return tree.initial_value;
            }
            if left <= search_from && search_to <= right {
                return tree.values[search_index];
            }
            let search_midium = (search_from + search_to) >> 1;
            let next_search_index = search_index << 1;
            let left_value = _query(
                tree,
                left,
                right,
                search_from,
                search_midium,
                next_search_index,
            );
            let right_value = _query(
                tree,
                left,
                right,
                search_midium + 1,
                search_to,
                next_search_index + 1,
            );
            tree.rule(left_value, right_value)
        }
        _query(self, left, right, 0, self.bottom_length - 1, 1)
    }
    fn rule(&self, a: usize, b: usize) -> usize {
        a.min(b)
    }
}

#[fastout]
fn solve() {
    input! {
        N: usize,
        L: usize,
        R: usize,
        X: [isize; N],
    }
    debug!(N, L, R, X);

    let mut tree = SegmentTree::new(N, N + 1);
    tree.update(0, 0);

    let mut dp = vec![N + 1; N];
    dp[0] = 0;

    for i in 1..N {
        let mut left_binary_search: BinarySearch<isize> = BinarySearch {
            data: &X,
            right: N as isize - 1,
            target: X[i] - R as isize,
            ..BinarySearch::default()
        };
        let left_index = left_binary_search.lower_bound() as usize;

        let mut right_binary_search: BinarySearch<isize> = BinarySearch {
            data: &X,
            right: N as isize - 1,
            target: X[i] - L as isize,
            ..BinarySearch::default()
        };
        let right_index = 1_usize.max(right_binary_search.upper_bound() as usize) - 1;

        dp[i] = tree.query(left_index, right_index) + 1;
        tree.update(i, dp[i]);
    }

    println!("{}", dp[N - 1]);
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
