#![allow(clippy::module_name_repetitions)]

use core::{cell::UnsafeCell, marker::PhantomData};

#[derive(Clone, Copy, Default, Debug, Hash)]
pub(crate) struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

impl<'id> InvariantLifetime<'id> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

/// A 'key' that unlocks [`GhostCell`]'s contents
/// with the mutability avaliable
/// ```rust
/// use graph::{edge::UnDirectedWeightedEdge, ghost::GhostToken, Graph};
///
/// GhostToken::new(|mut token| {
///     let mut graph: Graph<usize, (), UnDirectedWeightedEdge<_, _>> = Graph::new();
///
///     {
///         let first = graph.add_vertex(0);
///
///         let first_vertex = graph.get_vertex(first).unwrap().ghost();
///
///         // As `token` is mutable, we can change the contents of a `GhostCell`
///         *first_vertex.g_borrow_mut(&mut token).get_item_mut() += 10;
///     }
///
///     // Now `token` can be used mutably or immutably
///     // ...
/// })
/// ```
#[derive(Default)]
pub struct GhostToken<'id> {
    _marker: InvariantLifetime<'id>,
}

impl<'id> GhostToken<'id> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'new_id, F, R>(f: F) -> R
    where
        F: FnOnce(GhostToken<'new_id>) -> R + 'new_id,
    {
        let token = GhostToken::default();
        f(token)
    }
}

/// A 'lock' that can be unlocked using [`GhostToken`]
///
/// Refer to [`GhostToken`]'s documentation for example
/// usage
#[repr(transparent)]
pub struct GhostCell<'id, T> {
    value: UnsafeCell<T>,
    _marker: InvariantLifetime<'id>,
}

impl<'id, T> GhostCell<'id, T> {
    /// Creates a new [`GhostCell`] from a
    /// given value
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            _marker: InvariantLifetime::new(),
        }
    }
    /// Gets a mutable value
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
    /// Unwraps the value
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
    /// Immutably borrows the [`GhostCell`]'s contents,
    /// with the guarantee it's not being accessed mutably
    /// elsewhere by the fact the token must be immutably borrowed
    /// for the entirety of the time it's contents is borrowed
    pub const fn g_borrow<'a>(&'a self, _token: &'a GhostToken<'id>) -> &'a T {
        unsafe { &*self.value.get() }
    }
    /// Mutably borrows the [`GhostCell`]'s contents,
    /// guaranteeing unique mutably access to it's contents
    /// by the way that a mutable reference to the token
    /// is required
    pub const fn g_borrow_mut<'a>(&'a self, _token: &'a mut GhostToken<'id>) -> &'a mut T {
        unsafe { &mut *self.value.get() }
    }
}
