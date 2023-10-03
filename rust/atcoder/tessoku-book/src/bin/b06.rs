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
        N: usize,
        A: [isize; N],
        Q: usize,
        LR: [(Usize1, Usize1); Q],
    }

    let mut cumulative_wins: Vec<isize> = vec![0; N + 1];
    for i in 1..=N {
        cumulative_wins[i] = cumulative_wins[i - 1] + A[i - 1];
    }

    for j in 0..Q {
        let win = cumulative_wins[LR[j].1 + 1] - cumulative_wins[LR[j].0];
        let lotteries = LR[j].1 - LR[j].0 + 1;
        let result = match win * 2 - lotteries as isize {
            1.. => "win",
            0 => "draw",
            _ => "lose", // ..0 => "lose" : exclusive range pattern syntax is experimental. see https://github.com/rust-lang/rust/issues/37854
        };
        println!("{result}");
    }
}
