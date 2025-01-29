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
    let mut cons = g.edges(&1).expect("We know that this node exists.");

    // Order doesn't matter
    cons.sort();

    assert_eq!(expect, cons);
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
    let mut cons = g.connections(&1).expect("We know that this node exists.");

    // Order doesn't matter
    cons.sort();

    assert_eq!(expect, cons);
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
