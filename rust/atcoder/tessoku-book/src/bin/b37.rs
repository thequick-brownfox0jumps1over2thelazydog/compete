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
    fmt::Debug,
    iter::FromIterator,
    thread::Builder,
};

use num::integer::{gcd, lcm};
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

#[fastout]
fn solve() {
    input! {
        mut N: usize,
    }
    debug!(N);

    let mut result = 0;

    let mut i = 1;
    while true {
        let divider = 10_usize.pow(i);
        let quotient = N / divider;
        let remainder = N % divider;

        result += 45_usize * quotient * divider / 10;
        debug!(i, result, divider, quotient, remainder);

        if remainder > 0 {
            let divider_of_remainder = 10_usize.pow(i - 1);
            let quotient_of_remainder = (remainder + 1) / divider_of_remainder;
            let remainder_of_remainder = (remainder + 1) % divider_of_remainder;
            result += quotient_of_remainder * (quotient_of_remainder - 1) * divider_of_remainder
                / 2
                + quotient_of_remainder * remainder_of_remainder;
            debug!(
                i,
                result, divider_of_remainder, quotient_of_remainder, remainder_of_remainder
            );
        }

        if quotient == 0 {
            break;
        }

        i += 1;
    }

    println!("{result}");
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
