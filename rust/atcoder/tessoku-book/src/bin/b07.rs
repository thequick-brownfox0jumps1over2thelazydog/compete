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
        T: usize,
        N: usize,
        LR: [(usize, usize); N]  // NOTE: 0始まりなのでUsize1として定義できない
    }

    let mut staff_diffs = vec![0; T];
    for lr in LR.iter() {
        let l = lr.0;
        let r = lr.1;

        staff_diffs[l] += 1;
        if lr.1 < T {
            staff_diffs[r] -= 1;
        }
    }

    let mut previous_staffs = 0;
    for sd in staff_diffs.iter() {
        previous_staffs += sd;
        println!("{previous_staffs}");
    }
}
