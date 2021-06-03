use core::convert::Infallible;

use crate::{ghost::GhostToken, id::EdgeId, Shared, SharedNode, VertexId};

use super::EdgeTrait;

/// An undirected edge between two [vertices](crate::Vertex), with a given weight
#[derive(Debug)]
pub struct DirectedWeightedEdge<'id, Item, Weight>(
    pub Weight,
    SharedNode<'id, Item, Weight, Self>,
    SharedNode<'id, Item, Weight, Self>,
);

impl<'id, Item, Weight> DirectedWeightedEdge<'id, Item, Weight> {
    /// Returns the 'sender' in the edge
    pub fn sender(&self) -> &SharedNode<'id, Item, Weight, Self> {
        &self.1
    }
    /// Returns the 'receiver' in the edge
    pub fn receiver(&self) -> &SharedNode<'id, Item, Weight, Self> {
        &self.2
    }
}

impl<'id, Item, Weight> EdgeTrait<'id, Item, Weight>
    for DirectedWeightedEdge<'id, Item, Weight>
{
    type Error = Infallible;

    fn add_edge<'new_id>(
        weight: Weight,
        first: &SharedNode<'id, Item, Weight, Self>,
        second: &SharedNode<'id, Item, Weight, Self>,
        id: EdgeId<'id>,
        token: &'new_id mut GhostToken<'id>,
    ) -> Result<(), Self::Error> {
        let edge = Shared::new(Self(weight, first.clone(), second.clone()));

        first.borrow_mut(token).edges.insert(id, edge.clone());
        second.borrow_mut(token).edges.insert(id, edge);

        Ok(())
    }
    fn other<'new_id>(
        &'new_id self,
        id: VertexId<'id>,
        token: &'new_id GhostToken<'id>,
    ) -> Option<&SharedNode<'id, Item, Weight, Self>> {
        if id == self.1.borrow(token).id() {
            Some(&self.2)
        } else if id == self.2.borrow(token).id() {
            Some(&self.1)
        } else {
            None
        }
    }

    fn get_weight(&self) -> &Weight {
        &self.0
    }

    fn get_weight_mut(&mut self) -> &mut Weight {
        &mut self.0
    }
}
