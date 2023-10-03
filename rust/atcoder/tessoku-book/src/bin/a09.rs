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
        H: usize,
        W: usize,
        N: usize,
        ABCD: [(Usize1, Usize1, Usize1, Usize1); N],
    }

    let mut cumulative_diffs = vec![vec![0; W + 2]; H + 2];

    for abcd in ABCD.iter() {
        let a = abcd.0 + 1;
        let b = abcd.1 + 1;
        let c = abcd.2 + 1;
        let d = abcd.3 + 1;

        cumulative_diffs[a][b] += 1;
        cumulative_diffs[a][d + 1] -= 1;
        cumulative_diffs[c + 1][b] -= 1;
        cumulative_diffs[c + 1][d + 1] += 1;
    }

    for j in 1..=W {
        for i in 1..=H {
            cumulative_diffs[i][j] += cumulative_diffs[i - 1][j];
        }
    }
    for i in 1..=H {
        for j in 1..=W {
            cumulative_diffs[i][j] += cumulative_diffs[i][j - 1];
        }
    }

    for i in 1..=H {
        let mut result = Vec::new();
        for cd in cumulative_diffs[i][1..=W].iter() {
            result.push(cd.to_string());
        }

        println!("{}", result.join(" "));
    }
}
