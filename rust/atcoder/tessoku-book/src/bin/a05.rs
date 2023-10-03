#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: isize,
        K: isize,
    }

    let mut result = 0;

    for i in 1..=N {
        if i > K - 2 {
            break;
        }
        if i < K - N * 2 {
            continue;
        }

        for j in 1..=N {
            if j > K - i - 1 {
                break;
            }
            if j < K - i - N {
                continue;
            }

            result += 1;
        }
    }

    println!("{result}");
}
