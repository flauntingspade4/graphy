use std::{convert::Infallible, rc::Rc};

use crate::{edge::EdgeTrait, ghost::GhostToken, id::EdgeId, Node, VertexId};

/// An undirected edge between two [vertices](crate::Vertex)
#[derive(Debug)]
pub struct UnDirectedUnWeightedEdge<'id, Item>(Rc<Node<'id, Item, (), Self>>, Rc<Node<'id, Item, (), Self>>);

impl<'id, Item> Clone for UnDirectedUnWeightedEdge<'id, Item> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.0.clone())
    }
}

impl<'id, Item> EdgeTrait<'id, Item, ()> for UnDirectedUnWeightedEdge<'id, Item> {
    type Error = Infallible;

    fn add_edge<'new_id>(
        _weight: (),
        first: &Rc<Node<'id, Item, (), Self>>,
        second: &Rc<Node<'id, Item, (), Self>>,
        id: EdgeId<'id>,
        token: &'new_id mut GhostToken<'id>,
    ) -> Result<(), Self::Error> {
        let edge = Self(first.clone(), second.clone());

        first.g_borrow_mut(token).edges.insert(id, edge.clone());
        second.g_borrow_mut(token).edges.insert(id, edge);

        Ok(())
    }
    fn other<'new_id>(
        &'new_id self,
        id: VertexId<'id>,
        token: &'new_id GhostToken<'id>,
    ) -> Option<&Rc<Node<'id, Item, (), Self>>> {
        if id == self.0.g_borrow(token).id {
            Some(&self.1)
        } else if id == self.1.g_borrow(token).id {
            Some(&self.0)
        } else {
            None
        }
    }
}
