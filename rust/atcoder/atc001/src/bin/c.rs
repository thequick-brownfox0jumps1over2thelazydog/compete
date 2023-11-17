#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_attributes)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::precedence)]

use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::consts::PI,
    fmt::Debug,
    iter::FromIterator,
    thread::Builder,
};

use itertools::Itertools;
use num::{
    complex::Complex,
    integer::{gcd, lcm},
};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! debug {
    ($($var:expr),+) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($(stringify!($var), "={:?}  "),+), $(&$var),+);
    };
}

pub fn fast_fourier_transform(mut complex_vector: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let length = complex_vector.len();
    if length.count_ones() != 1 {
        panic!("the length of vector should be a power of two");
    }
    let n_bits = length.trailing_zeros() as usize;
    for shift in (0..n_bits).rev() {
        let half = 1 << shift;
        let step = half << 1;
        let zeta = Complex::from_polar(1.0, 2.0 * PI / step as f64);
        for j in 0..length / step {
            let left = j * step;
            let mut zi = Complex::new(1.0, 0.0);
            for i in 0..half {
                let sum = complex_vector[left + i] + complex_vector[left + i + half];
                let diff = complex_vector[left + i] - complex_vector[left + i + half];
                complex_vector[left + i] = sum;
                complex_vector[left + i + half] = diff * zi;
                zi *= zeta;
            }
        }
    }
    complex_vector
}
pub fn inverse_fast_fourier_transform(mut complex_vector: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let length = complex_vector.len();
    if length.count_ones() != 1 {
        panic!("the length of vector should be a power of two");
    }
    let n_bits = length.trailing_zeros() as usize;
    for shift in 0..n_bits {
        let half = 1 << shift;
        let step = half << 1;
        let zeta = Complex::from_polar(1.0, -2.0 * PI / step as f64);
        for j in 0..length / step {
            let left = j * step;
            let mut zi = Complex::new(1.0, 0.0);
            for i in 0..half {
                let sum = complex_vector[left + i] + complex_vector[left + i + half] * zi;
                let diff = complex_vector[left + i] - complex_vector[left + i + half] * zi;
                complex_vector[left + i] = sum;
                complex_vector[left + i + half] = diff;
                zi *= zeta;
            }
        }
    }
    let factor = Complex::new(length as f64, 0.0);
    complex_vector
        .iter()
        .map(|&z| z / factor)
        .collect::<Vec<Complex<f64>>>()
}
fn convolve(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let a_length = a.len();
    let b_length = b.len();
    let c_length = a_length + b_length - 1;
    let c_length_extended = c_length.next_power_of_two();
    let zero = Complex::new(0.0, 0.0);
    let mut a_extended = vec![zero; c_length_extended];
    for i in 0..a_length {
        a_extended[i] = Complex::new(a[i], 0.0);
    }
    a_extended = fast_fourier_transform(a_extended);
    let mut b_extended = vec![zero; c_length_extended];
    for j in 0..b_length {
        b_extended[j] = Complex::new(b[j], 0.0);
    }
    b_extended = fast_fourier_transform(b_extended);
    let mut c = (0..c_length_extended)
        .map(|l| a_extended[l] * b_extended[l])
        .collect::<Vec<Complex<f64>>>();
    c = inverse_fast_fourier_transform(c);
    (0..c_length).map(|k| c[k].re).collect::<Vec<f64>>()
}

#[fastout]
fn solve() {
    input! {
        N: usize,
        AB: [(usize, usize); N],
    }

    // practice
    /**
    for k in 1..=N * 2 {
        let mut result = 0;

        for i in 1..=N {
            if i + 1 > k {
                break;
            }
            if i + N < k {
                continue;
            }

            result += AB[i - 1].0 * AB[k - i - 1].1;
        }

        println!("{}", result);
    }
    */
    let mut A = vec![0.0; N + 1];
    let mut B = vec![0.0; N + 1];
    for i in 0..N {
        let (a, b) = AB[i];
        A[i + 1] = a as f64;
        B[i + 1] = b as f64;
    }

    let c = convolve(&A, &B);
    for i in 1..=N * 2 {
        println!("{}", c[i].round() as usize);
    }
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
