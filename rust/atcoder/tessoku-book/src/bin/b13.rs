#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
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
        todo!()
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }

    let mut cumulative_sums = vec![0; N + 1];
    for i in 1..=N {
        cumulative_sums[i] = cumulative_sums[i - 1] + A[i - 1];
    }

    let mut result = 0;
    let mut index = 0;
    for i in 0..N {
        for j in index.max(i + 1)..=N {
            if cumulative_sums[j] - cumulative_sums[i] > K {
                result += j - i - 1;
                index = j;
                break;
            }
            if j == N {
                result += j - i;
                index = j;
                break;
            }
        }
    }

    println!("{result}");
}
