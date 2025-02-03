use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;
use std::rc::Rc;
use thiserror::Error;

mod macros;

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
    nodes: BTreeSet<Rc<N>>,
    edges: BTreeMap<Rc<N>, BTreeSet<(Rc<N>, Option<E>)>>,
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
    /// use ferrisgraph::Graph;
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
        self.edges.insert(Rc::clone(&new_node), BTreeSet::new());
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
    pub fn nodes(&self) -> &BTreeSet<Rc<N>> {
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
    /// use ferrisgraph::*;
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
    /// use ferrisgraph::*;
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

    /// Returns an optional `Vec<(&N, &E)>` containing all the outgoing edges from the given node.
    /// None is returned if there exist no edges from the node.
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::*;
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
    pub fn edges<'a>(&self, node: &'a N) -> Result<Option<Vec<(&N, &Option<E>)>>, GraphError<'a, N>> {
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
    /// use ferrisgraph::*;
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

    /// Returns `true` if an edge exists between the source and destination, and `false` if not.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::*;
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

    /// This function returns the number of edges that are currently in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::*;
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
    /// use ferrisgraph::*;
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
    /// use ferrisgraph::*;
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
    /// use ferrisgraph::*;
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

    /// This function performs Breadth First Search on the graph, starting from the given source node.
    /// The function returns the predecessors in the form `HashMap<&N, &N>`, where a given N will map
    /// to its predecessor node. A `GraphError::NodeNotFound` error will be returned if the source node
    /// does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::*;
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Berlin", "Paris", "London", "Milan", "Zurich");
    /// g.add_edge(&"Berlin", &"Paris", None);
    /// g.add_edge(&"Berlin", &"Zurich", None);
    /// g.add_edge(&"Paris", &"London", None);
    ///
    /// let res = g.bfs(&"Berlin");
    /// assert!(res.is_ok());
    ///
    /// let predecessor = res.unwrap();
    /// assert_eq!(predecessor.len(), 4);
    /// assert_eq!(**predecessor.get(&"Berlin").unwrap(), "Berlin");
    /// assert_eq!(**predecessor.get(&"Paris").unwrap(), "Berlin");
    /// assert_eq!(**predecessor.get(&"Zurich").unwrap(), "Berlin");
    /// assert_eq!(**predecessor.get(&"London").unwrap(), "Paris");
    ///
    ///
    /// ```
    pub fn bfs<'a>(&'a self, src: &'a N) -> Result<HashMap<&'a N, &'a N>, GraphError<'a, N>> {
        let src_rc = match self.nodes.get(src) {
            Some(rc) => rc,
            None => return Err(GraphError::NodeNotFound(src)),
        };

        let mut q = VecDeque::new();
        let mut pred = HashMap::new();

        pred.insert(&**src_rc, &**src_rc);

        q.push_back(&**src_rc);

        loop {
            let curr = match q.pop_front() {
                Some(n) => n,
                None => break,
            };

            let curr_edges = match self.edges.get(curr) {
                Some(set) => set,
                None => return Err(GraphError::NodeNotFound(&curr)),
            };

            for (dst, _) in curr_edges.iter() {
                if pred.get(&**dst).is_none() {
                    pred.insert(dst, curr);
                    q.push_back(dst);
                }
            }
        }

        Ok(pred)
    }

    /// This function performs Depth First Search on the graph from the specified source.
    /// A visited set is returned on success, whereas a `GraphError::NodeNotFound` is returned
    /// if the source doesn't exist.
    ///
    /// This function can be used for things such as finding 'islands' or if there exists a path
    /// between two nodes.
    ///
    /// # Examples
    /// ```
    /// use ferrisgraph::*;
    /// use std::collections::BTreeSet;
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Berlin", "Paris", "London", "Milan", "Zurich");
    /// g.add_edge(&"Berlin", &"Paris", None);
    /// g.add_edge(&"Berlin", &"Zurich", None);
    /// g.add_edge(&"Paris", &"London", None);
    ///
    /// let res = g.dfs(&"Berlin");
    /// assert!(res.is_ok());
    ///
    /// let visited = res.unwrap();
    /// let expected: BTreeSet<&&str> = vec![&"Berlin", &"Paris", &"Zurich", &"London"].into_iter().collect();
    ///
    /// assert_eq!(visited.len(), 4);
    /// assert_eq!(visited, expected);
    ///
    /// ```
    ///
    pub fn dfs<'a>(&'a self, src: &'a N) -> Result<BTreeSet<&'a N>, GraphError<'a, N>> {
        let mut stack = Vec::new();
        let mut visited = BTreeSet::new();

        stack.push(src);

        loop {
            let curr = match stack.pop() {
                Some(n) => n,
                None => break,
            };

            let curr_edges = match self.edges.get(curr) {
                Some(set) => set,
                None => return Err(GraphError::NodeNotFound(curr)),
            };

            visited.insert(curr);

            for (dst, _) in curr_edges {
                if !visited.contains(&**dst) {
                    stack.push(&**dst);
                }
            }
        }

        Ok(visited)
    }


    /// This function returns true if the graph contains a cycle, and false if not.
    /// A cycle is a path in a graph that starts and ends at the same vertex.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ferrisgraph::*;
    ///
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Mexico City", "Tijuana", "Monterrey");
    /// 
    /// g.add_edge(&"Mexico City", &"Tijuana", None);
    /// g.add_edge(&"Tijuana", &"Monterrey", None);
    /// 
    /// assert!(!g.has_cycle());
    /// 
    /// g.add_edge(&"Monterrey", &"Mexico City", None);
    /// 
    /// assert!(g.has_cycle());
    /// ```
    pub fn has_cycle(&self) -> bool {
        let mut visited = BTreeSet::new();
        let mut stack = BTreeSet::new();

        for node in self.nodes.iter() {
            if !visited.contains(node) {                

                if self.explore_for_cycle(node, &mut visited, &mut stack) {
                    return true;
                }

            }
        }

        false
    }

    fn explore_for_cycle(&self, node: &Rc<N>, visited: &mut BTreeSet<Rc<N>>, stack: &mut BTreeSet<Rc<N>>) -> bool {
        if stack.contains(node) {
            return true;
        }

        if visited.contains(node) {
            return false;
        }

        let edges = self.edges.get(node).expect("There is no way that this isn't a node");
        visited.insert(node.clone());
        stack.insert(node.clone());

        for (dst, _) in edges.iter() {
            if self.explore_for_cycle(dst, visited, stack) {
                return true;
            }
        }

        stack.remove(node);

        false
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
    /// use ferrisgraph::*;
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
    /// use ferrisgraph::*;
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
            return false
        }

        self.add_edge(src, dst, weight.clone()) && self.add_edge(dst, src, weight)
    }
}

