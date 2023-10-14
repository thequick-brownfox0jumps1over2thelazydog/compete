#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(while_true)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::precedence)]

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    thread::Builder,
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

#[derive(Default)]
struct Solver {}

impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            XY: [(isize, isize); N],
        }
        debug!(N, XY);

        let factor = 10_f64.powi(4);

        let mut adjacency_matrix = vec![vec![0; N]; N];
        for i in 0..N {
            let (xi, yi) = XY[i];
            for j in i + 1..N {
                let (xj, yj) = XY[j];
                let squared_distance = (xi - xj).pow(2) + (yi - yj).pow(2);
                let distance = ((squared_distance as f64).sqrt() * factor) as isize;

                adjacency_matrix[i][j] = distance;
                adjacency_matrix[j][i] = distance;
            }
        }
        debug!(adjacency_matrix);

        let n_sets = 1 << N;
        let mut dp = vec![vec![-1; N]; n_sets];
        dp[0][0] = 0;
        for reached in 0..n_sets {
            for current in 0..N {
                let c = dp[reached][current];
                if c < 0 {
                    continue;
                }

                for next in 0..N {
                    if current == next || reached & (1 << next) > 0 {
                        continue;
                    }

                    let distance = adjacency_matrix[current][next];
                    let new_reached = reached | 1 << next;
                    let current_min_distance = dp[new_reached][next];

                    dp[new_reached][next] = if current_min_distance >= 0 {
                        min(c + distance, current_min_distance)
                    } else {
                        c + distance
                    };
                    debug!(
                        reached,
                        new_reached, current, next, distance, dp[new_reached][next]
                    );
                }
            }
        }
        debug!(dp);

        println!("{}", dp[n_sets - 1][0] as f64 / factor);
    }
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
