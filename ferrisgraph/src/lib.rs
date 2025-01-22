use std::collections::{HashMap, HashSet};
use std::hash::Hash;

struct Edge<N, E> {
    src: N,
    dst: N,
    weight: E
}

pub struct Graph<N, E>
where
    N: Hash + Eq + Copy,
{
    nodes: HashSet<N>,
    edges: HashMap<N, HashSet<Edge<N, E>>>
}

impl<N, E> Graph<N, E>
where
    N: Hash + Eq + Copy,
{

    pub fn new() -> Self {
        Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn is_node(&self, node: N) -> bool {
        self.nodes.contains(&node)
    }

    pub fn add_node(&mut self, node: N) -> bool {
        if self.is_node(node) {
            return false;
        }

        self.nodes.insert(node);
        self.edges.insert(node, HashSet::new());
        true
    }

}