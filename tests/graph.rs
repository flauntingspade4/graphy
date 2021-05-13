use graph::{ghost::GhostToken, Graph};

#[test]
fn make_empty() {
    let graph = Graph::new();

    assert!(graph.is_empty());
}

#[test]
fn add_one() {
    let mut graph = Graph::new();

    let id = graph.add_vertex();

    assert_eq!(id.id(), 0);
    assert_eq!(graph.len(), 1);
}

#[test]
fn add_many() {
    let mut graph = Graph::new();

    let x = 999_999;

    for _ in 0..x {
        graph.add_vertex();
    }

    let id = graph.add_vertex();

    assert_eq!(id.id(), x);
    assert_eq!(graph.len(), x + 1);
}

#[test]
fn add_edge() {
    GhostToken::new(|mut token| {
        let mut graph = Graph::new();

        let first = graph.add_vertex();

        let second = graph.add_vertex();

        let third = graph.add_vertex();

        let weight = 1.;

        graph
            .add_edge(first, second, weight, &mut token)
            .unwrap();

        graph
            .add_edge(second, third, weight, &mut token)
            .unwrap();

        let vertex = graph.get(second).unwrap();

        let vertex = vertex.ghost_borrow(&token);

        println!("{:?}", vertex.edges());
    });
}

#[test]
fn remove() {
    GhostToken::new(|mut t| {
        let mut graph = Graph::new();

        let first = graph.add_vertex();

        let second = graph.add_vertex();

        let third = graph.add_vertex();

        let weight = 1.;

        graph.add_edge(first, second, weight, &mut t).unwrap();

        graph.add_edge(second, third, weight, &mut t).unwrap();

        graph.remove(second, &mut t).unwrap();

        assert!(graph.get(second).is_none());
        assert_eq!(graph.get(first).unwrap().ghost_borrow(&t).edges().len(), 0);
        assert_eq!(graph.get(third).unwrap().ghost_borrow(&t).edges().len(), 0);
    })
}
