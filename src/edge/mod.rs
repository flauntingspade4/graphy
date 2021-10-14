#![allow(clippy::module_name_repetitions)]

mod directed_weighted;
mod undirected_weighted;

use crate::{ghost::GhostToken, id::EdgeId, Graph, SharedNode, VertexId};

pub use directed_weighted::DirectedWeightedEdge;

pub use undirected_weighted::UnDirectedWeightedEdge;

/// An undirected edge between two [vertices](crate::Vertex) with
/// no weight
pub type UnDirectedUnWeightedEdge<'id, Item> = UnDirectedWeightedEdge<'id, Item, ()>;

/// A graph can add edges between [`Vertices`](crate::Vertex) of any
/// type that implements [`EdgeTrait`]
pub trait EdgeTrait<'id, Item, Weight>: Sized {
    type Error;

    /// Adds an edge between `first`, `second`
    /// and the graph, with the given weight
    ///
    /// # Errors
    /// Returns [`Self::Error`] if adding an edge
    /// fails
    fn add_edge<'new_id>(
        weight: Weight,
        first: &SharedNode<'id, Item, Weight, Self>,
        second: &SharedNode<'id, Item, Weight, Self>,
        id: EdgeId<'id>,
        graph: &mut Graph<'id, Item, Weight, Self>,
        token: &'new_id mut GhostToken<'id>,
    ) -> Result<(), Self::Error>;
    /// Returns the other [`Vertex`](crate::Vertex) in `self`
    ///
    /// # Errors
    /// Returns `None` if the provided [`VertexId`] doesn't
    /// relate to either [`Vertex`](crate::Vertex) in `self`
    fn other<'new_id>(
        &'new_id self,
        id: VertexId<'id>,
        token: &'new_id GhostToken<'id>,
    ) -> Option<&SharedNode<'id, Item, Weight, Self>>;

    fn get_weight(&self) -> &Weight;

    fn get_weight_mut(&mut self) -> &mut Weight;

    fn connects(
        &self,
        first: &SharedNode<'id, Item, Weight, Self>,
        second: &SharedNode<'id, Item, Weight, Self>,
    ) -> bool;
}
