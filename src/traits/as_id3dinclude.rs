use winapi::um::d3dcommon::ID3DInclude;
use winapi::um::d3dcompiler::D3D_COMPILE_STANDARD_FILE_INCLUDE;

use std::ptr::*;



/// ### Safety
///
/// *   By implementing this trait, you promise to return either null, or a valid, well behaved `ID3DInclude` instance.
pub unsafe trait AsID3DInclude {
    fn as_id3dinclude(&self) -> *mut ID3DInclude;
}

unsafe impl AsID3DInclude for () {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { null_mut() }
}

pub struct StandardFileInclude;
unsafe impl AsID3DInclude for StandardFileInclude {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { D3D_COMPILE_STANDARD_FILE_INCLUDE }
}
