use graph::{
    edge::{EdgeTrait, UnDirectedWeightedEdge},
    ghost::GhostToken,
    Graph,
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
    assert_eq!(graph.len(), 1);
}

#[test]
fn add_many() {
    let mut graph: Graph<(), (), UnDirectedWeightedEdge<_, _>> = Graph::new();

    let x = 999_999;

    for _ in 0..x {
        graph.add_vertex(());
    }

    let id = graph.add_vertex(());

    assert_eq!(id.id(), x);
    assert_eq!(graph.len(), x + 1);
}

#[test]
fn add_edge() {
    GhostToken::new(|mut t| {
        let mut graph: Graph<usize, f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let first = graph.add_vertex(1);

        let second = graph.add_vertex(2);

        let third = graph.add_vertex(3);

        let weight = 1.;

        graph
            .add_edge(first, second, (), |_, _, _, _, _| weight, &mut t)
            .unwrap();

        graph
            .add_edge(second, third, (), |_, _, _, _, _| weight, &mut t)
            .unwrap();

        assert_eq!(graph.get(first).unwrap().borrow(&t).iter().len(), 1);
        assert_eq!(graph.get(second).unwrap().borrow(&t).iter().len(), 2);
        assert_eq!(graph.get(third).unwrap().borrow(&t).iter().len(), 1);
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

        graph
            .add_edge(first, second, (), |_, _, _, _, _| weight, &mut t)
            .unwrap();

        graph
            .add_edge(second, third, (), |_, _, _, _, _| weight, &mut t)
            .unwrap();

        graph.remove_edge_between(first, second, &mut t).unwrap();

        assert_eq!(graph.get(first).unwrap().borrow(&t).iter().len(), 0);
        assert_eq!(graph.get(second).unwrap().borrow(&t).iter().len(), 1);
        assert_eq!(graph.get(third).unwrap().borrow(&t).iter().len(), 1);
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

        graph
            .add_edge(first, second, (), |_, _, _, _, _| weight, &mut t)
            .unwrap();

        graph
            .add_edge(second, third, (), |_, _, _, _, _| weight, &mut t)
            .unwrap();

        graph.remove(second, &mut t).unwrap();

        assert!(graph.get(second).is_none());
        assert_eq!(graph.get(first).unwrap().borrow(&t).iter().len(), 0);
        assert_eq!(graph.get(third).unwrap().borrow(&t).iter().len(), 0);
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

        graph
            .add_edge(first, second, (), |_, _, _, _, _| weight, &mut t)
            .unwrap();

        graph.remove(second, &mut t).unwrap();

        let third = graph.add_vertex(());

        assert!(graph.get(second).is_none());
        assert_eq!(third.id(), 2);
    });
}

#[test]
fn adjacent() {
    GhostToken::new(|mut f| {
        let mut graph: Graph<(), f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let one = graph.add_vertex(());

        let two = graph.add_vertex(());

        graph
            .add_edge(one, two, (), |_, _, _, _, _| 1., &mut f)
            .unwrap();

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

        graph
            .add_edge(one, two, (), |_, _, _, _, _| x, &mut t)
            .unwrap();

        graph
            .get(one)
            .unwrap()
            .borrow_mut(&mut t)
            .iter_mut()
            .for_each(|(_, t)| *t.get_weight_mut() *= 5.);

        graph
            .get(one)
            .unwrap()
            .borrow(&t)
            .iter()
            .for_each(|(_, t)| assert_eq!(x * 5., *t.get_weight()))
    });
}

#[test]
fn distance() {
    GhostToken::new(|mut t| {
        let mut graph: Graph<(f64, f64), f64, UnDirectedWeightedEdge<_, _>> = Graph::new();

        let one = graph.add_vertex((0., 0.));

        let two = graph.add_vertex((1., 0.));

        graph
            .add_edge(
                one,
                two,
                (),
                |_, a, b, _, token| {
                    let a = a.g_borrow(token).get_item();
                    let b = b.g_borrow(token).get_item();
                    ((a.0 + b.0) * (a.0 + b.0) + (a.1 + b.1) * (a.1 + b.1)).sqrt()
                },
                &mut t,
            )
            .unwrap();

        let distance = graph
            .get(one)
            .unwrap()
            .borrow(&t)
            .iter()
            .next()
            .unwrap()
            .1
            .get_weight();

        assert_eq!(1., *distance);
    });
}
