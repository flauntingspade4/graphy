use core::{
    alloc::{Allocator, Layout},
    ptr::NonNull,
};

use alloc::boxed::Box;

use crate::ghost::{GhostCell, GhostToken};

/// A shared node, simply a wrapper around
/// a [`NonNull`] around a [`GhostCell`] around
/// the generic type `T`.
///
/// As the contents of the [`NonNull`] cannot be
/// mutated without a mutable reference to a
/// [`GhostToken`], it's guaranteed that Rust's
/// aliasing rules are upheld.
///
/// Cloning a [`Shared`] will **not** clone the
/// internal value, and instead return a [`Shared`]
/// with a pointer to the same memory location
#[derive(Debug)]
pub struct Shared<'id, T>(NonNull<GhostCell<'id, T>>);

impl<'id, T> Shared<'id, T> {
    /// Makes a new [`Shared`] based off a given item
    ///
    /// Makes the item into a [`GhostCell`], allocates it
    /// on the heap, and then uses the given memory address
    pub fn new(item: T) -> Self {
        Self(Box::leak(Box::new(GhostCell::new(item))).into())
    }
    /// Returns a reference to the underlying [`GhostCell`]
    #[must_use]
    pub fn ghost(&self) -> &GhostCell<'id, T> {
        unsafe { self.0.as_ref() }
    }
    /// A shorthand for `shared.ghost().g_borrow(&token)`
    #[must_use]
    pub fn borrow<'a>(&self, token: &'a GhostToken<'id>) -> &'a T {
        unsafe { self.0.as_ref() }.g_borrow(token)
    }
    /// A shorthand for `shared.ghost().g_borrow_mut(&mut token)`
    pub fn borrow_mut<'a>(&self, token: &'a mut GhostToken<'id>) -> &'a mut T {
        unsafe { self.0.as_ref() }.g_borrow_mut(token)
    }
    /// Drops the contents of `self`
    ///
    /// # Safety
    /// There can be no other pointers to the
    /// contents of self
    pub(crate) unsafe fn drop(&self) {
        core::ptr::drop_in_place(self.0.as_ptr());
        alloc::alloc::Global.deallocate(self.0.cast(), Layout::new::<T>());
    }
    /// Unsafely reads the inner value as mutable
    ///
    /// # Safety
    /// This should only be used if one is certain that
    /// no other pointers exist to `self` which could
    /// be reading or writing at the same time
    pub(crate) unsafe fn read_mut(&mut self) -> &mut T {
        let x = self.0.as_mut();
        x.get_mut()
    }
    /// Clones self. Implemented as a method rather than
    /// a trait so users can't clone it, leading to possible
    /// use-after frees
    pub(crate) const fn clone(&self) -> Self {
        Self(self.0)
    }
    /// Converts `Shared<T>` to `Shared<U>`.
    /// Will allocate for a new `Shared<U>`
    pub fn convert<U>(self, token: &mut GhostToken<'id>) -> Shared<'id, U>
    where
        U: From<T>,
    {
        // SAFETY: The pointer will never be non-null,
        // as it's from a `NonNull`.
        // As this function takes a mutable reference to
        // the token, it's guranteed to be the only writer/reader
        // to the pointer itself
        let ptr = unsafe { self.0.as_ptr().read() };
        // SAFETY: Nothing else can be pointing to the old
        // memory location, due to the mutable reference to
        // the token
        unsafe { self.drop() };

        let inner = ptr.into_inner();

        let converted_inner: U = inner.into();

        Shared::new(converted_inner)
    }
}

impl<'id, T> PartialEq for Shared<'id, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}

impl<'id, T> Eq for Shared<'id, T> {}
