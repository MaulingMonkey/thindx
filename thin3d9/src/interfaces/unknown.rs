use crate::*;

use std::ops::Deref;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)\]
/// Base COM interface all D3D9 types eventually derive from.
#[derive(Clone)] #[repr(transparent)]
pub struct Unknown(pub(crate) mcom::Rc<winapi::um::unknwnbase::IUnknown>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)\]
/// IUnknown extension methods
///
/// ### Methods
///
/// | thin3d9                                                       | docs.microsoft.com    | Description |
/// | ------------------------------------------------------------- | --------------------- | ----------- |
/// | <span style="opacity: 25%">N/A</span>                         | [AddRef]              | Increments the reference count for an interface pointer to a COM object.
/// | <span style="opacity: 25%">N/A</span>                         | [QueryInterface]      | Retrieves pointers to the supported interfaces on an object.
/// | <span style="opacity: 25%">N/A</span>                         | [Release]             | Decrements the reference count for an interface on a COM object.
/// | [approx_refcount](Self::approx_refcount)                      | [AddRef], [Release]   | Get the (strong) reference count.  This value is intended to be used only for test purposes.
///
/// [AddRef]:           https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref
/// [QueryInterface]:   https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void)
/// [Release]:          https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
///
pub trait IUnknownExt : private::Sealed {
    /// [AddRef] + [Release] to determine refcount.
    ///
    /// [AddRef]:   https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref
    /// [Release]:  https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release
    fn approx_refcount(&self) -> u32 {
        unsafe {
            let unk = self.as_winapi();
            unk.AddRef();
            unk.Release()
        }
    }
}

impl<T: private::Sealed> IUnknownExt for T {}

mod private {
    use winapi::um::unknwnbase::IUnknown;
    pub unsafe trait Sealed                     { fn as_winapi(&self) -> &IUnknown; }
    unsafe impl Sealed for mcom::Rc<IUnknown>   { fn as_winapi(&self) -> &IUnknown { &**self } }
    unsafe impl Sealed for super::Unknown       { fn as_winapi(&self) -> &IUnknown { &*self.0 } }
}

/// Like [std::mem::drop], but use `debug_assert_eq!` to verify the refcount is
/// one for e.g. device lost handling purpouses.
///
/// This assumption may be violated by:
/// *   Middleware acquiring refcounts
/// *   Graphics debuggers acquiring refcounts
/// *   ???
pub fn drop_final(unk: impl AsUnknown) {
    let rc = unk.as_unk().approx_refcount();
    debug_assert_eq!(1, rc, "this wasn't the final object");
}



pub trait AsUnknown {
    fn as_unk(&self) -> &Unknown;
}

impl AsUnknown for Unknown {
    fn as_unk(&self) -> &Unknown { self }
}

impl<T: Deref> AsUnknown for T where T::Target : AsUnknown {
    fn as_unk(&self) -> &Unknown { (**self).as_unk() }
}
