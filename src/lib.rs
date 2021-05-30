#![warn(clippy::pedantic, clippy::nursery, clippy::all)]
#![feature(const_raw_ptr_deref, const_mut_refs, option_result_unwrap_unchecked)]
#![no_std]

//! A simple and efficient graph-theory library written with a focus
//! on error handling
//!
//! If you're new to this library, a good place to start is [`Graph`]
//! to get a feel for how to add vertices and edges and then,
//! [`GhostToken`] to understand how to actually use the library.
//!
//! # What are [`GhostToken`] and [`GhostCell`]?
//!
//! [`GhostToken`] and [`GhostCell`] are used together to ensure
//! unique mutable ownership, or shared immutable ownership, and
//! are part of what makes the library so fast.
//!
//! They are based on <http://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf>
//!

extern crate alloc;

pub mod edge;
pub mod ghost;
mod id;
mod shared;
mod vertex;

use ghost::{GhostCell, GhostToken};

use edge::EdgeTrait;
use id::EdgeId;
pub use id::VertexId;
pub use shared::Shared;
pub use vertex::Vertex;

use hashbrown::HashMap;

type SharedNode<'id, Item, Weight, Edge> = Shared<'id, Vertex<'id, Item, Weight, Edge>>;
type Node<'id, Item, Weight, Edge> = GhostCell<'id, Vertex<'id, Item, Weight, Edge>>;

/// The overall graph, just a container for [`vertices`](Vertex)
///
/// # Types
/// `'id` - The marker lifetime to indicate which [`GhostToken`] works
/// with the specific graph's [`GhostToken`]s
///
/// `Item` - The type that each [`Vertex`] contains
///
/// `Weight` - The type that each edge between vertices contains
///
/// `Edge` - The type of edge being used, examples of which are
/// [`UnDirectedUnWeightedEdge`](edge::UnDirectedUnWeightedEdge)
/// and [`UnDirectedWeightedEdge`](edge::UnDirectedWeightedEdge)
pub struct Graph<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> {
    vertices: HashMap<VertexId<'id>, SharedNode<'id, Item, Weight, Edge>>,
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

impl<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> Drop
    for Graph<'id, Item, Weight, Edge>
{
    fn drop(&mut self) {
        self.clear()
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
        let vertex = GhostCell::new(Vertex::new(self.current_vertex_id, item));
        let id = self.new_vertex_id();
        self.len += 1;
        self.vertices.insert(id, Shared::new(vertex));
        id
    }
    /// Empties self
    pub fn clear(&mut self) {
        self.vertices.drain().for_each(|(_, s)| unsafe { s.drop() });
        self.current_vertex_id = 0;
        self.current_edge_id = 0;
        self.len = 0;
    }
    /// Adds an edge between the `id_one` and the `id_two`
    /// with the given weight
    ///
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
            &Node<'id, Item, Weight, Edge>,
            &Node<'id, Item, Weight, Edge>,
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

        let weight = weight(first.ghost(), second.ghost(), token);

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
    pub fn get(&self, id: VertexId<'id>) -> Option<&SharedNode<'id, Item, Weight, Edge>> {
        self.vertices.get(&id)
    }
    /// Returns an immutable iterator over the
    /// graph's nodes
    #[must_use]
    pub fn vertices(
        &self,
    ) -> hashbrown::hash_map::Iter<'_, VertexId<'id>, SharedNode<'id, Item, Weight, Edge>> {
        self.vertices.iter()
    }
    /// Returns an mutable iterator over the
    /// graph's nodes
    #[must_use]
    pub fn vertices_mut(
        &mut self,
    ) -> hashbrown::hash_map::IterMut<'_, VertexId<'id>, SharedNode<'id, Item, Weight, Edge>> {
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

        let seq = to_remove.borrow(token);
        let mut seq: alloc::vec::Vec<EdgeId> = seq.edges().keys().copied().collect();

        // Iterate over all the edge ids in the selected
        // vertex's edges
        while let Some(e_id) = seq.pop() {
            // Removes the edge from the selected vertex's edges
            let one = to_remove.borrow_mut(token);
            let one = one.edges_mut().remove(&e_id);

            // SAFETY: Each key will be removed only once,
            // and is guranteed to be within to_remove, as
            // the keys were gotten from to_remove
            let one = unsafe { one.unwrap_unchecked() };

            // Finds the other vertex in the edge
            let two = one.other(id, token).ok_or(VertexNotFound(id))?.clone();
            let two = two.borrow_mut(token);

            // Removes the edge from the other vertex's edges
            two.edges_mut().remove(&e_id).ok_or(EdgeNotFound(e_id))?;
        }

        unsafe { to_remove.drop() }

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
        let vertex_one = self.vertices.get(&id_one)?.borrow(token);

        if let Some(vertex_two) = self.vertices.get(&id_two) {
            let second = vertex_two.borrow(token);
            for (id, _) in vertex_one.edges() {
                /*if edge.other(id_one, token).unwrap().g_borrow(token).id == second {
                    return Some(true);
                }*/
                if second.edges().contains_key(id) {
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
