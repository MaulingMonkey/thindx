use crate::d3d11::*;

use winapi::um::d3d11shader::*;

use std::marker::PhantomData;
use std::ptr::NonNull;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionreflection)\]
/// ID3D11FunctionReflection
///
/// A function-reflection interface accesses function info.
#[derive(Clone)] #[repr(transparent)]
pub struct FunctionReflection<'r> {
    ptr:        NonNull<ID3D11FunctionReflection>,
    phantom:    PhantomData<&'r LibraryReflection>,
}

// TODO: macrofiy this boilerplate too?
impl<'r> FunctionReflection<'r> {
    pub unsafe fn from_raw(_: &'r LibraryReflection, fr: *mut ID3D11FunctionReflection) -> Option<Self> {
        Some(Self {
            ptr:        NonNull::new(fr)?,
            phantom:    PhantomData,
        })
    }
}

// TODO: methods