impl<N, E> Graph<N, E>
where
    N: Hash + Eq + Ord + Debug,
    E: Hash + Eq + Ord + Add<Output = E> + Clone,
{
    /// This function performs Djikstra's algorithm on the graph, beginning from the source node.
    /// The parameter `default_weight` is the weight that will be used for unweighted edges,
    /// and `zero` is the distance value for the source.
    ///
    /// The function returns a tuple `(dist, pred)`, in which `dist` is of type `HashMap<&N, E>`, mapping
    /// nodes to their total distances from the source. `pred` is of type `HashMap<&N, Option<&N>>`, mapping
    /// nodes to their predecessors, where the predecessor to the source is `None`.
    /// `GraphError::NodeNotFound` is returned if the src node doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::*;
    /// let mut g: Graph<&str, i32> = graph_with_nodes!("Sydney", "Melbourne", "Perth");
    ///
    /// g.add_undirected_edge(&"Sydney", &"Melbourne", Some(7));
    /// g.add_undirected_edge(&"Melbourne", &"Perth", Some(5));
    /// g.add_undirected_edge(&"Sydney", &"Perth", Some(15));
    ///
    /// let res = g.djikstra(&"Sydney", 1, 0).unwrap();
    ///
    /// let (dist, pred) = res;
    ///
    /// assert_eq!(*dist.get(&"Melbourne").unwrap(), 7);
    /// assert_eq!(*dist.get(&"Perth").unwrap(), 12);
    ///
    /// assert_eq!(*pred.get(&"Sydney").unwrap(), None);
    /// assert_eq!(*pred.get(&"Melbourne").unwrap(), Some(&"Sydney"));
    /// assert_eq!(*pred.get(&"Perth").unwrap(), Some(&"Melbourne"));
    ///
    /// ```
    pub fn djikstra<'a>(
        &'a self,
        src: &'a N,
        default_weight: E,
        zero: E,
    ) -> Result<(HashMap<&'a N, E>, HashMap<&'a N, Option<&'a N>>), GraphError<'a, N>> {
        let mut dist: HashMap<&N, E> = HashMap::new();
        let mut pred: HashMap<&N, Option<&N>> = HashMap::new();

        let mut pq = std::collections::BinaryHeap::new();
        pred.insert(src, None);
        dist.insert(src, zero.clone());
        pq.push((Reverse(zero), src));

        while let Some((Reverse(curr_dist), u)) = pq.pop() {
            if dist.get(u).is_some() && *dist.get(u).unwrap() < curr_dist {
                continue;
            }

            let u_edges = match self.edges.get(u) {
                Some(set) => set,
                None => return Err(GraphError::NodeNotFound(u)),
            };

            for (n, e) in u_edges {
                let weight = match e {
                    Some(x) => x.clone(),
                    None => default_weight.clone(),
                };

                let new_dist = weight + curr_dist.clone();

                if dist.get(&**n).is_none() || new_dist < *dist.get(&**n).unwrap() {
                    dist.insert(n, new_dist.clone());
                    pred.insert(n, Some(u));
                    pq.push((Reverse(new_dist), n))
                }
            }
        }

        Ok((dist, pred))
    }
}
