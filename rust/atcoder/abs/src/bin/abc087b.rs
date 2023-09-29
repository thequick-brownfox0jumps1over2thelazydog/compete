#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
        X: isize,
    }

    let mut result: isize = 0;

    for i in (0..=A).rev() {
        let a = i * 500;
        if B * 100 + C * 50 < X - a {
            break;
        }

        for j in (0..=B).rev() {
            let b = j * 100;
            if C * 50 < X - a - b {
                break;
            }

            for k in (0..=C).rev() {
                if a + b + k * 50 == X {
                    result += 1;
                    break;
                }
            }
        }
    }

    println!("{result}");
}
