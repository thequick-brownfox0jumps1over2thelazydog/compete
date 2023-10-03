#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::collections::HashSet;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        XY: [(usize, usize); N],
        Q: usize,
        ABCD: [(usize, usize, usize, usize); Q],
    }

    const L: usize = 1500;

    let mut cumulative_counts: Vec<Vec<usize>> = vec![vec![0; L + 1]; L + 1];
    for xy in XY.iter() {
        cumulative_counts[xy.0][xy.1] += 1;
    }

    for i in 1..=L {
        for j in 1..=L {
            cumulative_counts[i][j] += cumulative_counts[i][j - 1]
        }
    }
    for j in 1..=L {
        for i in 1..=L {
            cumulative_counts[i][j] += cumulative_counts[i - 1][j];
        }
    }

    for abcd in ABCD.iter() {
        let a = abcd.0;
        let b = abcd.1;
        let c = abcd.2;
        let d = abcd.3;

        let x00 = cumulative_counts[a - 1][b - 1];
        let x01 = cumulative_counts[c][b - 1];
        let x10 = cumulative_counts[a - 1][d];
        let x11 = cumulative_counts[c][d];

        println!("{}", x11 - x01 - x10 + x00);
    }
}
