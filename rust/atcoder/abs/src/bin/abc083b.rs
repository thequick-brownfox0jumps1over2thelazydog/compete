#![allow(non_snake_case)]
#![allow(unused_imports)]

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: u32,
        A: u32,
        B: u32,
    }

    let mut result: u32 = 0;

    for n in 1..=N {
        let sum_of_digits = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid number."))
            .sum::<u32>();
        if A <= sum_of_digits && sum_of_digits <= B {
            result += n;
        }
    }

    println!("{result}");
}
