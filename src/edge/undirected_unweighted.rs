use core::convert::Infallible;

use crate::{edge::EdgeTrait, ghost::GhostToken, id::EdgeId, SharedNode, VertexId};

/// An undirected edge between two [vertices](crate::Vertex)
#[derive(Debug)]
pub struct UnDirectedUnWeightedEdge<'id, Item>(
    SharedNode<'id, Item, (), Self>,
    SharedNode<'id, Item, (), Self>,
);

impl<'id, Item> Clone for UnDirectedUnWeightedEdge<'id, Item> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<'id, Item> EdgeTrait<'id, Item, ()> for UnDirectedUnWeightedEdge<'id, Item> {
    type Error = Infallible;

    fn add_edge<'new_id>(
        _weight: (),
        first: &SharedNode<'id, Item, (), Self>,
        second: &SharedNode<'id, Item, (), Self>,
        id: EdgeId<'id>,
        token: &'new_id mut GhostToken<'id>,
    ) -> Result<(), Self::Error> {
        let edge = Self(first.clone(), second.clone());

        first.borrow_mut(token).edges_mut().insert(id, edge.clone());

        second.borrow_mut(token).edges_mut().insert(id, edge);

        Ok(())
    }
    fn other<'new_id>(
        &'new_id self,
        id: VertexId<'id>,
        token: &'new_id GhostToken<'id>,
    ) -> Option<&SharedNode<'id, Item, (), Self>> {
        if id == self.0.borrow(token).id() {
            Some(&self.1)
        } else if id == self.1.borrow(token).id() {
            Some(&self.0)
        } else {
            None
        }
    }
}
