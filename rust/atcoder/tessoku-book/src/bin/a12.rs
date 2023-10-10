#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{cmp::Ordering, collections::HashSet};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

struct BinarySearch<'a, T> {
    // 元データ
    data: &'a [isize],
    left: isize,
    right: isize,
    // 目標
    target: isize,
    strict_equal: bool,
    // その他
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
        self.data
            .iter()
            .map(|a| query / a)
            .sum::<isize>()
            .cmp(&self.target)
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }

    /**
    let mut left = 1;
    let mut right = 10_isize.pow(9);

    while right - left > 1 {
        let midium = (left + right) / 2;
        if check(&A, midium, K) {
            right = midium;
        } else {
            left = midium;
        };
    }

    println!("{right}");
    */
    let mut binary_search: BinarySearch<isize> = BinarySearch {
        data: &A,
        left: 1,
        right: 10_isize.pow(9),
        target: K,
        ..BinarySearch::default()
    };
    println!("{}", binary_search.lower_bound());
}
