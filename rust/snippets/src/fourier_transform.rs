#![allow(dead_code)]

use std::f64::consts::PI;

use cargo_snippet::snippet;
use num::complex::Complex;

#[snippet("fourier_transform")]
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

#[snippet("fourier_transform")]
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

#[snippet("fourier_transform")]
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
