#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::collections::HashSet;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        N: usize,
        ABCD: [(usize, usize, usize, usize); N],
    }

    const L: usize = 1500;

    let mut cumulative_diffs: Vec<Vec<isize>> = vec![vec![0; L + 2]; L + 2];
    for abcd in ABCD.iter() {
        let a = abcd.0 + 1;
        let b = abcd.1 + 1;
        let c = abcd.2 + 1;
        let d = abcd.3 + 1;

        cumulative_diffs[a][b] += 1;
        cumulative_diffs[a][d] -= 1;
        cumulative_diffs[c][b] -= 1;
        cumulative_diffs[c][d] += 1;
    }

    for i in 1..=L + 1 {
        for j in 1..=L + 1 {
            cumulative_diffs[i][j] += cumulative_diffs[i][j - 1];
        }
    }
    for j in 1..=L + 1 {
        for i in 1..=L + 1 {
            cumulative_diffs[i][j] += cumulative_diffs[i - 1][j];
        }
    }

    let mut result: usize = 0;
    cumulative_diffs[1..=L + 1]
        .iter()
        .for_each(|row| result += row[1..=L + 1].iter().filter(|&&c| c > 0).count());

    println!("{result}");
}
