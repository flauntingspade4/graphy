#![warn(clippy::pedantic, clippy::nursery)]
#![feature(const_raw_ptr_deref, const_mut_refs)]

pub mod ghost;

use ghost::{GhostCell, GhostToken};

#[derive(Debug)]
pub struct Edge<'a, 'id>(
    f64,
    &'a GhostCell<'id, Vertex<'a, 'id>>,
    &'a GhostCell<'id, Vertex<'a, 'id>>,
);

impl<'a, 'id> Edge<'a, 'id> {
    #[must_use]
    pub const fn new(
        weight: f64,
        first: &'a GhostCell<'id, Vertex<'a, 'id>>,
        second: &'a GhostCell<'id, Vertex<'a, 'id>>,
    ) -> Self {
        Self(weight, first, second)
    }
}

#[derive(Debug)]
pub struct Vertex<'a, 'id> {
    id: usize,
    edges: Vec<Edge<'a, 'id>>,
}

impl<'a, 'id> Vertex<'a, 'id> {
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self {
            id,
            edges: Vec::new(),
        }
    }
    pub fn add_edge(
        first: &'a GhostCell<'id, Vertex<'a, 'id>>,
        second: &'a GhostCell<'id, Vertex<'a, 'id>>,
        weight: f64,
        token: &'a mut GhostToken<'id>,
    ) {
        let edge = Edge::new(weight, first, second);
        first.borrow_mut(token).edges.push(edge);
        let edge = Edge::new(weight, first, second);
        second.borrow_mut(token).edges.push(edge);
    }
    #[must_use]
    pub fn neighbors(&self) -> &[Edge<'a, 'id>] {
        &self.edges
    }
}

pub struct Graph<'a, 'id> {
    vertices: Vec<GhostCell<'id, Vertex<'a, 'id>>>,
    current_id: usize,
    len: usize,
}

impl<'a, 'id> Graph<'a, 'id> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            vertices: Vec::new(),
            current_id: 0,
            len: 0,
        }
    }
    pub fn add_vertex(&mut self) -> usize {
        let vertex = GhostCell::new(Vertex::new(self.current_id));
        self.vertices.push(vertex);
        self.current_id += 1;
        self.len += 1;
        self.current_id - 1
    }
    /// # Errors
    /// If first_index is the same as second_index, or either
    /// index doesn't exist within the graph, a `GraphError` will
    /// be returned
    pub fn add_edge(
        &'a mut self,
        first_index: usize,
        second_index: usize,
        weight: f64,
        token: &'a mut GhostToken<'id>,
    ) -> Result<(), GraphError> {
        if first_index == second_index {
            return Err(GraphError::IdenticalVertex(first_index));
        }

        let mut first_vertex = Err(GraphError::VertexNotFound(first_index));
        let mut second_vertex = Err(GraphError::VertexNotFound(second_index));

        for vertex in &self.vertices {
            let id = vertex.borrow(token).id;

            if id == first_index {
                first_vertex = Ok(vertex);
                if first_vertex.is_ok() && second_vertex.is_ok() {
                    break;
                }
            } else if id == second_index {
                second_vertex = Ok(vertex);
                if first_vertex.is_ok() && second_vertex.is_ok() {
                    break;
                }
            }
        }

        let first = first_vertex?;
        let second = second_vertex?;

        let edge = Edge::new(weight, first, second);
        first.borrow_mut(token).edges.push(edge);
        let edge = Edge::new(weight, first, second);
        second.borrow_mut(token).edges.push(edge);

        Ok(())
    }
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn get(
        &'a self,
        index: usize,
        token: &'a GhostToken<'id>,
    ) -> Option<&GhostCell<'id, Vertex<'a, 'id>>> {
        match self
            .vertices
            .binary_search_by(|vertex| vertex.borrow(token).id.cmp(&index))
        {
            Ok(i) => Some(&self.vertices[i]),
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum GraphError {
    VertexNotFound(usize),
    IdenticalVertex(usize),
}
