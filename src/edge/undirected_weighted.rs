use core::convert::Infallible;

use crate::{edge::EdgeTrait, ghost::GhostToken, id::EdgeId, SharedNode, VertexId};

use core::fmt::Debug;

/// An undirected edge between two [vertices](crate::Vertex), with a given weight
#[derive(Debug)]
pub struct UnDirectedWeightedEdge<'id, Item: Debug, Weight: Clone + Debug>(
    pub Weight,
    SharedNode<'id, Item, Weight, Self>,
    SharedNode<'id, Item, Weight, Self>,
);

impl<'id, Item: Debug, Weight: Clone + Debug> Clone for UnDirectedWeightedEdge<'id, Item, Weight> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<'id, Item: Debug, Weight: Clone + Debug> EdgeTrait<'id, Item, Weight>
    for UnDirectedWeightedEdge<'id, Item, Weight>
{
    type Error = Infallible;

    fn add_edge<'new_id>(
        weight: Weight,
        first: &SharedNode<'id, Item, Weight, Self>,
        second: &SharedNode<'id, Item, Weight, Self>,
        id: EdgeId<'id>,
        token: &'new_id mut GhostToken<'id>,
    ) -> Result<(), Self::Error> {
        let edge = Self(weight, first.clone(), second.clone());

        first.borrow_mut(token).edges_mut().insert(id, edge.clone());
        second.borrow_mut(token).edges_mut().insert(id, edge);

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
}
