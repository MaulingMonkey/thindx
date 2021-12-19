use std::fmt::{self, Debug, Formatter};
use std::ops::*;



#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Bytecode<T: AsRef<[u8]>>(T);

impl Bytecode {
    /// ### Safety
    ///
    /// Parsing APIs are prone to undefined behavior and CVEs, and bytecode is no exception.
    /// By invoking this fn, you pledge that `inner` is valid DXBC or DXIL bytecode.
    pub unsafe fn trust(inner: T) -> Self { Self(inner) }

    pub fn as_bytes(&self) -> &[u8] { self.0.as_ref() }
    pub fn bytes<'s>(&'s self) -> impl Iterator<Item = u8> + 's { self.0.as_ref().iter().copied() }
    pub fn into_inner(self) -> T { self.0 }
}

impl AsRef <[u8]> for Bytecode { fn as_ref(&self) -> &[u8] { self.0.as_ref() } }
impl Borrow<[u8]> for Bytecode { fn borrow(&self) -> &[u8] { self.0.as_ref() } }
impl Debug for Bytecode { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.0.as_ref(), fmt) } }
impl Deref for Bytecode { fn deref(&self) -> &[u8] { self.0.as_ref() } type Target = [u8]; }

impl<T> From<Bytecode<T>> for T { fn from(outer: Bytecode<T>) -> T { outer.0 } }

impl<I> Index<I> for Bytecode where I: SliceIndex<[u8]> {
    type Output = <I as SliceIndex<[u8]>>::Output;
    fn index(&self, index: I) -> &Self::Output { self.0.as_ref().index(index) }
}
