#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]

use std::{
    collections::{HashMap, HashSet},
    thread::Builder,
};

use libm::exp;
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

#[derive(Default)]
struct Solver {}

impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            A: usize,
            B: usize,
        }

        let mut a = vec![];
        let mut temp = A;
        let square = (A as f64).sqrt() as usize;
        let mut count = 0;

        for i in 2..=square {
            if temp % i == 0 {
                count = 0;
                while temp % i == 0 {
                    count += 1;
                    temp /= i;
                }
                a.push((i, count));
            }
        }
        if temp != 1 {
            a.push((temp, 1));
        }
        if a.is_empty() {
            a.push((A, 1));
        }

        const MODULO: usize = 998244353;
        const INVERSE: usize = 499122177;
        let mut exponents = a
            .iter()
            .map(|(n, count)| count * B + 1)
            .collect::<Vec<usize>>();

        let mut is_squared = true;
        for i in 0..exponents.len() {
            if exponents[i] == 1 || exponents[i] % 2 == 0 {
                is_squared = false;
                break;
            }
        }

        let mut n_pairs = exponents
            .iter()
            .fold(1, |product, n| (product % MODULO) * (n % MODULO));

        debug!(A, B, a, exponents, n_pairs, is_squared);

        if is_squared {
            n_pairs -= 1;
        }
        let mut result = ((B % MODULO) * (n_pairs % MODULO)) % MODULO;

        result = result * INVERSE % MODULO;
        debug!(result);
        if is_squared {
            result = (result
                + if B % 2 == 0 {
                    (B % MODULO * INVERSE) % MODULO
                } else {
                    (((B - 1) % MODULO) * INVERSE) % MODULO
                })
                % MODULO;
        }

        println!("{result}");
    }
}

fn main() {
    Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
