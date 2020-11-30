use std::marker::PhantomData;

pub(crate) unsafe trait ParentOrPhantom<'r> {}

unsafe impl<'r, T> ParentOrPhantom<'r> for &'r T {}
unsafe impl<'r, T> ParentOrPhantom<'r> for PhantomData<&'r T> {}
