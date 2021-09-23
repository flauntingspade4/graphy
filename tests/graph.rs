use graph::{
    edge::{EdgeTrait, UnDirectedWeightedEdge},
    ghost::GhostToken,
    Graph, Node,
};

#[test]
fn make_empty() {
    let graph: Graph<(), (), UnDirectedWeightedEdge<_, _>> = Graph::new();

    assert!(graph.is_empty());
}

#[test]
fn add_one() {
    let mut graph: Graph<(), (), UnDirectedWeightedEdge<_, _>> = Graph::new();

    let id = graph.add_vertex(());

    assert_eq!(id.id(), 0);
    assert_eq!(graph.vertex_len(), 1);
}

#[test]
#[cfg_attr(miri, ignore)]
fn add_many() {
    let mut graph: Graph<(), (), UnDirectedWeightedEdge<_, _>> = Graph::new();

    let x = 999_999;

    for _ in 0..x {
        graph.add_vertex(());
    }

    let id = graph.add_vertex(());

    assert_eq!(id.id(), x);
    assert_eq!(graph.vertex_len(), x + 1);
}

#[test]
fn add_edge() {
    GhostToken::new(|mut t| {
        let mut graph: Graph<usize, f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let first = graph.add_vertex(1);

        let second = graph.add_vertex(2);

        let third = graph.add_vertex(3);

        let weight = 1.;

        graph.add_edge(first, second, weight, &mut t).unwrap();

        graph.add_edge(second, third, weight, &mut t).unwrap();

        assert_eq!(graph.get_vertex(first).unwrap().borrow(&t).edges().len(), 1);
        assert_eq!(
            graph.get_vertex(second).unwrap().borrow(&t).edges().len(),
            2
        );
        assert_eq!(graph.get_vertex(third).unwrap().borrow(&t).edges().len(), 1);
    });
}

#[test]
fn remove_edge_between() {
    GhostToken::new(|mut t| {
        let mut graph: Graph<(), f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let first = graph.add_vertex(());

        let second = graph.add_vertex(());

        let third = graph.add_vertex(());

        let weight = 1.;

        graph.add_edge(first, second, weight, &mut t).unwrap();

        graph.add_edge(second, third, weight, &mut t).unwrap();

        graph.remove_edge_between(first, second, &mut t).unwrap();

        assert_eq!(graph.get_vertex(first).unwrap().borrow(&t).edges().len(), 0);
        assert_eq!(
            graph.get_vertex(second).unwrap().borrow(&t).edges().len(),
            1
        );
        assert_eq!(graph.get_vertex(third).unwrap().borrow(&t).edges().len(), 1);
    })
}

#[test]
fn remove() {
    GhostToken::new(|mut t| {
        let mut graph: Graph<(), f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let first = graph.add_vertex(());

        let second = graph.add_vertex(());

        let third = graph.add_vertex(());

        let weight = 1.;

        graph.add_edge(first, second, weight, &mut t).unwrap();

        graph.add_edge(second, third, weight, &mut t).unwrap();

        graph.remove(second, &mut t).unwrap();

        assert!(graph.get_vertex(second).is_none());
        assert_eq!(graph.get_vertex(first).unwrap().borrow(&t).edges().len(), 0);
        assert_eq!(graph.get_vertex(third).unwrap().borrow(&t).edges().len(), 0);
    })
}

// Just makes sure that VertexId and EdgeId don't conflict
#[test]
fn id_out_of_order() {
    GhostToken::new(|mut t| {
        let mut graph: Graph<(), f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let weight = 1.;

        let first = graph.add_vertex(());

        let second = graph.add_vertex(());

        graph.add_edge(first, second, weight, &mut t).unwrap();

        graph.remove(second, &mut t).unwrap();

        let third = graph.add_vertex(());

        assert!(graph.get_vertex(second).is_none());
        assert_eq!(third.id(), 2);
    });
}

#[test]
fn adjacent() {
    GhostToken::new(|mut f| {
        let mut graph: Graph<(), f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let one = graph.add_vertex(());

        let two = graph.add_vertex(());

        graph.add_edge(one, two, 1., &mut f).unwrap();

        assert!(graph.adjacent(one, two, &f).unwrap());
    });
}

#[test]
fn edges_mut() {
    GhostToken::new(|mut t| {
        let mut graph: Graph<(), f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let x = 7.;

        let one = graph.add_vertex(());

        let two = graph.add_vertex(());

        graph.add_edge(one, two, x, &mut t).unwrap();

        for (_, edge) in graph
            .get_vertex(one)
            .unwrap()
            .borrow_mut(&mut t)
            .edges_mut()
        {
            *edge.get_weight_mut() *= 5.;
        }

        graph
            .get_vertex(one)
            .unwrap()
            .borrow(&t)
            .edges()
            .for_each(|(_, e)| assert_eq!(x * 5., *e.borrow(&t).get_weight()))
    });
}

#[test]
fn distance() {
    GhostToken::new(|mut t| {
        let mut graph: Graph<(f64, f64), f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let one = graph.add_vertex((0., 0.));

        let two = graph.add_vertex((1., 0.));

        let weight = {
            let a = graph.get_vertex(one).unwrap().borrow(&t).get_item();
            let b = graph.get_vertex(two).unwrap().borrow(&t).get_item();

            ((a.0 + b.0) * (a.0 + b.0) + (a.1 + b.1) * (a.1 + b.1)).sqrt()
        };

        graph.add_edge(one, two, weight, &mut t).unwrap();

        let distance = graph
            .get_vertex(one)
            .unwrap()
            .borrow(&t)
            .edges()
            .next()
            .unwrap()
            .1
            .borrow(&t)
            .get_weight();

        assert_eq!(1., *distance);
    });
}
