#![allow(dead_code)]
#![allow(unused_variables)]

use cargo_snippet::snippet;

#[snippet("integer")]
fn prime_factorize(number: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if number <= 1 {
        return result;
    }
    let square_root = (number as f64).sqrt() as usize;
    let mut n = number;
    for i in 2..=square_root {
        if n % i == 0 {
            let mut count = 0;
            while n % i == 0 {
                n /= i;
                count += 1;
            }
            result.push((i, count));
        }
    }
    if n != 1 {
        result.push((number, 1));
    }
    result
}
