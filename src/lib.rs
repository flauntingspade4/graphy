#![warn(clippy::pedantic, clippy::nursery)]
#![feature(const_raw_ptr_deref, const_mut_refs, option_result_unwrap_unchecked)]

pub mod edge;
pub mod ghost;
mod id;

use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use ghost::{GhostCell, GhostToken};

use edge::EdgeTrait;
use id::EdgeId;
pub use id::VertexId;

type Node<'id, Item, Weight, Edge> = GhostCell<'id, Vertex<'id, Item, Weight, Edge>>;

/// Represents a vertex in a graph. Vertexes holds no data,
/// and is only useful in relation to other vertices
#[derive(Debug)]
pub struct Vertex<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> {
    id: VertexId<'id>,
    edges: HashMap<EdgeId<'id>, Edge>,
    item: Item,
    _phantom: &'id PhantomData<Weight>,
}

impl<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> Vertex<'id, Item, Weight, Edge> {
    /// Creates a new [`Vertex`] with the given `id`
    #[must_use]
    fn new(id: usize, item: Item) -> Self {
        Self {
            id: VertexId::new(id),
            edges: HashMap::new(),
            item,
            _phantom: &PhantomData,
        }
    }
    /// Returns an iterator over all the vertice's edges
    #[must_use]
    pub fn edges(&self) -> std::collections::hash_map::Iter<'_, EdgeId<'id>, Edge> {
        self.edges.iter()
    }
    /// Gets a reference to `self`'s inner item
    pub fn get_item(&self) -> &Item {
        &self.item
    }
    /// Gets a mutable reference to `self`'s inner item
    pub fn get_item_mut(&mut self) -> &mut Item {
        &mut self.item
    }
}

/// The overall graph, just a container for [`vertices`](Vertex)
pub struct Graph<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> {
    vertices: HashMap<VertexId<'id>, Rc<Node<'id, Item, Weight, Edge>>>,
    current_vertex_id: usize,
    current_edge_id: usize,
    len: usize,
}

