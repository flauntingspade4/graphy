#![allow(clippy::module_name_repetitions)]

mod undirected_unweighted;
mod undirected_weighted;

use crate::{ghost::GhostToken, id::EdgeId, SharedNode, VertexId};

pub use undirected_unweighted::UnDirectedUnWeightedEdge;
pub use undirected_weighted::UnDirectedWeightedEdge;

/// A graph can add edges between [`Vertices`](crate::Vertex) of any
/// type that implements [`EdgeTrait`]
pub trait EdgeTrait<'id, Item, Weight>: Sized {
    type Error;

    /// Adds an edge between `first` and `second`,
    /// with the given weight
    /// # Errors
    /// Returns [`Self::Error`] if adding an edge
    /// fails
    fn add_edge<'new_id>(
        weight: Weight,
        first: &SharedNode<'id, Item, Weight, Self>,
        second: &SharedNode<'id, Item, Weight, Self>,
        id: EdgeId<'id>,
        token: &'new_id mut GhostToken<'id>,
    ) -> Result<(), Self::Error>;
    /// Returns the other [`Vertex`](crate::Vertex) in `self`
    /// # Errors
    /// Returns `None` if the provided [`VertexId`] doesn't
    /// relate to either [`Vertex`](crate::Vertex) in `self`
    fn other<'new_id>(
        &'new_id self,
        id: VertexId<'id>,
        token: &'new_id GhostToken<'id>,
    ) -> Option<&SharedNode<'id, Item, Weight, Self>>;
}
