#![warn(clippy::pedantic, clippy::nursery, clippy::all)]
#![feature(
    const_raw_ptr_deref,
    const_mut_refs,
    option_result_unwrap_unchecked,
    allocator_api
)]
#![no_std]

//! A simple and efficient graph-theory library written with a focus
//! on error handling
//!
//! If you're new to this library, a good place to start is [`Graph`]
//! to get a feel for how to add vertices and edges and then,
//! [`GhostToken`](ghost::GhostToken) to understand how to actually use the library.
//!
//! # What are [`GhostToken`](ghost::GhostToken) and [`GhostCell`](ghost::GhostCell)?
//!
//! [`GhostToken`](ghost::GhostToken) and [`GhostCell`](ghost::GhostCell) are used together to ensure
//! unique mutable ownership, or shared immutable ownership, and
//! are part of what makes the library so fast.
//!
//! They are based on <http://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf>

extern crate alloc;

/// A module containing the [`EdgeTrait`], which is the trait
/// that must be implemented by any edge that can be used, and
/// the edges that already implement [`EdgeTrait`]
pub mod edge;
/// A module containing the types outlined in
/// <http://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf>,
/// [`GhostToken`](ghost::GhostToken) and [`GhostCell`](ghost::GhostCell)
pub mod ghost;
mod graph;
mod id;
mod shared;
mod vertex;

pub use crate::graph::Graph;
use edge::EdgeTrait;
pub use id::{EdgeId, VertexId};
pub use shared::Shared;
pub use vertex::Vertex;

pub type SharedNode<'id, Item, Weight, Edge> = Shared<'id, Vertex<'id, Item, Weight, Edge>>;
/// A node within the graph, shorthand for `GhostCell<Vertex>`
pub type Node<'id, Item, Weight, Edge> = ghost::GhostCell<'id, Vertex<'id, Item, Weight, Edge>>;

/// An error returned by various method in this library
#[derive(Debug)]
pub enum GraphError<'id, Item, Weight, Edge: EdgeTrait<'id, Item, Weight>> {
    /// A general error for when an edge isn't found containing the
    /// missing edge's id
    EdgeNotFound(EdgeId<'id>),
    /// A general error for when a vertex isn't found containing the
    /// missing vertice's id
    VertexNotFound(VertexId<'id>),
    /// An error for when two identical [`VertexId`]s are used when
    /// only unique [`VertexId`]s should be used
    IdenticalVertex(VertexId<'id>),
    /// A generic error for when there's an error whilst adding an edge
    /// between two [vertices](vertex::Vertex) in the graph.
    ///
    /// The exact error contained depends on the type of edge that's attempting
    /// to be added
    AddEdgeError(Edge::Error),
    /// An error for when there's no edge between two [vertices](vertex::Vertex)
    /// when there should be
    NoEdgeBetween,
    /// An error for when there's already an edge between two
    /// [vertices](vertex::Vertex) when there shouldn't be
    AlreadyEdgeBetween,
}
