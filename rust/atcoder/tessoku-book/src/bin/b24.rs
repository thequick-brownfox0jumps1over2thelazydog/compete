#![allow(dead_code)]
#![allow(non_snake_case)]
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

use std::cmp::Ordering;
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
    fn step_function(&self, query: isize) -> Ordering {
        self.data[query as usize].cmp(&self.target)
    }
}

#[derive(Default)]
struct Solver {}

impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            mut XY: [(isize, isize); N],
        }

        XY.sort_unstable_by_key(|&(x, y)| (x, -y));
        debug!(XY);

        let mut dp = vec![1; N];
        let mut L = vec![XY[0].1];

        for i in 1..N {
            let y = XY[i].1;

            let mut binary_search: BinarySearch<isize> = BinarySearch {
                data: &L,
                right: L.len() as isize,
                target: y,
                ..BinarySearch::default()
            };
            let index = binary_search.lower_bound() as usize;

            dp[i] = index + 1;

            if L.len() < dp[i] {
                L.push(y);
                continue;
            }

            let index = dp[i] - 1;
            L[index] = min(L[index], y);
        }

        println!("{}", L.len());
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
