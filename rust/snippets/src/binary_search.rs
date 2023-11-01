#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::Ordering;

use cargo_snippet::snippet;

#[snippet("binary_search", prefix = "use std::cmp::Ordering;")]
/**
 *  e.g.
 *  ```
 *  let mut binary_search: BinarySearch<isize> = BinarySearch {
 *      data: &[],
 *      right: 10_isize.pow(9),
 *      target: 0,
 *      strict_equal: true,
 *      ..BinarySearch::default(),
 *  };
 *  let index = binary_search.lower_bound();
 *  ```
 */
pub struct BinarySearch<'a, T> {
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

#[snippet("binary_search")]
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

#[snippet("binary_search")]
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

#[snippet("binary_search")]
impl<'a, T> BinarySearch<'a, T> {
    /**
     * e.g.
     * ```
     * self.data[query as usize].cmp(&self.target)
     * ```
     */
    fn step_function(&self, query: isize) -> Ordering {
        todo!()
    }
}
