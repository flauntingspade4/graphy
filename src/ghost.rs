#![allow(clippy::module_name_repetitions)]

use core::{cell::UnsafeCell, marker::PhantomData};

#[derive(Clone, Copy, Default, Debug, Hash)]
pub(crate) struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);

impl<'id> InvariantLifetime<'id> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

#[derive(Default)]
pub struct GhostToken<'id> {
    _marker: InvariantLifetime<'id>,
}

impl<'id> GhostToken<'id> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<R>(f: impl for<'new_id> FnOnce(GhostToken<'new_id>) -> R) -> R {
        let token = Self::default();
        f(token)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct GhostCell<'id, T> {
    value: UnsafeCell<T>,
    _marker: InvariantLifetime<'id>,
}

impl<'id, T> GhostCell<'id, T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            _marker: InvariantLifetime::new(),
        }
    }
    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
    pub const fn g_borrow<'a>(&'a self, _token: &'a GhostToken<'id>) -> &T {
        unsafe { &*self.value.get() }
    }
    pub const fn g_borrow_mut<'a>(&'a self, _token: &'a mut GhostToken<'id>) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}
