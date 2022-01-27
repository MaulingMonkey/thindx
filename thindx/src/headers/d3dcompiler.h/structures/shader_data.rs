use winapi::um::d3dcompiler::*;

use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ns-d3dcompiler-d3d_shader_data)\]
/// D3D_SHADER_DATA
///
/// &amp;\[[u8]\] equivalent that's ABI-compatible with some D3D APIs
#[derive(Clone, Copy)]
#[repr(C)] pub struct ShaderData<'d> {
    data:               D3D_SHADER_DATA,
    phantom_data:       PhantomData<&'d [u8]>,
}

impl<'d> ShaderData<'d> {
    /// Get the data in question as a slice of bytes
    pub fn as_bytes(&self) -> &'d [u8] {
        // SAFETY: ✔️ pBytecode / ByteCodeLength were initially constructed from a `&'d [u8]`
        //  In the case of ABI transmutation, d3d is generally reading - not writing - this type, and u8s impose no alignment requirements.
        unsafe { std::slice::from_raw_parts(self.data.pBytecode.cast(), self.data.BytecodeLength) }
    }
}

impl AsRef<[u8]> for ShaderData<'_> {
    fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

impl Borrow<[u8]> for ShaderData<'_> {
    fn borrow(&self) -> &[u8] { self.as_bytes() }
}

impl<'d> From<&'d [u8]> for ShaderData<'d> {
    fn from(slice: &'d [u8]) -> Self {
        Self {
            data: D3D_SHADER_DATA {
                pBytecode:      slice.as_ptr().cast(),
                BytecodeLength: slice.len(),
            },
            phantom_data: PhantomData,
        }
    }
}

impl Default for ShaderData<'_> {
    fn default() -> Self {
        Self {
            data: D3D_SHADER_DATA {
                pBytecode:      null(),
                BytecodeLength: 0,
            },
            phantom_data: PhantomData,
        }
    }
}

impl Deref for ShaderData<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target { self.as_bytes() }
}

#[test] fn layout() {
    assert_eq!(std::mem::align_of::<ShaderData>(), std::mem::align_of::<D3D_SHADER_DATA>());
    assert_eq!(std::mem::size_of ::<ShaderData>(), std::mem::size_of ::<D3D_SHADER_DATA>());
}

//#cpp2rust D3D_SHADER_DATA                         = d3d::ShaderData
