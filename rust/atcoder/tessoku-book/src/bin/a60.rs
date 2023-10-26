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

#[derive(Debug)]
struct StockPrice {
    day: usize,
    price: usize,
}

impl Ord for StockPrice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for StockPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl PartialEq for StockPrice {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for StockPrice {}

#[fastout]
fn solve() {
    input! {
        N: usize,
        A: [usize; N],
    }
    debug!(N, A);

    let mut stacks = vec![StockPrice {
        day: 1,
        price: A[0],
    }];
    let mut result = vec![String::from(""); N];
    result[0] = "-1".to_string();

    for i in 1..N {
        let stock_price = StockPrice {
            day: i + 1,
            price: A[i],
        };
        debug!(i, stacks, stock_price);

        while !stacks.is_empty() && *stacks.last().unwrap() < stock_price {
            stacks.pop();
        }

        result[i] = match stacks.last() {
            Some(sp) => sp.day.to_string(),
            None => "-1".to_string(),
        };
        stacks.push(StockPrice {
            day: i + 1,
            price: A[i],
        });
    }

    println!("{}", result.join(" "));
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
