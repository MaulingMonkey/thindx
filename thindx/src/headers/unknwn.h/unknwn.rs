use std::ops::Deref;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)\]
/// IUnknown
///
/// Base COM interface most (but not all!) DirectX interfaces eventually derive from.
#[derive(Clone)] #[repr(transparent)]
pub struct Unknown(pub(crate) mcom::Rc<winapi::um::unknwnbase::IUnknown>);

convert!(unsafe Unknown, winapi::um::unknwnbase::IUnknown);



/// Auto trait implemented for anything that can chain-[Deref] to [thindx::Unknown](crate::Unknown).
pub trait AsUnknown {
    fn as_unk(&self) -> &Unknown;
}

impl AsUnknown for Unknown {
    fn as_unk(&self) -> &Unknown { self }
}

impl<T: Deref> AsUnknown for T where T::Target : AsUnknown {
    fn as_unk(&self) -> &Unknown { (**self).as_unk() }
}
