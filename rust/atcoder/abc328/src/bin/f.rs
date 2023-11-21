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

use itertools::Itertools;
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
struct Node {
    id: usize,
    previous_node_id: usize,
    distance: isize,
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
#[derive(Clone, Debug)]
struct Edge {
    capacity: usize,
    flow: usize,
}
impl Edge {
    fn increasable(&self) -> bool {
        self.surplus() > 0
    }
    fn decreasable(&self) -> bool {
        self.flow > 0
    }
    fn surplus(&self) -> usize {
        self.capacity - self.flow
    }
    fn increase_flow(&mut self, diff: usize) {
        if !self.increasable() {
            panic!("Cannot increase flow.");
        }
        self.flow += diff;
    }
    fn decrease_flow(&mut self, diff: usize) {
        if !self.decreasable() {
            panic!("Cannot decrease flow.");
        }
        self.flow -= diff;
    }
}
#[derive(Debug, PartialEq)]
enum NetworkType {
    Undirected,
    Directed,
    Residual,
}
#[derive(Debug)]
struct Network {
    network_type: NetworkType,
    n_nodes: usize,
    edge_maps: Vec<BTreeMap<usize, Edge>>,
}
impl Network {
    fn new(
        network_type: NetworkType,
        n_nodes: usize,
        edges: Vec<(usize, usize, usize)>,
    ) -> Network {
        let mut network = Network {
            network_type,
            n_nodes,
            edge_maps: vec![BTreeMap::new(); n_nodes],
        };
        for (a, b, c) in edges.into_iter() {
            network.add_edge(a, b, c);
        }
        network
    }
    fn add_edge(&mut self, from: usize, to: usize, capacity: usize) {
        self.edge_maps[from]
            .entry(to)
            .and_modify(|e| {
                e.capacity += capacity;
            })
            .or_insert(Edge { capacity, flow: 0 });
        if self.network_type == NetworkType::Undirected {
            self.edge_maps[to].insert(from, Edge { capacity, flow: 0 });
        }
        if self.network_type == NetworkType::Residual {
            self.edge_maps[to]
                .entry(from)
                .and_modify(|e| {
                    e.capacity += capacity;
                    e.flow += capacity;
                })
                .or_insert(Edge {
                    capacity,
                    flow: capacity,
                });
        }
    }
    fn depth_first_search(
        &self,
        node_id: usize,
        goal_id: usize,
        histories: &mut Vec<bool>,
        route: &mut Vec<usize>,
    ) -> bool {
        histories[node_id] = true;
        if node_id == goal_id {
            route.push(node_id);
            return true;
        }
        let mut is_reached = false;
        for &next_node_id in self.edge_maps[node_id].keys().rev() {
            if histories[next_node_id] {
                continue;
            }
            if !self.edge_maps[node_id][&next_node_id].increasable() {
                continue;
            }
            is_reached = self.depth_first_search(next_node_id, goal_id, histories, route);
            if is_reached {
                route.push(node_id);
                break;
            }
        }
        is_reached
    }
    fn breadth_first_search(&mut self, start_id: usize, goal_id: usize) -> Vec<Node> {
        let mut nodes = (0..self.n_nodes)
            .map(|n| Node {
                id: n,
                previous_node_id: n,
                distance: -1,
            })
            .collect::<Vec<Node>>();
        let mut queues = VecDeque::new();
        queues.push_back(Node {
            id: start_id,
            previous_node_id: start_id,
            distance: 0,
        });
        while !queues.is_empty() {
            let node = queues.pop_front().unwrap();
            if nodes[node.id].distance >= 0 {
                continue;
            }
            nodes[node.id].previous_node_id = node.previous_node_id;
            nodes[node.id].distance = node.distance;
            if node.id == goal_id {
                break;
            }
            for &next_node in self.edge_maps[node.id].keys() {
                queues.push_back(Node {
                    id: next_node,
                    previous_node_id: node.id,
                    distance: node.distance + 1,
                });
            }
        }
        nodes
    }
    fn dijkstra(&mut self, start_id: usize, goal_id: usize) -> Vec<Node> {
        let mut nodes = (0..self.n_nodes)
            .map(|n| Node {
                id: n,
                previous_node_id: n,
                distance: -1,
            })
            .collect::<Vec<Node>>();
        let mut queues = BinaryHeap::from_iter([Node {
            id: start_id,
            previous_node_id: start_id,
            distance: 0,
        }]);
        while !queues.is_empty() {
            let min_node = queues.pop().unwrap();
            if nodes[min_node.id].distance >= 0 {
                continue;
            }
            nodes[min_node.id].previous_node_id = min_node.previous_node_id;
            nodes[min_node.id].distance = min_node.distance;
            if min_node.id == goal_id {
                break;
            }
            for (next_node, edge) in self.edge_maps[min_node.id].iter() {
                if nodes[*next_node].distance >= 0 {
                    continue;
                }
                queues.push(Node {
                    id: *next_node,
                    previous_node_id: min_node.id,
                    distance: min_node.distance + edge.capacity as isize,
                });
            }
        }
        nodes
    }
    fn max_flow(&mut self, capacity_upper_bound: usize) -> usize {
        let mut result = 0;
        while true {
            let mut histories = vec![false; self.n_nodes];
            let mut route: Vec<usize> = vec![];
            if !self.depth_first_search(0, self.n_nodes - 1, &mut histories, &mut route) {
                break;
            }
            route.reverse();
            let mut min_capacity = capacity_upper_bound;
            for i in 0..route.len() - 1 {
                min_capacity = min_capacity.min(self.edge_maps[route[i]][&route[i + 1]].surplus());
            }
            for i in 0..route.len() - 1 {
                let from = route[i];
                let to = route[i + 1];
                self.edge_maps[from].entry(to).and_modify(|e| {
                    e.increase_flow(min_capacity);
                });
                self.edge_maps[to].entry(from).and_modify(|e| {
                    e.decrease_flow(min_capacity);
                });
            }
            result += min_capacity;
        }
        result
    }
}
#[derive(Debug)]
struct UnionFind {
    parents: Vec<usize>,
    tree_sizes: Vec<usize>,
    weights: Vec<isize>,
}
impl UnionFind {
    fn new(n_nodes: usize) -> Self {
        Self {
            parents: (0..n_nodes).collect::<Vec<usize>>(),
            tree_sizes: vec![1; n_nodes],
            weights: vec![0; n_nodes],
        }
    }
    fn root_of(&mut self, node: usize) -> usize {
        let parent = self.parents[node];
        if parent == node {
            return node;
        }
        let grand_parent = self.root_of(parent);
        self.parents[node] = grand_parent;
        self.weights[node] += self.weights[parent];
        grand_parent
    }
    fn weight_of(&mut self, node: usize) -> isize {
        let _ = self.root_of(node);
        self.weights[node]
    }
    fn is_same(&mut self, a: usize, b: usize) -> bool {
        self.root_of(a) == self.root_of(b)
    }
    fn diff_between(&mut self, a: usize, b: usize) -> isize {
        if !self.is_same(a, b) {
            panic!("Can't calculate diff between two nodes in different trees");
        }
        self.weights[a] - self.weights[b]
    }
    fn merge_or_verify(&mut self, a: usize, b: usize, weight_diff: isize) -> bool {
        let a_root = self.root_of(a);
        let b_root = self.root_of(b);
        if a_root == b_root {
            return self.diff_between(b, a) == weight_diff;
        }
        let new_weight = self.weight_of(a) - self.weight_of(b) + weight_diff;
        match self.tree_sizes[a_root].cmp(&self.tree_sizes[b_root]) {
            Ordering::Greater | Ordering::Equal => {
                self.parents[b_root] = a_root;
                self.tree_sizes[a_root] += self.tree_sizes[b_root];
                self.tree_sizes[b_root] = 1;
                self.weights[b_root] = new_weight;
            }
            Ordering::Less => {
                self.parents[a_root] = b_root;
                self.tree_sizes[b_root] += self.tree_sizes[a_root];
                self.tree_sizes[a_root] = 1;
                self.weights[a_root] = -new_weight;
            }
        }
        true
    }
}

#[fastout]
fn solve() {
    // from: 2023-11-17 15:17
    input! {
        N: usize,
        Q: usize,
        ABD: [(Usize1, Usize1, isize); Q],
    }
    debug!(ABD);

    let mut union_find = UnionFind::new(N);

    let mut indices = vec![];
    for i in 0..Q {
        let (a, b, d) = ABD[i];
        if union_find.merge_or_verify(a, b, d) {
            indices.push((i + 1).to_string());
        }
        debug!(union_find);
    }

    if !indices.is_empty() {
        println!("{}", indices.join(" "));
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
