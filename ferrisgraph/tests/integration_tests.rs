use std::collections::BTreeSet;

use ferrisgraph::{graph_with_nodes, Graph};

#[test]
fn test_node_insertion_and_contains() {
    let mut g: Graph<i32, i32> = Graph::new();

    assert!(g.is_empty());

    assert_eq!(g.is_node(&1), false);

    assert!(g.add_node(1));
    assert!(g.is_node(&1));

    assert_eq!(g.add_node(1), false);

    assert_eq!(g.is_empty(), false);
}

#[test]
fn test_nodes_method() {
    let g: Graph<&str, i32> = graph_with_nodes!("Sydney", "Melbourne", "Brisbane");

    let nodes = g.nodes();

    assert!(nodes.contains(&"Sydney"));

    let melb = nodes.get(&"Melbourne");

    assert!(melb.is_some());

    assert_eq!(**melb.unwrap(), "Melbourne");
}

#[test]
fn test_edge_insertion() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    assert!(g.add_edge(&1, &2, Some(1000)));
    assert_eq!(g.add_edge(&1, &2, Some(1000)), false);

    assert!(g.is_edge(&1, &2, &Some(1000)));
    assert_eq!(g.is_edge(&1, &2, &None), false);
    assert_eq!(g.is_edge(&3, &4, &Some(1000)), false);
}

#[test]
fn test_node_removal() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    g.add_edge(&1, &2, None);
    g.add_edge(&1, &3, None);
    g.add_edge(&3, &1, None);
    g.add_edge(&4, &1, None);
    g.add_edge(&3, &2, None);

    assert!(g.remove_node(&1));
    assert_eq!(g.remove_node(&1), false);

    assert_eq!(g.is_node(&1), false);

    // Ensure edges were removed as expected
    assert!(g.is_edge(&3, &2, &None));
    assert_eq!(g.is_edge(&1, &2, &None), false);
    assert_eq!(g.is_edge(&1, &3, &None), false);
    assert_eq!(g.is_edge(&3, &1, &None), false);
    assert_eq!(g.is_edge(&4, &1, &None), false);
}

#[test]
fn test_edge_removal() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    g.add_edge(&1, &2, None);
    g.add_edge(&1, &3, None);

    assert!(g.remove_edge(&1, &2, None));
    assert_eq!(g.remove_edge(&1, &2, None), false);

    assert!(g.is_edge(&1, &3, &None));
    assert_eq!(g.is_edge(&1, &2, &None), false);
}

#[test]
fn test_edges() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    g.add_edge(&1, &2, None);
    g.add_edge(&1, &3, Some(100));
    g.add_edge(&1, &5, Some(1001));

    let expect = vec![(&2, &None), (&3, &Some(100)), (&5, &Some(1001))];
    let cons = g.edges(&1);

    assert!(cons.is_ok());
    let cons = cons.unwrap();
    assert!(cons.is_some());
    let mut cons = cons.unwrap();

    // Order doesn't matter
    cons.sort();

    assert_eq!(expect, cons);

    assert!(g.edges(&6).is_err());
}

#[test]
fn test_is_connected() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    assert_eq!(g.is_connected(&1, &2), false);
    assert_eq!(g.is_connected(&1, &3), false);

    g.add_edge(&1, &2, None);
    g.add_edge(&1, &3, Some(10));

    assert!(g.is_connected(&1, &2));
    assert!(g.is_connected(&1, &3));
    assert_eq!(g.is_connected(&1, &5), false);

    g.remove_edge(&1, &2, None);

    assert_eq!(g.is_connected(&1, &2), false);
}

#[test]
fn test_connections() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    g.add_edge(&1, &2, None);
    g.add_edge(&1, &3, Some(100));
    g.add_edge(&1, &5, Some(1001));
    g.add_edge(&4, &1, None);

    let expect = vec![&2, &3, &5];
    let cons = g.connections(&1);

    assert!(cons.is_ok());
    let cons = cons.unwrap();
    assert!(cons.is_some());
    let mut cons = cons.unwrap();

    // Order doesn't matter
    cons.sort();

    assert_eq!(expect, cons);

    assert!(g.connections(&6).is_err());
}

