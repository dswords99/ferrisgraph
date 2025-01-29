use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;

mod macros;

// #[derive(PartialEq)]
// struct Edge<N, E>
// {
//     src: N,
//     dst: N,
//     weight: E
// }

/// A directed, weighted graph implementation using Rust standard library containers.
///
/// It is required that the node type implements Hash, Eq
pub struct Graph<N, E>
where
    N: Hash + Eq,
    E: Hash + Eq,
{
    nodes: HashSet<Rc<N>>,
    edges: HashMap<Rc<N>, HashSet<(Rc<N>, E)>>,
}

impl<N, E> Graph<N, E>
where
    N: Hash + Eq,
    E: Hash + Eq,
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

    /// Returns `true` if a given node is in the graph.
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
    /// Returns `true` if successful, and `false` if the node already exists in the graph.
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

    /// Returns `true` if the graph has no nodes (and thus, no edges).
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

    /// Returns `true` if a given edge is in the graph.
    ///
    /// # Examples
    /// ```
    /// use ferrisgraph::*;
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Seoul", "Busan", "Jeju");
    ///
    /// assert_eq!(g.is_edge(&"Seoul", &"Busan", &1000), false);
    /// g.add_edge(&"Seoul", &"Busan", 1000);
    /// assert!(g.is_edge(&"Seoul", &"Busan", &1000));
    /// ```
    pub fn is_edge(&self, src: &N, dst: &N, weight: &E) -> bool {
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
    /// use ferrisgraph::*;
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Taipei", "Kaohsiung", "Hualien");
    ///
    /// assert!(g.add_edge(&"Kaohsiung", &"Hualien", 300));
    /// assert_eq!(g.add_edge(&"Kaohsiung", &"Hualien", 300), false);
    /// ```
    pub fn add_edge(&mut self, src: &N, dst: &N, weight: E) -> bool {
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
    /// use ferrisgraph::*;
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Berlin", "Frankfurt", "Munich");
    /// g.add_edge(&"Berlin", &"Munich", 100);
    /// g.add_edge(&"Frankfurt", &"Berlin", 100);
    ///
    /// assert!(g.remove_node(&"Berlin"));
    /// assert_eq!(g.remove_node(&"Hamburg"), false);
    ///
    /// assert_eq!(g.is_node(&"Berlin"), false);
    /// assert_eq!(g.is_edge(&"Berlin", &"Hamburg", &100), false);
    /// assert_eq!(g.is_edge(&"Frankfurt", &"Berlin", &100), false);
    ///
    /// ```
    pub fn remove_node(&mut self, node: &N) -> bool {
        if !self.is_node(node) {
            return false;
        }

        // Remove the HashSet associated with node (out-going edges)
        self.edges.remove(node);

        // Remove all edges in other HashSets associated with node (in-going edges)
        self.edges
            .iter_mut()
            .for_each(|(_, set)| set.retain(|(dst, _)| **dst != *node));

        // Remove the node itself
        self.nodes.remove(node);

        true
    }

    /// Returns the amount of nodes present in the graph.
    ///
    /// # Examples
    /// ```
    /// use ferrisgraph::*;
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Tokyo", "Osaka", "Sapporo");
    ///
    /// assert_eq!(g.num_nodes(), 3);
    /// g.add_node("Fukuoka");
    /// assert_eq!(g.num_nodes(), 4);
    ///
    /// ```
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Removes a given edge from the graph.
    /// Returns `true` if successful, and `false` if the edge already does not exist in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::*;
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("New York", "Los Angeles", "Chicago");
    ///
    /// g.add_edge(&"New York", &"Chicago", 100);
    ///
    /// assert!(g.is_edge(&"New York", &"Chicago", &100));
    /// assert!(g.remove_edge(&"New York", &"Chicago", 100));
    ///
    /// assert_eq!(g.is_edge(&"New York", &"Chicago", &100), false);
    /// assert_eq!(g.remove_edge(&"New York", &"Chicago", 100), false);
    ///
    /// ```
    pub fn remove_edge(&mut self, src: &N, dst: &N, weight: E) -> bool {
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

    /// Returns an optional `Vec<(&N, &E)>` containing all the outgoing edges from the given node.
    /// None is returned if there exist no edges from the node.
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::*;
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Beijing", "Shanghai", "Guangzhou");
    ///
    /// g.add_edge(&"Beijing", &"Shanghai", 100);
    /// g.add_edge(&"Beijing", &"Guangzhou", 200);
    ///
    /// let expected = vec![(&"Guangzhou", &200), (&"Shanghai", &100)];
    /// let mut cons = g.edges(&"Beijing").expect("We know that Beijing is a node.");
    /// cons.sort();
    ///
    /// assert_eq!(expected, cons);
    /// assert_eq!(g.edges(&"Shanghai"), None);
    ///
    /// ```
    pub fn edges(&self, node: &N) -> Option<Vec<(&N, &E)>> {
        let node_edges = match self.edges.get(node) {
            Some(set) => set,
            None => return None,
        };

        if node_edges.is_empty() {
            return None;
        };

        let mut vec = Vec::new();

        node_edges.iter().for_each(|(n, e)| vec.push((&(**n), e)));

        Some(vec)
    }

    /// Returns `true` if an edge exists between the source and destination, and `false` if not.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::*;
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("New Delhi", "Mumbai", "Bengaluru");
    ///
    /// g.add_edge(&"Mumbai", &"Bengaluru", 100);
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
