use winapi::um::d3dcompiler::*;

use std::marker::PhantomData;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ns-d3dcompiler-d3d_shader_data)\]
/// D3D_SHADER_DATA
///
/// Describes shader data.
#[derive(Clone, Copy)]
#[repr(C)] pub struct ShaderData<'d> {
    data:               D3D_SHADER_DATA,
    phantom_data:       PhantomData<&'d [u8]>,
}

impl<'d> ShaderData<'d> {
    pub fn bytes(&self) -> &'d [u8] {
        unsafe { std::slice::from_raw_parts(self.data.pBytecode.cast(), self.data.BytecodeLength) }
    }
}

impl AsRef<[u8]> for ShaderData<'_> {
    fn as_ref(&self) -> &[u8] { self.bytes() }
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

#[test] fn layout() {
    assert_eq!(std::mem::align_of::<ShaderData>(), std::mem::align_of::<D3D_SHADER_DATA>());
    assert_eq!(std::mem::size_of ::<ShaderData>(), std::mem::size_of ::<D3D_SHADER_DATA>());
}
