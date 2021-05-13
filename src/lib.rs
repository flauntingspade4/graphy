#![warn(clippy::pedantic, clippy::nursery)]
#![feature(const_raw_ptr_deref, const_mut_refs)]

pub mod ghost;

use std::{collections::HashMap, rc::Rc};

use ghost::{GhostCell, GhostToken, InvariantLifetime};

type Cell<'a, 'id> = GhostCell<'id, Vertex<'a, 'id>>;

/// An edge between two [vertices](Vertex), with a given weight
#[derive(Debug, Clone)]
pub struct Edge<'a, 'id>(f64, Rc<Cell<'a, 'id>>, Rc<Cell<'a, 'id>>);

impl<'a, 'id> Edge<'a, 'id> {
    fn add_edge<'new_id>(
        weight: f64,
        first: &Rc<Cell<'a, 'id>>,
        second: &Rc<Cell<'a, 'id>>,
        id: Id<'id>,
        token: &'new_id mut GhostToken<'id>,
    ) {
        let edge = Self(weight, first.clone(), second.clone());

        first.ghost_borrow_mut(token).edges.insert(id, edge.clone());
        second.ghost_borrow_mut(token).edges.insert(id, edge);
    }
    /// Returns the other [`Vertex`] in `self`, returning `None`
    /// if the provided [`Id`] doesn't relate to either [`Vertex`]
    /// in `self`
    pub fn other<'new_id>(
        &'new_id self,
        id: Id<'id>,
        token: &'new_id GhostToken<'id>,
    ) -> Option<&Rc<Cell<'a, 'id>>> {
        if id.id == self.1.ghost_borrow(token).id {
            Some(&self.2)
        } else if id.id == self.2.ghost_borrow(token).id {
            Some(&self.1)
        } else {
            None
        }
    }
}

/// Represents a vertex in a graph-holds no data itself, and is only
/// useful in relation to other vertices
#[derive(Debug)]
pub struct Vertex<'a, 'id> {
    id: usize,
    edges: HashMap<Id<'id>, Edge<'a, 'id>>,
}

impl<'a, 'id> Vertex<'a, 'id> {
    /// Creates a new [`Vertex`] with the given `id`
    #[must_use]
    pub fn new(id: usize) -> Self {
        Self {
            id,
            edges: HashMap::new(),
        }
    }
    /// Returns an iterator over all the vertice's edges
    #[must_use]
    pub fn edges(&self) -> std::collections::hash_map::Iter<'_, Id<'id>, Edge<'a, 'id>> {
        self.edges.iter()
    }
}

/// The overall graph, just a container for [`vertices`](Vertex)
pub struct Graph<'a, 'id> {
    vertices: HashMap<Id<'id>, Rc<Cell<'a, 'id>>>,
    current_id: usize,
    len: usize,
}

impl<'a, 'id> Graph<'a, 'id> {
    /// Constructs a new empty graph
    #[must_use]
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            current_id: 0,
            len: 0,
        }
    }
    /// Adds a vertex with no edges, and returns the id of the
    /// created vertex
    pub fn add_vertex(&mut self) -> Id<'id> {
        let vertex = Rc::new(GhostCell::new(Vertex::new(self.current_id)));
        let id = self.new_id();
        self.len += 1;
        self.vertices.insert(id, vertex);
        id
    }
    /// Empties self
    pub fn clear<'new_id>(&mut self, token: &'new_id mut GhostToken<'id>) {
        let mut seq = self.vertices.keys().cloned().collect::<Vec<_>>();
        while let Some(index) = seq.pop() {
            self.remove(index, token).unwrap();
        }
    }
    /// Adds an edge between the `first_index` and the `second_index`
    /// with weight `weight`
    /// # Errors
    /// If `first_index` is the same as `second_index`, or either
    /// index doesn't exist within the graph, a `GraphError` will
    /// be returned
    pub fn add_edge<'new_id>(
        &'new_id mut self,
        first_index: Id<'id>,
        second_index: Id<'id>,
        weight: f64,
        token: &'new_id mut GhostToken<'id>,
    ) -> Result<(), GraphError<'id>> {
        if first_index == second_index {
            return Err(GraphError::IdenticalVertex(first_index));
        }

        let mut first_vertex = Err(GraphError::VertexNotFound(first_index));
        let mut second_vertex = Err(GraphError::VertexNotFound(second_index));

        for (id, vertex) in &self.vertices {
            if *id == first_index {
                first_vertex = Ok(vertex.clone());
                if first_vertex.is_ok() && second_vertex.is_ok() {
                    break;
                }
            } else if *id == second_index {
                second_vertex = Ok(vertex.clone());
                if first_vertex.is_ok() && second_vertex.is_ok() {
                    break;
                }
            }
        }

        // Either will fail if the vertex is not found
        let first = first_vertex?;
        let second = second_vertex?;

        let id = self.new_id();

        Edge::add_edge(weight, &first, &second, id, token);

        Ok(())
    }
    /// The number of [vertices](Vertex) in the graph
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }
    /// If there are no [vertices](Vertex) in the graph
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Gets a new id for a new [`Vertex`] or [`Edge`]
    fn new_id(&mut self) -> Id<'id> {
        let id = Id::new(self.current_id);
        self.current_id += 1;
        id
    }
    /// Attempts to get a vertice using a given [`Id`]
    #[must_use]
    pub fn get<'new_id>(&'new_id self, index: Id<'id>) -> Option<&'new_id Rc<Cell<'a, 'id>>> {
        self.vertices.get(&index)
    }
    /// Attempts to remove a [`Vertex`] from the graph, removing all [`edges`](Edge) from and
    /// to the [`Vertex`]
    pub fn remove(
        &mut self,
        index: Id<'id>,
        token: &mut GhostToken<'id>,
    ) -> Result<(), GraphError<'id>> {
        let to_remove = self
            .vertices
            .remove(&index)
            .ok_or(GraphError::IdenticalVertex(index))?;

        let mut seq = to_remove
            .ghost_borrow_mut(token)
            .edges
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        while let Some(id) = seq.pop() {
            let edge = to_remove.ghost_borrow_mut(token).edges.remove(&id).unwrap();

            let other = edge.other(index, &token).unwrap().clone();
            other.ghost_borrow_mut(token).edges.remove(&id).unwrap();
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum GraphError<'id> {
    VertexNotFound(Id<'id>),
    IdenticalVertex(Id<'id>),
}

/// The type describing a [`Vertex`] within a [`Graph`]
#[derive(Clone, Copy, Hash)]
pub struct Id<'id> {
    id: usize,
    _marker: InvariantLifetime<'id>,
}

impl<'id> Id<'id> {
    /// Constructs a new [`Id`] with a given id
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self {
            id,
            _marker: InvariantLifetime::new(),
        }
    }
    #[must_use]
    pub const fn id(self) -> usize {
        self.id
    }
}

impl<'id> PartialEq for Id<'id> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<'id> Eq for Id<'id> {}

impl<'id> core::fmt::Debug for Id<'id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
