#![allow(dead_code)]
#![allow(unused_variables)]

use cargo_snippet::snippet;

#[snippet("modulo")]
fn modulo_sum(a: usize, b: usize, modulo: usize) -> usize {
    (a % modulo + b % modulo) % modulo
}

#[snippet("modulo")]
fn modulo_product(a: usize, b: usize, modulo: usize) -> usize {
    (a % modulo) * (b % modulo) % modulo
}

#[snippet("modulo")]
fn modulo_factorial(left: usize, right: usize, modulo: usize) -> usize {
    let mut result = 1;
    for i in left..=right {
        result = modulo_product(result, i, modulo);
    }

    result
}

#[snippet("modulo")]
fn modulo_power(number: usize, exponent: usize, modulo: usize) -> usize {
    let n = number % modulo;

    let digits = format!("{:b}", exponent).len();
    let mut factor = 1;
    let mut result = 1;
    for i in 0..digits {
        factor = factor * if factor == 1 { n } else { factor } % modulo;
        if exponent & 1 << i > 0 {
            result = result * factor % modulo;
        }
    }

    result
}

#[snippet("modulo")]
fn modulo_divide(numerator: usize, denominator: usize, modulo: usize) -> usize {
    let inverse = modulo_power(denominator, modulo - 2, modulo);

    modulo_product(numerator, inverse, modulo)
}
