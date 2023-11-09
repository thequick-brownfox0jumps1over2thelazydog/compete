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

#[fastout]
fn solve() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        C: [Chars; H],
    }
    debug!(H, W, K, C);

    let mut max_n_black_cells = 0;

    for n in 0..2_usize.pow(H as u32) {
        let n_rows = n.count_ones() as usize;
        if n_rows > K {
            continue;
        }

        let mut cloned_C = C.clone();
        for i in 0..H {
            if n >> i & 1 == 0 {
                continue;
            }
            cloned_C[i] = vec!['#'; W];
        }
        let mut n_black_cells = cloned_C
            .iter()
            .map(|row| row.iter().filter(|&&c| c == '#').count())
            .sum::<usize>();
        let n_columns = K - n_rows;
        if n_columns == 0 {
            max_n_black_cells = max(max_n_black_cells, n_black_cells);
            continue;
        }

        let mut n_white_cells_per_column = vec![0; W];
        for j in 0..W {
            for i in 0..H {
                if cloned_C[i][j] == '.' {
                    n_white_cells_per_column[j] += 1;
                }
            }
        }
        n_white_cells_per_column.sort_by(|a, b| b.cmp(a));
        n_black_cells += n_white_cells_per_column[..n_columns].iter().sum::<usize>();
        max_n_black_cells = max(max_n_black_cells, n_black_cells);
        debug!(n, max_n_black_cells);
    }

    println!("{max_n_black_cells}");
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
