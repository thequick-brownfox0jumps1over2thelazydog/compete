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

fn adjacency_maps(
    n_nodes: usize,
    edges: Vec<(usize, usize, usize)>,
    is_symmetric: bool,
) -> Vec<BTreeMap<usize, usize>> {
    let mut result = vec![BTreeMap::new(); n_nodes];
    for (a, b, v) in edges.into_iter() {
        result[a].insert(b, v);
        if is_symmetric {
            result[b].insert(a, v);
        }
    }
    result
}
fn dfs(
    node: usize,
    goal: usize,
    adjacency_maps: &Vec<BTreeMap<usize, usize>>,
    histories: &mut Vec<bool>,
    route: &mut Vec<usize>,
) -> bool {
    histories[node] = true;
    if node == goal {
        route.push(node);
        return true;
    }

    let mut is_reached = false;
    for &nn in adjacency_maps[node].keys().rev() {
        if histories[nn] {
            continue;
        }

        is_reached = dfs(nn, goal, adjacency_maps, histories, route);
        if is_reached {
            route.push(node);
            break;
        }
    }

    is_reached
}

#[fastout]
fn solve() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
    }
    debug!(N, M, AB);

    let adjacency_maps = adjacency_maps(
        N,
        AB.into_iter()
            .map(|(a, b)| (a, b, 1))
            .collect::<Vec<(usize, usize, usize)>>(),
        true,
    );

    let mut distances = vec![-1; N];
    distances[0] = 0;
    let mut distance = 1;
    let mut queues = vec![0];

    while !queues.is_empty() {
        let mut next_queues = vec![];
        for &i in queues.iter() {
            for &j in adjacency_maps[i].keys() {
                if distances[j] >= 0 {
                    continue;
                }

                distances[j] = distance;
                next_queues.push(j);
            }
        }

        distance += 1;
        queues = next_queues;
    }

    for i in 0..N {
        println!("{}", distances[i]);
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
