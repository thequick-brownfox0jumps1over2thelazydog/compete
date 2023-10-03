#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        TXY: [(isize, isize, isize); N],
    }

    for i in 0..N {
        let t = TXY[i].0;
        let x = TXY[i].1;
        let y = TXY[i].2;

        let mut prev_t = 0;
        let mut prev_x = 0;
        let mut prev_y = 0;
        if i > 0 {
            prev_t = TXY[i - 1].0;
            prev_x = TXY[i - 1].1;
            prev_y = TXY[i - 1].2;
        }

        let dt = t - prev_t;
        let dx = (x - prev_x).abs();
        let dy = (y - prev_y).abs();

        if dx + dy > dt || (dx + dy - dt) % 2 != 0 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
