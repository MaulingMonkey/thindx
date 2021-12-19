use std::borrow::Borrow;
use std::fmt::{self, Debug, Formatter};
use std::ops::*;
use std::slice::SliceIndex;



#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bytecode<T: AsRef<[u8]>>(T);

impl<T: AsRef<[u8]>> Bytecode<T> {
    /// ### Safety
    ///
    /// Parsing APIs are prone to undefined behavior and CVEs, and bytecode is no exception.
    /// By invoking this fn, you pledge that `inner` is valid DXBC or DXIL bytecode.
    pub unsafe fn trust(inner: T) -> Self { Self(inner) }

    pub fn as_bytes(&self) -> &[u8] { self.0.as_ref() }
    pub fn bytes<'s>(&'s self) -> impl Iterator<Item = u8> + 's { self.0.as_ref().iter().copied() }
    pub fn into_inner(self) -> T { self.0 }
    pub fn as_ref(&self) -> Bytecode<&[u8]> { Bytecode(self.0.as_ref()) }
}

impl<T: AsRef<[u8]>> AsRef <[u8]> for Bytecode<T> { fn as_ref(&self) -> &[u8] { self.0.as_ref() } }
impl<T: AsRef<[u8]>> Borrow<[u8]> for Bytecode<T> { fn borrow(&self) -> &[u8] { self.0.as_ref() } }
impl<T: AsRef<[u8]>> Debug for Bytecode<T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.0.as_ref(), fmt) } }
impl<T: AsRef<[u8]>> Deref for Bytecode<T> { fn deref(&self) -> &[u8] { self.0.as_ref() } type Target = [u8]; }

impl<T: AsRef<[u8]>, I> Index<I> for Bytecode<T> where I: SliceIndex<[u8]> {
    type Output = <I as SliceIndex<[u8]>>::Output;
    fn index(&self, index: I) -> &Self::Output { self.0.as_ref().index(index) }
}
