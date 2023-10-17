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
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
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
        R: usize,
        C: usize,
        SY: Usize1,
        SX: Usize1,
        GY: Usize1,
        GX: Usize1,
        YX: [Chars; R],
    }
    debug!(R, C, (SY, SX), (GY, GX), YX);

    let mut distances = vec![vec![-1; C]; R];
    distances[SY][SX] = 0;
    let mut distance = 1;
    let mut queues = vec![(SY, SX)];

    while !queues.is_empty() {
        let mut next_queues = vec![];
        for (y, x) in queues.into_iter() {
            let mut next_points = vec![];
            if y > 0 {
                next_points.push((y - 1, x));
            }
            if y < R - 1 {
                next_points.push((y + 1, x));
            }
            if x > 0 {
                next_points.push((y, x - 1));
            }
            if x < C - 1 {
                next_points.push((y, x + 1));
            }
            for (next_y, next_x) in next_points.into_iter() {
                if YX[next_y][next_x] == '#' || distances[next_y][next_x] >= 0 {
                    continue;
                }
                if next_y == GY && next_x == GX {
                    println!("{}", distance);
                    return;
                }

                distances[next_y][next_x] = distance;
                next_queues.push((next_y, next_x));
            }
        }

        distance += 1;
        queues = next_queues;
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
