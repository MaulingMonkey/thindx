use std::marker::PhantomData;

/// Utility trait for inferring raw object lifetimes from `self` or `self.phantom`
#[doc(hidden)]
pub unsafe trait ParentOrPhantom<'r> {}

unsafe impl<'r, T> ParentOrPhantom<'r> for &'r T {}
unsafe impl<'r, T> ParentOrPhantom<'r> for PhantomData<&'r T> {}
