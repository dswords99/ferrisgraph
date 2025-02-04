use super::graph::GraphError;
use super::Graph;
use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;
use std::rc::Rc;

impl<N, E> Graph<N, E>
where
    N: Hash + Eq + Debug + Ord,
    E: Hash + Eq + Ord,
{
    /// This function performs Breadth First Search on the graph, starting from the given source node.
    /// The function returns the predecessors in the form `HashMap<&N, &N>`, where a given N will map
    /// to its predecessor node. A `GraphError::NodeNotFound` error will be returned if the source node
    /// does not exist in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
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
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
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
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
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

    fn explore_for_cycle(
        &self,
        node: &Rc<N>,
        visited: &mut BTreeSet<Rc<N>>,
        stack: &mut BTreeSet<Rc<N>>,
    ) -> bool {
        if stack.contains(node) {
            return true;
        }

        if visited.contains(node) {
            return false;
        }

        let edges = self
            .edges
            .get(node)
            .expect("There is no way that this isn't a node");
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
    /// use ferrisgraph::{graph::Graph, graph_with_nodes};
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
