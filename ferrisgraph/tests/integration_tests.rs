use ferrisgraph::Graph;

#[test]
fn test_node_insertion_and_contains() {
    let mut g: Graph<i32, i32> = Graph::new();

    assert_eq!(g.is_node(1), false);

    assert!(g.add_node(1));
    assert!(g.is_node(1));

    assert_eq!(g.add_node(1), false);
}