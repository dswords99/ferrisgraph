use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

mod macros;
pub use macros::*;

struct Edge<N, E> {
    src: N,
    dst: N,
    weight: E
}

/// A directed, weighted graph implementation using Rust standard library containers.
/// 
/// It is required that the node type implements Hash, Eq
pub struct Graph<N, E>
where
    N: Hash + Eq,
{
    nodes: HashSet<Rc<N>>,
    edges: HashMap<Rc<N>, HashSet<Edge<Rc<N>, E>>>
}

impl<N, E> Graph<N, E>
where
    N: Hash + Eq,
{
    /// Creates an empty `Graph`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ferrisgraph::Graph;
    /// let mut g: Graph<String, i32> = Graph::new();
    /// ```
    pub fn new() -> Self {
        Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    /// Returns `true` if a node is in the graph.
    ///
    /// # Examples
    /// 
    /// ```
    /// use ferrisgraph::Graph;
    /// let mut g: Graph<i32, i32> = Graph::new();
    /// 
    /// assert_eq!(g.is_node(&1), false);
    /// g.add_node(1);
    /// assert!(g.is_node(&1));
    /// ```
    pub fn is_node(&self, node: &N) -> bool {
        self.nodes.contains(node)
    }

    /// Adds a node to the graph.
    /// Returns true if successful, and false if the node already exists in the graph.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ferrisgraph::*;
    /// let mut g: Graph<String, i32> = Graph::new();
    /// 
    /// assert!(g.add_node("Sydney".to_string()));
    /// assert_eq!(g.add_node("Sydney".to_string()), false);
    /// ```
    pub fn add_node(&mut self, node: N) -> bool {
        if self.is_node(&node) {
            return false;
        }

        let new_node = Rc::new(node);
        
        self.nodes.insert(Rc::clone(&new_node));
        self.edges.insert(Rc::clone(&new_node), HashSet::new());
        true
    }



    /// Returns a reference to the set of nodes in the graph.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ferrisgraph::*;
    /// let mut g: Graph<i32, i32> = graph_with_nodes![1, 2, 3];
    /// 
    /// let nodes = g.nodes();
    /// 
    /// assert!(nodes.contains(&1));
    /// assert!(nodes.contains(&3));
    /// assert_eq!(nodes.contains(&42), false);
    /// 
    /// ```
    pub fn nodes(&self) -> &HashSet<Rc<N>> {
        &(self.nodes)
    }

    /// Returns true if the graph has no nodes (and thus, no edges).
    /// 
    /// # Examples
    /// ```
    /// use ferrisgraph::*;
    /// let mut g: Graph<String, i32> = Graph::new();
    /// 
    /// assert!(g.is_empty());
    /// g.add_node("Sydney".to_string());
    /// assert_eq!(g.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

}