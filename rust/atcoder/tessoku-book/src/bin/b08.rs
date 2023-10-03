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
        XY: [(Usize1, Usize1); N],
        Q: usize,
        ABCD: [(Usize1, Usize1, Usize1, Usize1); Q],
    }

    const L: usize = 1500;

    let mut cumulative_counts = vec![vec![0; L + 1]; L + 1];
    for xy in XY.iter() {
        let x = xy.0;
        let y = xy.1;

        cumulative_counts[x + 1][y + 1] += 1;
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

        let x00 = cumulative_counts[a][b];
        let x01 = cumulative_counts[c + 1][b];
        let x10 = cumulative_counts[a][d + 1];
        let x11 = cumulative_counts[c + 1][d + 1];

        println!("{}", x11 - x01 - x10 + x00);
    }
}
