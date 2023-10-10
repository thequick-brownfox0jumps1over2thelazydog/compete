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
        self.data[query as usize].cmp(&self.target)
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
    }

    let left_power = N / 2;
    let right_power = N - left_power;

    let left = 2_isize.pow(left_power as u32);
    let right = 2_isize.pow(right_power as u32);

    let mut L: HashSet<isize> = HashSet::new();
    for i in 0..left {
        let mut left_total = 0;
        for (k, c) in format!("{:0digits$b}", i, digits = left_power)
            .chars()
            .enumerate()
        {
            if c == '1' {
                left_total += A[k]
            }
        }

        L.insert(left_total);
    }

    let mut R: HashSet<isize> = HashSet::new();
    for j in 0..right {
        let mut right_total = 0;
        for (m, c) in format!("{:0digits$b}", j, digits = right_power)
            .chars()
            .enumerate()
        {
            if c == '1' {
                right_total += A[left_power + m]
            }
        }

        R.insert(right_total);
    }

    let mut Rv: Vec<_> = R.into_iter().collect();
    let R_len = Rv.len() as isize;
    Rv.sort();

    for l in L.iter() {
        let mut binary_search: BinarySearch<Vec<isize>> = BinarySearch {
            data: &Rv,
            right: R_len,
            target: K - l,
            strict_equal: true,
            ..BinarySearch::default()
        };
        if binary_search.lower_bound() >= 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
