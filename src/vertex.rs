use core::marker::PhantomData;

use hashbrown::HashMap;

use crate::{edge::EdgeTrait, id::EdgeId, Shared, VertexId};

/// Represents a vertex in a graph. Vertices can hold data,
/// but are usually only useful in relation to other
/// vertices
#[derive(Debug)]
pub struct Vertex<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> {
    pub(crate) id: VertexId<'id>,
    pub(crate) edges: HashMap<EdgeId<'id>, Shared<'id, Edge>>,
    item: Item,
    _phantom: &'id PhantomData<Weight>,
}

impl<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> Vertex<'id, Item, Weight, Edge> {
    /// Creates a new [`Vertex`] with the given `id`
    #[must_use]
    pub(crate) fn new(id: usize, item: Item) -> Self {
        Self {
            id: VertexId::new(id),
            edges: HashMap::new(),
            item,
            _phantom: &PhantomData,
        }
    }
    pub fn id(&self) -> VertexId<'id> {
        self.id
    }
    /// Gets a reference to `self`'s inner item
    pub fn get_item(&self) -> &Item {
        &self.item
    }
    /// Gets a mutable reference to `self`'s inner item
    pub fn get_item_mut(&mut self) -> &mut Item {
        &mut self.item
    }
    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, EdgeId<'id>, Shared<'id, Edge>> {
        self.edges.iter()
    }
    #[must_use]
    pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<'_, EdgeId<'id>, Shared<'id, Edge>> {
        self.edges.iter_mut()
    }
}
