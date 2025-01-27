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
