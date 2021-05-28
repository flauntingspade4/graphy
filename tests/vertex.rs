use graph::{edge::UnDirectedWeightedEdge, ghost::GhostToken, Graph};

#[test]
fn make_empty() {
    GhostToken::new(|t| {
        let mut graph: Graph<f64, (), UnDirectedWeightedEdge<f64, ()>> = Graph::new();

        let first_item = 15.7;

        let token = graph.add_vertex(first_item);

        let item = graph.get(token).unwrap();

        assert_eq!(*item.g_borrow(&t).get_item(), first_item);
    });
}