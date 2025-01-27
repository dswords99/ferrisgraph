#[macro_export]
macro_rules! graph_with_nodes {
    ($($node:expr),*) => {{
        let mut g = Graph::new();

        $(
            g.add_node($node);
        )*

        g
    }}
}
