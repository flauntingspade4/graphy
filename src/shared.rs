use core::ptr::NonNull;

use alloc::boxed::Box;

use crate::ghost::{GhostCell, GhostToken};

/// A shared node, simply a wrapper around
/// a [`NonNull`] around a [`GhostCell`] around
/// `T`.
///
/// As the contents of the [`NonNull`] cannot be
/// mutated without a mutable reference to a
/// [`GhostToken`], it's guranteed that Rust's
/// aliasing rules are upheld.
///
/// Cloning a [`Shared`] will **not** clone the
/// internal value, and instead return a [`Shared`]
/// with a pointer to the same value
#[derive(Debug)]
pub struct Shared<'id, T>(NonNull<GhostCell<'id, T>>);

impl<'id, T> Clone for Shared<'id, T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'id, T> Shared<'id, T> {
    /// Makes a new [`Shared`] based off a given [`GhostCell`]
    pub fn new(item: GhostCell<'id, T>) -> Self {
        Self(Box::leak(Box::new(item)).into())
    }
    /// Returns a reference to the underlying [`GhostCell`]
    pub fn ghost(&self) -> &GhostCell<'id, T> {
        unsafe { self.0.as_ref() }
    }
    /// A shorthand for `shared.ghost().g_borrow(&token)`
    pub fn borrow<'a>(&self, token: &'a GhostToken<'id>) -> &'a T {
        unsafe { self.0.as_ref() }.g_borrow(token)
    }
    /// A shorthand for `shared.ghost().g_borrow_mut(&token)`
    pub fn borrow_mut<'a>(&self, token: &'a mut GhostToken<'id>) -> &'a mut T {
        unsafe { self.0.as_ref() }.g_borrow_mut(token)
    }
    /// Drops the contents of `self`
    ///
    /// # Safety
    /// There can be no other pointers to the
    /// contents of self
    pub(crate) unsafe fn drop(&self) {
        core::ptr::drop_in_place(self.0.as_ptr())
    }
}
