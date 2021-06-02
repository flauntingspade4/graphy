use hashbrown::HashMap;

use crate::{
    edge::EdgeTrait,
    ghost::{GhostCell, GhostToken},
    id::EdgeId,
    GraphError, Node, Shared, SharedNode, Vertex, VertexId,
};

/// The overall graph, just a container for [vertices](Vertex)
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
/// [`UnDirectedUnWeightedEdge`](crate::edge::UnDirectedUnWeightedEdge)
/// and [`UnDirectedWeightedEdge`](crate::edge::UnDirectedWeightedEdge)
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
    /// Adds an edge between the `id_one` and the `id_two`
    /// with the given weight
    ///
    /// The internal edge count will still be incremented,
    /// even if the method fails
    /// # Errors
    /// If `id_one` is the same as `id_two`, or either
    /// id doesn't exist within the graph, a [`GraphError`] will
    /// be returned
    pub fn add_edge<T>(
        &mut self,
        id_one: VertexId<'id>,
        id_two: VertexId<'id>,
        item: T,
        weight: impl for<'a> Fn(
            T,
            &'a Node<'id, Item, Weight, Edge>,
            &'a Node<'id, Item, Weight, Edge>,
            &'a mut Self,
            &'a mut GhostToken<'id>,
        ) -> Weight,
        token: &mut GhostToken<'id>,
    ) -> Result<(), GraphError<'id, Item, Weight, Edge>> {
        use GraphError::{AlreadyEdgeBetween, IdenticalVertex, VertexNotFound};

        let id = self.new_edge_id();

        if id_one == id_two {
            Err(IdenticalVertex(id_one))
        } else if self.adjacent(id_one, id_two, token)? {
            Err(AlreadyEdgeBetween)
        } else {
            let first = self
                .vertices
                .get(&id_one)
                .ok_or(VertexNotFound(id_one))?
                .clone();

            let second = self
                .vertices
                .get(&id_two)
                .ok_or(VertexNotFound(id_two))?
                .clone();

            let weight = weight(item, first.ghost(), second.ghost(), self, token);

            Edge::add_edge(weight, &first, &second, id, token).map_err(GraphError::AddEdgeError)
        }
    }
    /// Creates an edge between `id_one` and `id_two`
    /// if it doesn't already exist, and if it does,
    /// changes it to the result of `weight`
    ///
    /// # Errors
    /// If `id_one` is the same as `id_two`,
    /// [`GraphError::IdenticalVertex`] is returned,
    /// or if either id doesn't have a vertex
    /// within the graph, an [`GraphError::VertexNotFound`]
    /// will be returned
    pub fn create_or_update_edge_between<T>(
        &mut self,
        id_one: VertexId<'id>,
        id_two: VertexId<'id>,
        item: T,
        weight: impl for<'a> Fn(
            T,
            &'a Node<'id, Item, Weight, Edge>,
            &'a Node<'id, Item, Weight, Edge>,
            &'a mut Self,
            &'a mut GhostToken<'id>,
        ) -> Weight,
        token: &mut GhostToken<'id>,
    ) -> Result<(), GraphError<'id, Item, Weight, Edge>>
    where
        T: Clone,
    {
        use GraphError::{IdenticalVertex, VertexNotFound};

        if id_one == id_two {
            Err(IdenticalVertex(id_one))
        } else {
            let vertex_one = self
                .vertices
                .get(&id_one)
                .ok_or(VertexNotFound(id_one))?
                .clone();

            let vertex_two = self
                .vertices
                .get(&id_two)
                .ok_or(VertexNotFound(id_two))?
                .clone();

            let mut edge_id = None;

            {
                for id in vertex_one.borrow(token).edges.keys() {
                    if vertex_two.borrow(token).edges.contains_key(id) {
                        edge_id = Some(*id);
                        break;
                    }
                }
            }

            match edge_id {
                Some(id) => {
                    *vertex_one
                        .borrow_mut(token)
                        .edges
                        .get_mut(&id)
                        .unwrap()
                        .get_weight_mut() = weight(
                        item.clone(),
                        vertex_one.ghost(),
                        vertex_two.ghost(),
                        self,
                        token,
                    );

                    *vertex_two
                        .borrow_mut(token)
                        .edges
                        .get_mut(&id)
                        .unwrap()
                        .get_weight_mut() =
                        weight(item, vertex_one.ghost(), vertex_two.ghost(), self, token);

                    Ok(())
                }
                None => Edge::add_edge(
                    weight(item, vertex_one.ghost(), vertex_two.ghost(), self, token),
                    &vertex_one,
                    &vertex_two,
                    self.new_edge_id(),
                    token,
                )
                .map_err(GraphError::AddEdgeError),
            }
        }
    }

    /// Empties self
    pub fn clear(&mut self) {
        self.vertices.drain().for_each(|(_, s)| unsafe { s.drop() });
        self.current_vertex_id = 0;
        self.current_edge_id = 0;
        self.len = 0;
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

        self.len -= 1;

        let to_remove = self.vertices.remove(&id).ok_or(VertexNotFound(id))?;

        let seq = to_remove.borrow(token);
        let mut seq: alloc::vec::Vec<EdgeId> = seq.edges.keys().copied().collect();

        // Iterate over all the edge ids in the selected
        // vertex's edges
        while let Some(e_id) = seq.pop() {
            // Removes the edge from the selected vertex's edges
            let one = to_remove.borrow_mut(token);
            let one = one.edges.remove(&e_id);

            // SAFETY: Each key will be removed only once,
            // and is guranteed to be within to_remove, as
            // the keys were gotten from to_remove
            let one = unsafe { one.unwrap_unchecked() };

            // Finds the other vertex in the edge
            let two = one.other(id, token).ok_or(VertexNotFound(id))?.clone();
            let two = two.borrow_mut(token);

            // Removes the edge from the other vertex's edges
            two.edges.remove(&e_id).ok_or(EdgeNotFound(e_id))?;
        }

        unsafe { to_remove.drop() }

        Ok(())
    }
    /// Removes the edge between `id_one` and `id_two`
    ///
    /// # Errors
    /// If there is no edge between `id_one` and `id_two`,
    /// [`GraphError::NoEdgeBetween`] will be returned.
    /// If either `id_one` or `id_two` don't have an associated
    /// vertex within the graph, [`GraphError::VertexNotFound`]
    /// will be returned
    pub fn remove_edge_between(
        &mut self,
        id_one: VertexId<'id>,
        id_two: VertexId<'id>,
        token: &mut GhostToken<'id>,
    ) -> Result<(), GraphError<'id, Item, Weight, Edge>> {
        use GraphError::{NoEdgeBetween, VertexNotFound};

        // Finds the `edge_id` of the edge between
        // `id_one` and `id_two` - remains `None` if
        // there is no edge between them
        let mut edge_id = None;
        {
            let vertex_one = self
                .vertices
                .get(&id_one)
                .ok_or(VertexNotFound(id_one))?
                .borrow(token);

            if let Some(vertex_two) = self.vertices.get(&id_two) {
                let second = vertex_two.borrow(token);
                for id in vertex_one.edges.keys() {
                    if second.edges.contains_key(id) {
                        edge_id = Some(*id);
                        break;
                    }
                }
            } else {
                return Err(VertexNotFound(id_two));
            }
        }

        // Actually remove the edges
        self.vertices
            .get(&id_one)
            .ok_or(VertexNotFound(id_one))?
            .borrow_mut(token)
            .edges
            .remove(&edge_id.ok_or(NoEdgeBetween)?);

        self.vertices
            .get(&id_two)
            .ok_or(VertexNotFound(id_one))?
            .borrow_mut(token)
            .edges
            .remove(&edge_id.ok_or(NoEdgeBetween)?);

        Ok(())
    }
    /// Returns whether `id_one` and `id_two` have an edge
    /// connecting them
    /// # Errors
    /// Returns [`None`] if either `id_one` or `id_two` is not
    /// found within the graph
    pub fn adjacent(
        &self,
        id_one: VertexId<'id>,
        id_two: VertexId<'id>,
        token: &GhostToken<'id>,
    ) -> Result<bool, GraphError<'id, Item, Weight, Edge>> {
        use GraphError::VertexNotFound;

        let vertex_one = self
            .vertices
            .get(&id_one)
            .ok_or(VertexNotFound(id_one))?
            .borrow(token);

        if let Some(vertex_two) = self.vertices.get(&id_two) {
            let second = vertex_two.borrow(token);
            for id in vertex_one.edges.keys() {
                if second.edges.contains_key(id) {
                    return Ok(true);
                }
            }
        }
        Err(VertexNotFound(id_two))
    }
}
