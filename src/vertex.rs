use core::marker::PhantomData;

use hashbrown::HashMap;

use crate::{edge::EdgeTrait, id::EdgeId, shared::Shared, Node, SharedNode, VertexId};

pub struct Vertices<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> {
    inner: HashMap<VertexId<'id>, SharedNode<'id, Item, Weight, Edge>>,
}

impl<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> Vertices<'id, Item, Weight, Edge> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    pub fn insert(&mut self, id: VertexId<'id>, vertex: Node<'id, Item, Weight, Edge>) {
        let vertex = Shared::new(vertex);

        self.inner.insert(id, vertex);
    }
    pub fn clear(&mut self) {
        self.inner.clear()
    }
    pub fn get(&self, id: VertexId<'id>) -> Option<&SharedNode<'id, Item, Weight, Edge>> {
        self.inner.get(&id)
    }
    pub fn iter(
        &self,
    ) -> hashbrown::hash_map::Iter<'_, VertexId<'id>, SharedNode<'id, Item, Weight, Edge>> {
        self.inner.iter()
    }
    #[must_use]
    pub fn iter_mut(
        &mut self,
    ) -> hashbrown::hash_map::IterMut<'_, VertexId<'id>, SharedNode<'id, Item, Weight, Edge>> {
        self.inner.iter_mut()
    }
    pub fn remove(&mut self, id: VertexId<'id>) -> Option<SharedNode<'id, Item, Weight, Edge>> {
        self.inner.remove(&id)
    }
}

/// Represents a vertex in a graph. Vertices holds data,
/// but are usually only useful in relation to other
/// vertices
#[derive(Debug)]
pub struct Vertex<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>>(
    VertexInner<'id, Item, Weight, Edge>,
);

impl<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> Vertex<'id, Item, Weight, Edge> {
    /// Creates a new [`Vertex`] with the given `id`
    #[must_use]
    pub(crate) fn new(id: usize, item: Item) -> Self {
        Self(VertexInner {
            id: VertexId::new(id),
            edges: HashMap::new(),
            item,
            _phantom: &PhantomData,
        })
    }
    pub(crate) fn edges(&self) -> &HashMap<EdgeId<'id>, Edge> {
        &self.0.edges
    }
    pub(crate) fn edges_mut(&mut self) -> &mut HashMap<EdgeId<'id>, Edge> {
        &mut self.0.edges
    }
    pub fn id(&self) -> VertexId<'id> {
        self.0.id
    }
    /// Gets a reference to `self`'s inner item
    pub fn get_item(&self) -> &Item {
        &self.0.item
    }
    /// Gets a mutable reference to `self`'s inner item
    pub fn get_item_mut(&mut self) -> &mut Item {
        &mut self.0.item
    }
    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, EdgeId<'id>, Edge> {
        self.0.edges.iter()
    }
    #[must_use]
    pub fn iter_mut(&mut self) -> hashbrown::hash_map::IterMut<'_, EdgeId<'id>, Edge> {
        self.0.edges.iter_mut()
    }
}

#[derive(Debug)]
struct VertexInner<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> {
    pub(crate) id: VertexId<'id>,
    pub(crate) edges: HashMap<EdgeId<'id>, Edge>,
    item: Item,
    _phantom: &'id PhantomData<Weight>,
}

/*
impl<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> VertexInner<'id, Item, Weight, Edge> {
    /// Returns an iterator over all the vertice's edges
    #[must_use]
    pub fn edges(&self) -> std::collections::hash_map::Iter<'_, EdgeId<'id>, Edge> {
        self.edges.iter()
    }
}
*/
