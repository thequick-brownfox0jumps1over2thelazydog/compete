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
        a + b
    }
}

#[fastout]
fn solve() {
    input! {
        N: usize,
        P: [[usize; N]; N],
    }
    debug!(N, P);

    let mut xs = vec![0; N];
    for j in 0..N {
        for i in 0..N {
            if P[i][j] > 0 {
                xs[j] = P[i][j];
                break;
            }
        }
    }
    debug!(xs);

    let mut x_times = 0;
    let mut x_tree = SegmentTree::new(N, 0);
    for &x in xs.iter() {
        x_times += x_tree.query(x, N - 1);
        x_tree.update(x - 1, 1);
    }

    let mut ys = vec![0; N];
    for i in 0..N {
        for j in 0..N {
            if P[i][j] > 0 {
                ys[i] = P[i][j];
                break;
            }
        }
    }
    debug!(ys);

    let mut y_times = 0;
    let mut y_tree = SegmentTree::new(N, 0);
    for &y in ys.iter() {
        y_times += y_tree.query(y, N - 1);
        y_tree.update(y - 1, 1);
    }

    println!("{}", x_times + y_times);
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
