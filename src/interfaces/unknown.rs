use crate::*;

use std::ops::Deref;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)\]
/// Base COM interface all D3D9 types eventually derive from.
#[derive(Clone)] #[repr(transparent)]
pub struct Unknown(pub(crate) mcom::Rc<winapi::um::unknwnbase::IUnknown>);

impl Unknown {
    /// [AddRef] + [Release] to determine refcount.
    ///
    /// [AddRef]:   https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref
    /// [Release]:  https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    pub fn approx_refcount(&self) -> u32 {
        unsafe {
            let unk = &*self.0;
            unk.AddRef();
            unk.Release()
        }
    }
}

/// Like [std::mem::drop], but use `debug_assert_eq!` to verify the refcount is
/// one for e.g. device lost handling purpouses.
///
/// This assumption may be violated by:
/// *   Middleware acquiring refcounts
/// *   Graphics debuggers acquiring refcounts
/// *   ???
pub fn drop_final(unk: impl AsRef<Unknown>) {
    let rc = unk.as_ref().approx_refcount();
    debug_assert_eq!(1, rc, "this wasn't the final object");
}



pub trait AsRef<Unknown> {
    fn as_ref(&self) -> &Unknown;
}

impl AsRef<Unknown> for Unknown {
    fn as_ref(&self) -> &Unknown { self }
}

impl<T: Deref> AsRef<Unknown> for T where T::Target : AsRef<Unknown> {
    fn as_ref(&self) -> &Unknown { (**self).as_ref() }
}
