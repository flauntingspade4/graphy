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

    assert_eq!(id, 0);
    assert_eq!(graph.len(), 1);
}

#[test]
fn add_100() {
    let mut graph = Graph::new();

    for _ in 0..99 {
        graph.add_vertex();
    }

    let id = graph.add_vertex();

    assert_eq!(id, 99);
    assert_eq!(graph.len(), 100);
}

#[test]
fn add_edge() {
    GhostToken::new(move |mut token| {
        let mut graph = Graph::new();

        let first = graph.add_vertex();

        let second = graph.add_vertex();

        let weight = 1.;

        {
            graph.add_edge(first, second, weight, &mut token).unwrap();
        }

        let first = graph.get(0, &token).unwrap().borrow(&token);

        println!("{:?}", first.neighbors());
    });
}
