#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(while_true)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    for i in 0..N {
        for j in 0..N {
            if i == j || A[i] + A[j] >= 1000 {
                continue;
            }

            for k in 0..N {
                if i == k || j == k || A[i] + A[j] + A[k] != 1000 {
                    continue;
                }

                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
