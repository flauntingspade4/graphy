use graph::{ghost::GhostToken, Graph, Id};

#[test]
fn remove() {
    GhostToken::new(|mut t| {
        let start = std::time::Instant::now();

        let mut graph = Graph::new();

        let weight = 1.;

        for i in 5..999_999 {
            let index = graph.add_vertex();

            'inner: for j in -5..5 {
                if i + j > 0 {
                    let id = Id::new((i + j) as usize);

                    if graph.add_edge(index, id, weight, &mut t).is_err() {
                        break 'inner;
                    }
                }
            }
        }

        println!("Took {}ms", start.elapsed().as_millis());

        graph.clear(&mut t);

        println!("Took {}ms", start.elapsed().as_millis());
    })
}
