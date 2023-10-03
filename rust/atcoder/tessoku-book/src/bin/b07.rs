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
        LR: [(usize, usize); N]
    }

    let mut staff_diffs: Vec<isize> = vec![0; T];
    for lr in LR.iter() {
        staff_diffs[lr.0] += 1;
        if lr.1 < T {
            staff_diffs[lr.1] -= 1;
        }
    }

    let mut previous_staffs: isize = 0;
    for sd in staff_diffs.iter() {
        previous_staffs += sd;
        println!("{previous_staffs}");
    }
}
