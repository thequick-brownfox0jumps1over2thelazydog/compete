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
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
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
        M: usize,
        ABC: [(Usize1, Usize1, usize); M],
    }
    debug!(N, M, ABC);

    let adjacency_maps = adjacency_maps(N, ABC, true);
    debug!(adjacency_maps);

    let mut queues = BinaryHeap::from_iter([Node {
        id: 0,
        previous_node_id: 0,
        distance: 0,
    }]);
    let mut nodes = (0..N)
        .map(|n| Node {
            id: n,
            previous_node_id: n,
            distance: -1,
        })
        .collect::<Vec<Node>>();

    while !queues.is_empty() {
        debug!(queues);

        let min_node = queues.pop().unwrap();
        if nodes[min_node.id].distance >= 0 {
            continue;
        }

        nodes[min_node.id] = if min_node.id == 0 {
            Node {
                id: min_node.id,
                previous_node_id: min_node.id,
                distance: 0,
            }
        } else {
            Node {
                id: min_node.id,
                previous_node_id: min_node.previous_node_id,
                distance: min_node.distance,
            }
        };

        /**
        if min_node.id == N - 1 {
            break;
        }
        */
        for (&next_node, &edge_value) in adjacency_maps[min_node.id].iter() {
            if nodes[next_node].distance >= 0 {
                continue;
            }

            let new_distance = min_node.distance + edge_value as isize;
            queues.push(Node {
                id: next_node,
                previous_node_id: min_node.id,
                distance: new_distance,
            });
        }
    }

    debug!(nodes);

    /**
    let mut result = vec![];
    let mut node_id = N - 1;
    while node_id > 0 {
        result.push(node_id);
        node_id = nodes[node_id].previous_node_id;
    }
    result.push(0);
    */
    for n in nodes.iter() {
        println!("{}", n.distance);
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
