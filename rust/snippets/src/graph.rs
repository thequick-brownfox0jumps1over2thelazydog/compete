#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{Debug, Formatter, Result},
};

use cargo_snippet::snippet;

#[snippet("graph", prefix = "use std::cmp::Ordering;")]
#[snippet(prefix = "use std::collections::BTreeMap;")]
#[snippet(prefix = "use std::fmt::{Debug, Formatter, Result};")]
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

#[snippet("graph")]
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

#[snippet("graph")]
struct Node {
    id: usize,
    previous_node_id: usize,
    distance: isize,
}

#[snippet("graph")]
impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Node(id: {}, previous_node_id: {}, distance: {})",
            self.id, self.previous_node_id, self.distance
        )
    }
}

#[snippet("graph")]
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

#[snippet("graph")]
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[snippet("graph")]
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

#[snippet("graph")]
impl Eq for Node {}
