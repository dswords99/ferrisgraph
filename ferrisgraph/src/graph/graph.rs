use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use thiserror::Error;

// mod macros;

/// A directed, weighted multi-graph implementation using Rust standard library containers.
/// The data structure can be used as unweighted by making all weights None, or can be used
/// as a mixed graph with both weighted and unweighted edges.
///
/// It is required that the node type implements Hash, Eq, Ord and Debug.
/// It is required that the edge type implements Hash, Eq, Ord and Debug.

#[derive(PartialEq, Debug)]
pub struct Graph<N, E>
where
    N: Hash + Eq + Ord + Debug,
    E: Hash + Eq + Ord,
{
    pub(crate) nodes: BTreeSet<Rc<N>>,
    pub(crate) edges: BTreeMap<Rc<N>, BTreeSet<(Rc<N>, Option<E>)>>,
}

#[derive(Debug, Error, PartialEq)]
pub enum GraphError<'a, N>
where
    N: Debug,
{
    #[error("Node {:?} does not exist.", _0)]
    NodeNotFound(&'a N),
}

impl<N, E> Graph<N, E>
where
    N: Hash + Eq + Debug + Ord,
    E: Hash + Eq + Ord,
{
    /// Creates an empty `Graph`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::graph::Graph;
    /// let mut g: Graph<String, i32> = Graph::new();
    /// ```
    pub fn new() -> Self {
        Graph {
            nodes: BTreeSet::new(),
            edges: BTreeMap::new(),
        }
    }

    /// Returns `true` if a given node is in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::graph::Graph;
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
    /// Returns `true` if successful, and `false` if the node already exists in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
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
        self.edges.insert(Rc::clone(&new_node), BTreeSet::new());
        true
    }

    /// Returns `true` if a given edge is in the graph.
    ///
    /// # Examples
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Seoul", "Busan", "Jeju");
    ///
    /// assert_eq!(g.is_edge(&"Seoul", &"Busan", &Some(1000)), false);
    /// g.add_edge(&"Seoul", &"Busan", Some(1000));
    /// assert!(g.is_edge(&"Seoul", &"Busan", &Some(1000)));
    /// ```
    pub fn is_edge(&self, src: &N, dst: &N, weight: &Option<E>) -> bool {
        if !self.is_node(dst) {
            return false;
        }

        let src_edges = match self.edges.get(src) {
            Some(set) => set,
            None => return false,
        };

        src_edges
            .iter()
            .any(|(rc_dst, w)| **rc_dst == *dst && *weight == *w)
    }

    /// Adds an edge to the graph.
    /// Returns `true` if successful, and `false` if the edge already exists in the graph.
    ///
    /// # Examples
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Taipei", "Kaohsiung", "Hualien");
    ///
    /// assert!(g.add_edge(&"Kaohsiung", &"Hualien", Some(300)));
    /// assert!(g.add_edge(&"Taipei", &"Hualien", None));
    /// assert_eq!(g.add_edge(&"Kaohsiung", &"Hualien", Some(300)), false);
    /// ```
    pub fn add_edge(&mut self, src: &N, dst: &N, weight: Option<E>) -> bool {
        if self.is_edge(src, dst, &weight) {
            return false;
        }

        let src_edges = match self.edges.get_mut(src) {
            Some(set) => set,
            None => return false,
        };

        let rc_dst = match self.nodes.get(dst) {
            Some(rc) => rc,
            None => return false,
        };

        src_edges.insert((rc_dst.clone(), weight));

        true
    }

    /// Removes a node from the graph, and thus all associated edges.
    /// Returns `true` if successful, and `false` if the node already does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Berlin", "Frankfurt", "Munich");
    /// g.add_edge(&"Berlin", &"Munich", Some(100));
    /// g.add_edge(&"Frankfurt", &"Berlin", Some(100));
    ///
    /// assert!(g.remove_node(&"Berlin"));
    /// assert_eq!(g.remove_node(&"Hamburg"), false);
    ///
    /// assert_eq!(g.is_node(&"Berlin"), false);
    /// assert_eq!(g.is_edge(&"Berlin", &"Hamburg", &Some(100)), false);
    /// assert_eq!(g.is_edge(&"Frankfurt", &"Berlin", &Some(100)), false);
    ///
    /// ```
    pub fn remove_node(&mut self, node: &N) -> bool {
        if !self.is_node(node) {
            return false;
        }

        // Remove the BTreeSet associated with node (out-going edges)
        self.edges.remove(node);

        // Remove all edges in other BTreeSets associated with node (in-going edges)
        self.edges
            .iter_mut()
            .for_each(|(_, set)| set.retain(|(dst, _)| **dst != *node));

        // Remove the node itself
        self.nodes.remove(node);

        true
    }

    /// Removes a given edge from the graph.
    /// Returns `true` if successful, and `false` if the edge already does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("New York", "Los Angeles", "Chicago");
    ///
    /// g.add_edge(&"New York", &"Chicago", Some(100));
    ///
    /// assert!(g.is_edge(&"New York", &"Chicago", &Some(100)));
    /// assert!(g.remove_edge(&"New York", &"Chicago", Some(100)));
    ///
    /// assert_eq!(g.is_edge(&"New York", &"Chicago", &Some(100)), false);
    /// assert_eq!(g.remove_edge(&"New York", &"Chicago", Some(100)), false);
    ///
    /// ```
    pub fn remove_edge(&mut self, src: &N, dst: &N, weight: Option<E>) -> bool {
        if !self.is_edge(src, dst, &weight) {
            return false;
        }

        let src_edges = self
            .edges
            .get_mut(src)
            .expect("We just verified the edge, and thus the src, exists.");
        let dst_rc = self
            .nodes
            .get(dst)
            .expect("We just verified the edge, and thus the dst, exists.");

        src_edges.remove(&(dst_rc.clone(), weight));

        true
    }

    /// Returns `true` if an edge exists between the source and destination, and `false` if not.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("New Delhi", "Mumbai", "Bengaluru");
    ///
    /// g.add_edge(&"Mumbai", &"Bengaluru", Some(100));
    ///
    /// assert!(g.is_connected(&"Mumbai", &"Bengaluru"));
    /// assert_eq!(g.is_connected(&"New Delhi", &"Bengaluru"), false);
    /// assert_eq!(g.is_connected(&"Bengaluru", &"Mumbai", ), false);
    /// ```
    pub fn is_connected(&self, src: &N, dst: &N) -> bool {
        let src_edges = match self.edges.get(src) {
            Some(set) => set,
            None => return false,
        };

        src_edges.iter().any(|(n, _)| **n == *dst)
    }
}