#[test]
fn test_num_edges() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    assert_eq!(g.num_edges(), 0);

    g.add_edge(&1, &2, None);
    g.add_edge(&1, &3, Some(100));

    assert_eq!(g.num_edges(), 2);

    g.add_edge(&1, &5, Some(1001));
    g.add_edge(&4, &1, None);

    assert_eq!(g.num_edges(), 4);
}

#[test]
fn test_degree() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    assert_eq!(g.in_degree(&1), 0);
    assert_eq!(g.out_degree(&1), 0);

    g.add_edge(&1, &3, Some(100));
    g.add_edge(&1, &5, Some(1001));
    g.add_edge(&4, &1, None);

    assert_eq!(g.in_degree(&1), 1);
    assert_eq!(g.out_degree(&1), 2);
    assert_eq!(g.in_degree(&4), 0);
    assert_eq!(g.out_degree(&4), 1);

    assert_eq!(g.degree(&1), 3);
    assert_eq!(g.degree(&4), 1);
}

#[test]
fn test_bfs() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    g.add_edge(&1, &2, None);
    g.add_edge(&1, &3, None);
    g.add_edge(&2, &5, None);
    g.add_edge(&5, &5, None);

    let res = g.bfs(&1);
    assert!(res.is_ok());

    let pred = res.unwrap();
    assert_eq!(pred.len(), 4);
    assert_eq!(**pred.get(&1).unwrap(), 1);
    assert_eq!(**pred.get(&2).unwrap(), 1);
    assert_eq!(**pred.get(&3).unwrap(), 1);
    assert_eq!(**pred.get(&5).unwrap(), 2);
}

#[test]
fn test_complex_bfs() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);

    g.add_edge(&0, &1, None);
    g.add_edge(&1, &0, None);

    g.add_edge(&0, &2, None);
    g.add_edge(&2, &0, None);

    g.add_edge(&0, &5, None);
    g.add_edge(&5, &0, None);

    g.add_edge(&2, &3, None);
    g.add_edge(&3, &2, None);

    g.add_edge(&1, &5, None);
    g.add_edge(&5, &1, None);

    g.add_edge(&3, &5, None);
    g.add_edge(&5, &3, None);

    g.add_edge(&4, &5, None);
    g.add_edge(&5, &4, None);

    g.add_edge(&3, &4, None);
    g.add_edge(&4, &3, None);

    g.add_edge(&6, &5, None);
    g.add_edge(&5, &6, None);

    g.add_edge(&5, &7, None);
    g.add_edge(&7, &5, None);

    g.add_edge(&4, &7, None);
    g.add_edge(&7, &4, None);

    g.add_edge(&4, &8, None);
    g.add_edge(&8, &4, None);

    g.add_edge(&8, &7, None);
    g.add_edge(&7, &8, None);

    g.add_edge(&9, &7, None);
    g.add_edge(&7, &9, None);

    g.add_edge(&9, &8, None);
    g.add_edge(&8, &9, None);

    let res = g.bfs(&0);

    assert!(res.is_ok());

    let pred = res.unwrap();

    assert_eq!(pred.len(), 10);

    assert_eq!(**pred.get(&0).unwrap(), 0);
    assert_eq!(**pred.get(&1).unwrap(), 0);
    assert_eq!(**pred.get(&2).unwrap(), 0);
    assert_eq!(**pred.get(&3).unwrap(), 2);
    assert_eq!(**pred.get(&4).unwrap(), 5);
    assert_eq!(**pred.get(&5).unwrap(), 0);
    assert_eq!(**pred.get(&6).unwrap(), 5);
    assert_eq!(**pred.get(&7).unwrap(), 5);
    assert_eq!(**pred.get(&8).unwrap(), 4);
    assert_eq!(**pred.get(&9).unwrap(), 7);
}

