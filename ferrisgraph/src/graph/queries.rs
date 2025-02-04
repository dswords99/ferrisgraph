use super::graph::GraphError;
use super::Graph;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

impl<N, E> Graph<N, E>
where
    N: Hash + Eq + Debug + Ord,
    E: Hash + Eq + Ord,
{
    /// Returns a reference to the set of nodes in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    /// let mut g: Graph<i32, i32> = graph_with_nodes![1, 2, 3];
    ///
    /// let nodes = g.nodes();
    ///
    /// assert!(nodes.contains(&1));
    /// assert!(nodes.contains(&3));
    /// assert_eq!(nodes.contains(&42), false);
    ///
    /// ```
    pub fn nodes(&self) -> &BTreeSet<Rc<N>> {
        &(self.nodes)
    }

    /// Returns `true` if the graph has no nodes (and thus, no edges).
    ///
    /// # Examples
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    /// let mut g: Graph<String, i32> = Graph::new();
    ///
    /// assert!(g.is_empty());
    /// g.add_node("Sydney".to_string());
    /// assert_eq!(g.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the amount of nodes present in the graph.
    ///
    /// # Examples
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
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

    /// Returns an optional `Vec<(&N, &E)>` containing all the outgoing edges from the given node.
    /// None is returned if there exist no edges from the node.
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Beijing", "Shanghai", "Guangzhou");
    ///
    /// g.add_edge(&"Beijing", &"Shanghai", Some(100));
    /// g.add_edge(&"Beijing", &"Guangzhou", Some(200));
    ///
    /// let expected = vec![(&"Guangzhou", &Some(200)), (&"Shanghai", &Some(100))];
    /// let cons = g.edges(&"Beijing");
    ///
    /// assert!(cons.is_ok());
    ///
    /// let cons = cons.unwrap();
    ///
    /// assert!(cons.is_some());
    ///
    /// let mut cons = cons.unwrap();
    ///
    /// cons.sort();
    ///
    /// assert_eq!(expected, cons);
    /// assert_eq!(g.edges(&"Shanghai"), Ok(None));
    ///
    /// ```
    pub fn edges<'a>(
        &self,
        node: &'a N,
    ) -> Result<Option<Vec<(&N, &Option<E>)>>, GraphError<'a, N>> {
        let node_edges = match self.edges.get(node) {
            Some(set) => set,
            None => return Err(GraphError::NodeNotFound(node)),
        };

        if node_edges.is_empty() {
            return Ok(None);
        };

        let mut vec = Vec::new();

        node_edges.iter().for_each(|(n, e)| vec.push((&(**n), e)));

        Ok(Some(vec))
    }

    /// Returns an optional `Vec<&N>` containing all the outgoing connections from the given node.
    /// Returns None if there exist no outgoing connections from the node.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("London", "Glasgow", "Manchester");
    /// g.add_edge(&"London", &"Glasgow", None);
    /// g.add_edge(&"Manchester", &"London", Some(100));
    ///
    /// let expected = vec![&"Glasgow"];
    /// let cons = g.connections(&"London");
    ///
    /// assert!(cons.is_ok());
    /// let cons = cons.unwrap();
    ///
    /// assert!(cons.is_some());
    /// let cons = cons.unwrap();
    ///
    /// assert_eq!(expected, cons);
    /// ```
    pub fn connections<'a>(&self, node: &'a N) -> Result<Option<Vec<&N>>, GraphError<'a, N>> {
        let node_edges = match self.edges.get(node) {
            Some(set) => set,
            None => return Err(GraphError::NodeNotFound(node)),
        };

        if node_edges.is_empty() {
            return Ok(None);
        };

        let mut vec = Vec::new();

        node_edges.iter().for_each(|(n, _)| vec.push(&(**n)));

        Ok(Some(vec))
    }

    /// This function returns the number of edges that are currently in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Madrid", "Barcelona", "Malaga");
    ///
    /// assert_eq!(g.num_edges(), 0);
    /// g.add_edge(&"Madrid", &"Malaga", None);
    /// assert_eq!(g.num_edges(), 1);
    /// ```
    pub fn num_edges(&self) -> usize {
        self.edges.iter().map(|(_, set)| set.len()).sum()
    }

    /// This function returns the out-degree of the given node. That is, the number of outgoing edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Johannesburg", "Cape Town", "Durban");
    ///
    /// assert_eq!(g.out_degree(&"Johannesburg"), 0);
    ///
    /// g.add_edge(&"Johannesburg", &"Cape Town", None);
    /// assert_eq!(g.out_degree(&"Johannesburg"), 1);
    ///
    /// g.add_edge(&"Johannesburg", &"Durban", Some(100));
    /// assert_eq!(g.out_degree(&"Johannesburg"), 2);
    ///
    /// g.add_edge(&"Cape Town", &"Johannesburg", Some(1000));
    /// assert_eq!(g.out_degree(&"Johannesburg"), 2);
    /// ```
    pub fn out_degree(&self, node: &N) -> usize {
        let node_edges = match self.edges.get(node) {
            Some(set) => set,
            None => return 0,
        };

        node_edges.len()
    }

    /// This function returns the in-degree of the given node. That is, the number of incoming edges.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Rio de Janeiro", "Sao Paulo", "Brasilia");
    ///
    /// assert_eq!(g.in_degree(&"Rio de Janeiro"), 0);
    ///
    /// g.add_edge(&"Rio de Janeiro", &"Sao Paulo", None);
    /// assert_eq!(g.in_degree(&"Rio de Janeiro"), 0);
    ///
    /// g.add_edge(&"Brasilia", &"Rio de Janeiro", Some(100));
    /// assert_eq!(g.in_degree(&"Rio de Janeiro"), 1);
    ///
    /// g.add_edge(&"Sao Paulo", &"Rio de Janeiro", Some(1000));
    /// assert_eq!(g.in_degree(&"Rio de Janeiro"), 2);
    /// ```
    pub fn in_degree(&self, node: &N) -> usize {
        if !self.is_node(node) {
            return 0;
        }

        self.edges
            .iter()
            .flat_map(|(_, set)| set.iter())
            .filter(|(dst, _)| **dst == *node)
            .count()
    }

    /// This function returns the degree of the given node. That is, the number of edges connected to the node, incoming or outgoing.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Toronto", "Vancouver", "Montreal");
    ///
    /// assert_eq!(g.degree(&"Toronto"), 0);
    ///
    /// g.add_edge(&"Toronto", &"Vancouver", None);
    /// assert_eq!(g.degree(&"Toronto"), 1);
    ///
    /// g.add_edge(&"Montreal", &"Toronto", Some(100));
    /// assert_eq!(g.degree(&"Toronto"), 2);
    ///
    /// g.add_edge(&"Vancouver", &"Toronto", Some(1000));
    ///
    /// assert_eq!(g.degree(&"Toronto"), 3);
    /// assert_eq!(g.degree(&"Vancouver"), 2);
    /// assert_eq!(g.degree(&"Montreal"), 1);
    /// ```
    pub fn degree(&self, node: &N) -> usize {
        self.in_degree(node) + self.out_degree(node)
    }
}
