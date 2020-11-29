use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_library_desc)\]
/// D3D11_LIBRARY_DESC
///
/// &amp;\[[u8]\] equivalent that's ABI-compatible with some D3D APIs
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct LibraryDesc<'s> {
    creator:                        Option<CStrPtr<'s>>,
    flags:                          u32,
    function_count:                 u32,
}

test_layout! { LibraryDesc => unsafe D3D11_LIBRARY_DESC {
    creator                         => Creator,
    flags                           => Flags,
    function_count                  => FunctionCount,
}}
