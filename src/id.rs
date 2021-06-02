#![allow(clippy::module_name_repetitions)]

use crate::ghost::InvariantLifetime;

/// The type describing a [`Vertex`](crate::Vertex)'s
/// index within a [`Graph`](crate::Graph)
///
/// While all vertices added in a graph are in order,
/// removing a vertex from the graph will not change
/// the next generated id, leading to possible errors like
/// ```rust
/// use graph::{edge::UnDirectedWeightedEdge, Graph, VertexId, ghost::GhostToken};
///
/// GhostToken::new(|mut t| {
///     let mut graph: Graph<(), (), UnDirectedWeightedEdge<_, _>> = Graph::new();
///
///     let first = graph.add_vertex(());
///     let second = graph.add_vertex(());
///
///     assert_eq!(first.id(), 0);
///
///     graph.remove(second, &mut t);
///
///     let third = graph.add_vertex(());
///
///     assert!(graph.get(VertexId::new(1)).is_none());
/// })
/// ```
#[derive(Clone, Copy, Hash)]
pub struct VertexId<'id> {
    id: usize,
    _marker: InvariantLifetime<'id>,
}

impl<'id> VertexId<'id> {
    /// Constructs a new [`VertexId`] with a given id
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self {
            id,
            _marker: InvariantLifetime::new(),
        }
    }
    /// Returns the internal `id`
    #[must_use]
    pub const fn id(self) -> usize {
        self.id
    }
}

impl<'id> PartialEq for VertexId<'id> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<'id> Eq for VertexId<'id> {}

impl<'id> core::fmt::Debug for VertexId<'id> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// The type describing a [`Edge`] within a [`Graph`].
/// Seperate from [`VertexId`] so a user doesn't have to
/// think about the id of their vertices being out of order
#[derive(Clone, Copy, Hash)]
pub struct EdgeId<'id> {
    pub id: usize,
    _marker: InvariantLifetime<'id>,
}

impl<'id> EdgeId<'id> {
    /// Constructs a new [`Id`] with a given id
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self {
            id,
            _marker: InvariantLifetime::new(),
        }
    }
}

impl<'id> PartialEq for EdgeId<'id> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<'id> Eq for EdgeId<'id> {}

impl<'id> core::fmt::Debug for EdgeId<'id> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.id)
    }
}