impl<N, E> Graph<N, E>
where
    N: Hash + Eq + Ord + Debug + Clone,
    E: Hash + Eq + Ord + Clone,
{
    /// This function clones a graph. It is required that the node and edge types are clone.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Riyadh", "Jeddah", "Mecca");
    /// g.add_edge(&"Riyadh", &"Jeddah", None);
    ///
    /// let mut new_g: Graph<&str, i32> = graph_with_nodes!("Foo");
    /// assert_ne!(new_g, g);
    ///
    /// new_g = g.clone();
    /// assert_eq!(new_g, g);
    ///
    /// ```
    pub fn clone(&self) -> Self {
        Graph {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        }
    }

    /// This function adds an undirected edge, i.e. it automatically adds two directed edges going either way between two nodes.
    /// Returns true if successful, and returns false if either of the edges already exist, or if src and dst are the same (loop).
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Lagos", "Abuja", "Kano");
    ///
    /// assert!(g.add_undirected_edge(&"Lagos", &"Kano", None));
    /// assert_eq!(g.add_undirected_edge(&"Lagos", &"Kano", None), false);
    ///
    /// assert!(g.is_edge(&"Lagos", &"Kano", &None));
    /// assert!(g.is_edge(&"Kano", &"Lagos", &None));
    /// ```
    pub fn add_undirected_edge(&mut self, src: &N, dst: &N, weight: Option<E>) -> bool {
        if src == dst {
            return false;
        }

        if self.is_edge(src, dst, &weight) || self.is_edge(dst, src, &weight) {
            return false;
        }

        self.add_edge(src, dst, weight.clone()) && self.add_edge(dst, src, weight)
    }
}
