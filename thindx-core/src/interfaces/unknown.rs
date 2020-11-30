use crate::*;
use std::ops::Deref;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)\]
/// Base COM interface all D3D9 types eventually derive from.
#[derive(Clone)] #[repr(transparent)]
pub struct Unknown(pub(crate) mcom::Rc<winapi::um::unknwnbase::IUnknown>);



/// Auto trait implemented for anything that can chain-[Deref] to [thin3dcompiler::Unknown](crate::Unknown).
pub trait AsUnknown {
    fn as_unk(&self) -> &Unknown;
}

impl AsUnknown for Unknown {
    fn as_unk(&self) -> &Unknown { self }
}

impl<T: Deref> AsUnknown for T where T::Target : AsUnknown {
    fn as_unk(&self) -> &Unknown { (**self).as_unk() }
}

convert!(unsafe Unknown, winapi::um::unknwnbase::IUnknown);
