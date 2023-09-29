#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: isize,
        A: isize,
        B: isize,
    }

    let mut result: isize = 0;

    for n in 1..=N {
        let sum_of_digits = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid number."))
            .sum::<u32>();
        if A <= sum_of_digits as isize && sum_of_digits as isize <= B {
            result += n;
        }
    }

    println!("{result}");
}
