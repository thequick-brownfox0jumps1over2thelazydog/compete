#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        H: usize,
        W: usize,
        X: [[isize; W]; H],
        Q: usize,
        ABCD: [(Usize1, Usize1, Usize1, Usize1); Q],
    }

    let mut cumulative_sums: Vec<Vec<isize>> = vec![vec![0; W + 1]; H + 1];
    for i in 1..=H {
        for j in 1..=W {
            cumulative_sums[i][j] = cumulative_sums[i][j - 1] + X[i - 1][j - 1];
        }
    }
    for j in 1..=W {
        for i in 1..=H {
            cumulative_sums[i][j] += cumulative_sums[i - 1][j];
        }
    }

    for abcd in ABCD.iter() {
        let a = abcd.0;
        let b = abcd.1;
        let c = abcd.2;
        let d = abcd.3;

        let x00 = cumulative_sums[a][b];
        let x01 = cumulative_sums[a][d + 1];
        let x10 = cumulative_sums[c + 1][b];
        let x11 = cumulative_sums[c + 1][d + 1];

        println!("{}", x11 - x01 - x10 + x00);
    }
}
