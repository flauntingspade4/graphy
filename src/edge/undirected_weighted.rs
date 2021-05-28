use std::{convert::Infallible, rc::Rc};

use crate::{edge::EdgeTrait, ghost::GhostToken, id::EdgeId, Node, VertexId};

/// An undirected edge between two [vertices](crate::Vertex), with a given weight
#[derive(Debug)]
pub struct UnDirectedWeightedEdge<'id, Item, Weight: Clone>(
    pub Weight,
    Rc<Node<'id, Item, Weight, Self>>,
    Rc<Node<'id, Item, Weight, Self>>,
);

impl<'id, Item, Weight: Clone> Clone for UnDirectedWeightedEdge<'id, Item, Weight> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl<'id, Item, Weight: Clone> EdgeTrait<'id, Item, Weight>
    for UnDirectedWeightedEdge<'id, Item, Weight>
{
    type Error = Infallible;

    fn add_edge<'new_id>(
        weight: Weight,
        first: &Rc<Node<'id, Item, Weight, Self>>,
        second: &Rc<Node<'id, Item, Weight, Self>>,
        id: EdgeId<'id>,
        token: &'new_id mut GhostToken<'id>,
    ) -> Result<(), Self::Error> {
        let edge = Self(weight, first.clone(), second.clone());

        first.g_borrow_mut(token).edges.insert(id, edge.clone());
        second.g_borrow_mut(token).edges.insert(id, edge);

        Ok(())
    }
    fn other<'new_id>(
        &'new_id self,
        id: VertexId<'id>,
        token: &'new_id GhostToken<'id>,
    ) -> Option<&Rc<Node<'id, Item, Weight, Self>>> {
        if id == self.1.g_borrow(token).id {
            Some(&self.2)
        } else if id == self.2.g_borrow(token).id {
            Some(&self.1)
        } else {
            None
        }
    }
}