#[test]
fn test_clone() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5);

    g.add_edge(&1, &2, None);

    let mut new_g: Graph<i32, i32> = graph_with_nodes!(0, -1, -2, -3);

    assert_ne!(g, new_g);

    new_g = g.clone();

    assert_eq!(new_g, g);
}

#[test]
fn test_dfs() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(1, 2, 3, 4, 5, 6, 7);

    g.add_edge(&1, &2, None);
    g.add_edge(&2, &3, None);
    g.add_edge(&3, &1, None);

    g.add_edge(&4, &5, None);
    g.add_edge(&4, &6, None);
    g.add_edge(&6, &7, None);

    let res = g.dfs(&1);

    assert!(res.is_ok());
    let res = res.unwrap();
    let expected: BTreeSet<&i32> = vec![&1, &2, &3].into_iter().collect();

    assert_eq!(res.len(), 3);
    assert_eq!(res, expected);

    let res = g.dfs(&4);
    assert!(res.is_ok());

    let res = res.unwrap();
    let expected: BTreeSet<&i32> = vec![&4, &5, &6, &7].into_iter().collect();

    assert_eq!(res, expected);

    let res = g.dfs(&0);
    assert!(res.is_err());
}

#[test]
fn test_djikstra() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(0, 1, 2, 3, 4, 5);

    g.add_undirected_edge(&0, &1, Some(14));
    g.add_undirected_edge(&0, &2, Some(9));
    g.add_undirected_edge(&0, &3, Some(7));

    g.add_undirected_edge(&1, &4, Some(5));

    g.add_undirected_edge(&2, &1, Some(4));
    g.add_undirected_edge(&2, &5, Some(3));
    g.add_undirected_edge(&2, &3, Some(10));

    g.add_undirected_edge(&3, &5, Some(15));

    g.add_undirected_edge(&4, &5, Some(8));

    let res = g.djikstra(&0, 1, 0);

    assert!(res.is_ok());

    let (dist, pred) = res.unwrap();

    assert_eq!(*dist.get(&0).unwrap(), 0);
    assert_eq!(*dist.get(&1).unwrap(), 13);
    assert_eq!(*dist.get(&2).unwrap(), 9);
    assert_eq!(*dist.get(&3).unwrap(), 7);
    assert_eq!(*dist.get(&4).unwrap(), 18);
    assert_eq!(*dist.get(&5).unwrap(), 12);

    assert_eq!(*pred.get(&0).unwrap(), None);
    assert_eq!(*pred.get(&1).unwrap(), Some(&2));
    assert_eq!(*pred.get(&2).unwrap(), Some(&0));
    assert_eq!(*pred.get(&3).unwrap(), Some(&0));
    assert_eq!(*pred.get(&4).unwrap(), Some(&1));
    assert_eq!(*pred.get(&5).unwrap(), Some(&2));
}

#[test]
fn test_has_cycle() {
    let mut g: Graph<i32, i32> = graph_with_nodes!(0, 1, 2, 3, 4, 5);

    g.add_edge(&0, &1, None);
    g.add_edge(&0, &2, None);

    assert!(!g.has_cycle());
    
    g.add_edge(&1, &2, None);

    assert!(!g.has_cycle());

    g.add_edge(&2, &0, None);

    assert!(g.has_cycle());

    let mut g: Graph<i32, i32> = graph_with_nodes!(0, 1, 2, 3, 4, 5);

    g.add_edge(&0, &1, None);
    g.add_edge(&0, &2, None);

    g.add_edge(&3, &4, None);
    g.add_edge(&4, &5, None);

    assert!(!g.has_cycle());

    g.add_edge(&5, &3, None);

    assert!(g.has_cycle());
}