impl<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> Default
    for Graph<'id, Item, Weight, Edge>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> Graph<'id, Item, Weight, Edge> {
    /// Constructs a new empty graph
    #[must_use]
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            current_vertex_id: 0,
            current_edge_id: 0,
            len: 0,
        }
    }
    /// Adds a vertex with no edges, and returns the [`VertexId`] of the
    /// created vertex
    pub fn add_vertex(&mut self, item: Item) -> VertexId<'id> {
        let vertex = Rc::new(GhostCell::new(Vertex::new(self.current_vertex_id, item)));
        let id = self.new_vertex_id();
        self.len += 1;
        self.vertices.insert(id, vertex);
        id
    }
    /// Empties self
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.current_vertex_id = 0;
        self.current_edge_id = 0;
        self.len = 0;
    }
    /// Adds an edge between the `id_one` and the `id_two`
    /// with the given weight
    /// The internal edge count will still be incremented,
    /// even if the method fails
    /// # Errors
    /// If `id_one` is the same as `id_two`, or either
    /// id doesn't exist within the graph, a [`GraphError`] will
    /// be returned
    pub fn add_edge(
        &mut self,
        id_one: VertexId<'id>,
        id_two: VertexId<'id>,
        weight: impl Fn(
            &Rc<Node<'id, Item, Weight, Edge>>,
            &Rc<Node<'id, Item, Weight, Edge>>,
            &mut GhostToken<'id>,
        ) -> Weight,
        token: &mut GhostToken<'id>,
    ) -> Result<(), GraphError<'id, Item, Weight, Edge>> {
        use GraphError::{IdenticalVertex, VertexNotFound};

        let id = self.new_edge_id();

        if id_one == id_two {
            return Err(IdenticalVertex(id_one));
        }

        let first = self.vertices.get(&id_one).ok_or(VertexNotFound(id_one))?;

        let second = self.vertices.get(&id_two).ok_or(VertexNotFound(id_two))?;

        let weight = weight(first, second, token);

        Edge::add_edge(weight, first, second, id, token).map_err(GraphError::AddEdgeError)
    }
    /// The number of [vertices](Vertex) in the graph
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }
    /// If there are no [vertices](Vertex) in the graph
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Gets a new id for a new [`Vertex`]
    fn new_vertex_id(&mut self) -> VertexId<'id> {
        let id = VertexId::new(self.current_vertex_id);
        self.current_vertex_id += 1;
        id
    }
    /// Gets a new id for a new [`Edge`]
    fn new_edge_id(&mut self) -> EdgeId<'id> {
        let id = EdgeId::new(self.current_edge_id);
        self.current_edge_id += 1;
        id
    }
    /// Attempts to get a vertex using a given [`VertexId`]
    /// # Errors
    /// Returns `None` if `id` does not exist within the graph
    #[must_use]
    pub fn get(&self, id: VertexId<'id>) -> Option<&Rc<Node<'id, Item, Weight, Edge>>> {
        self.vertices.get(&id)
    }
    /// Returns an immutable iterator over the
    /// graph's nodes
    #[must_use]
    pub fn vertices(
        &self,
    ) -> std::collections::hash_map::Iter<'_, VertexId<'id>, Rc<Node<'id, Item, Weight, Edge>>>
    {
        self.vertices.iter()
    }
    /// Returns an mutable iterator over the
    /// graph's nodes
    #[must_use]
    pub fn vertices_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<'_, VertexId<'id>, Rc<Node<'id, Item, Weight, Edge>>>
    {
        self.vertices.iter_mut()
    }
    /// Attempts to remove a [`Vertex`] from the graph, removing all edges from and
    /// to the [`Vertex`]
    /// # Errors
    /// Returns a [`GraphError`] if `id` is not found within the graph
    pub fn remove(
        &mut self,
        id: VertexId<'id>,
        token: &mut GhostToken<'id>,
    ) -> Result<(), GraphError<'id, Item, Weight, Edge>> {
        use GraphError::{EdgeNotFound, VertexNotFound};

        let to_remove = self.vertices.remove(&id).ok_or(VertexNotFound(id))?;

        let mut seq: Vec<EdgeId> = to_remove
            .g_borrow_mut(token)
            .edges
            .keys()
            .copied()
            .collect();

        // Iterate over all the edge ids in the selected
        // vertex's edges
        while let Some(e_id) = seq.pop() {
            // Removes the edge from the selected vertex's edges
            let one = to_remove.g_borrow_mut(token).edges.remove(&e_id);

            // SAFETY: Each key will be removed only once,
            // and is guranteed to be within to_remove, as
            // the keys were gotten from to_remove
            let one = unsafe { one.unwrap_unchecked() };

            // Finds the other vertex in the edge
            let two = one.other(id, token).ok_or(VertexNotFound(id))?.clone();

            // Removes the edge from the other vertex's edges
            two.g_borrow_mut(token)
                .edges
                .remove(&e_id)
                .ok_or(EdgeNotFound(e_id))?;
        }

        Ok(())
    }
    /// Returns whether `id_one` and `id_two` have an edge
    /// connecting them
    /// # Errors
    /// Returns [`None`] if either `id_one` or `id_two` is not
    /// found within the graph
    #[must_use]
    pub fn adjacent(
        &self,
        id_one: VertexId<'id>,
        id_two: VertexId<'id>,
        token: &GhostToken<'id>,
    ) -> Option<bool> {
        let vertex_one = self.vertices.get(&id_one)?.g_borrow(token);

        if let Some(vertex_two) = self.vertices.get(&id_two) {
            let second = vertex_two.g_borrow(token);
            for (id, _) in vertex_one.edges() {
                /*if edge.other(id_one, token).unwrap().g_borrow(token).id == second {
                    return Some(true);
                }*/
                if second.edges.contains_key(id) {
                    return Some(true);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum GraphError<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> {
    EdgeNotFound(EdgeId<'id>),
    VertexNotFound(VertexId<'id>),
    IdenticalVertex(VertexId<'id>),
    AddEdgeError(Edge::Error),
}
