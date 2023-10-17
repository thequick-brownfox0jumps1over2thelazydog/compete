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
    cmp::{max, min, Ordering},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Formatter, Result},
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
struct Node {
    id: usize,
    previous_node_id: usize,
    distance: isize,
}
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Node(id: {}, previous_node_id: {}, distance: {})",
            self.id, self.previous_node_id, self.distance
        )
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl Eq for Node {}

#[fastout]
fn solve() {
    input! {
        N: usize,
        T: Usize1,
        AB: [(Usize1, Usize1); N - 1],
    }
    debug!(N, T, AB);

    let adjacency_maps = adjacency_maps(
        N,
        AB.into_iter()
            .map(|(a, b)| (a, b, 1))
            .collect::<Vec<(usize, usize, usize)>>(),
        true,
    );
    debug!(adjacency_maps);

    let mut distances = (0..N).map(|i| (i, -1)).collect::<Vec<(usize, isize)>>();
    let mut queues = VecDeque::new();
    queues.push_back(Node {
        id: T,
        previous_node_id: T,
        distance: 0,
    });

    while !queues.is_empty() {
        let node = queues.pop_front().unwrap();
        distances[node.id] = (node.id, node.distance);
        for &next_node in adjacency_maps[node.id].keys() {
            if distances[next_node].1 >= 0 {
                continue;
            }
            queues.push_back(Node {
                id: next_node,
                previous_node_id: node.id,
                distance: node.distance + 1,
            });
        }
    }

    let mut loop_order = distances.clone();
    loop_order.sort_by(|a, b| (-a.1).cmp(&(-b.1)));
    let bottom = loop_order[0].1;

    let mut result = vec![0; N];
    for (index, layer) in loop_order.iter() {
        if *layer == bottom {
            continue;
        }

        result[*index] = adjacency_maps[*index]
            .keys()
            .filter(|&n| distances[*n].1 >= *layer)
            .map(|n| result[*n])
            .max()
            .unwrap_or(-1)
            + 1;
    }

    println!(
        "{}",
        result
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn main() {
    Builder::new()
        .stack_size(32 * 1024 * 1024) // default: 2MiB -> 32MiB
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
