#![warn(clippy::undocumented_unsafe_blocks)]

use crate::AsSafe;
use winapi::um::unknwnbase::IUnknown;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)\]
/// IUnknown
///
/// Base COM interface most (but not all!) DirectX interfaces eventually derive from.
#[derive(Clone)] #[repr(transparent)]
pub struct Unknown(pub(crate) mcom::Rc<IUnknown>);

// SAFETY: ✔️ yep, safe
convert!(unsafe Unknown, IUnknown);

unsafe impl AsSafe<IUnknown> for Unknown { fn as_safe(&self) -> &IUnknown { &*self.0 } }



/// Like [std::mem::drop], but use `debug_assert_eq!` to verify the refcount of a COM object is `1` for e.g. device lost handling purpouses.
///
/// This assumption may be violated by:
/// *   Middleware acquiring refcounts
/// *   Graphics debuggers acquiring refcounts
/// *   Release() not returning an accurate refcount
/// *   ???
//#allow_missing_argument_docs
#[allow(dead_code)] // XXX
pub(crate) fn drop_final(unk: impl AsSafe<IUnknown>) {
    let unk = unk.as_safe();
    // SAFETY: ✔️ initial refcount implied to be >= 1 by AsSafe bound, final refcount should be equivalent to initial refcount
    let rc = unsafe {
        unk.AddRef();
        unk.Release()
    };
    debug_assert_eq!(1, rc, "this wasn't the final object");
}

//#cpp2rust IUnknown                    = Unknown

//#cpp2rust IUnknown::AddRef            = mcom::Rc::clone
//#cpp2rust IUnknown::Release           = mcom::Rc::drop
//#cpp2rust IUnknown::QueryInterface    = mcom::Rc::try_cast
