use minidl::*;

use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;

use winapi::um::d3dcommon::*;

mod compile;        pub use compile::*;
mod compile2;       pub use compile2::*;



/// Lazily-loaded `d3dcompiler_NN.dll`
#[allow(non_snake_case)] // fn ptrs
pub struct D3DCompiler {
    pub(crate) D3DCompile: unsafe extern "system" fn (
        pSrcData:       LPCVOID,
        SrcDataSize:    SIZE_T,
        pSourceName:    LPCSTR,
        pDefines:       *const D3D_SHADER_MACRO,
        pInclude:       *mut ID3DInclude,
        pEntrypoint:    LPCSTR,
        pTarget:        LPCSTR,
        Flags1:         UINT,
        Flags2:         UINT,
        ppCode:         *mut *mut ID3DBlob,
        ppErrorMsgs:    *mut *mut ID3DBlob,
    ) -> HRESULT,
    pub(crate) D3DCompile2: unsafe extern "system" fn (
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        pSourceName:        LPCSTR,
        pDefines:           *const D3D_SHADER_MACRO,
        pInclude:           *mut ID3DInclude,
        pEntrypoint:        LPCSTR,
        pTarget:            LPCSTR,
        Flags1:             UINT,
        Flags2:             UINT,
        SecondaryDataFlags: UINT,
        pSecondaryData:     LPCVOID,
        SecondaryDataSize:  SIZE_T,
        ppCode:             *mut *mut ID3DBlob,
        ppErrorMsgs:        *mut *mut ID3DBlob,
    ) -> HRESULT,
}

impl D3DCompiler {
    pub fn new(version: u32) -> Result<Self> {
        let name = format!("d3dcompiler_{}.dll", version);
        let lib = Library::load(name)?;
        unsafe{Ok(Self{
            D3DCompile:             lib.sym("D3DCompile\0")?,
            D3DCompile2:            lib.sym("D3DCompile2\0")?,
        })}
    }
}



#[test] fn d3dcompiler_47() {
    let _d3dc47 = D3DCompiler::new(47).unwrap();
}

#[should_panic] #[test] fn d3dcompiler_48() {
    let _d3dc48 = D3DCompiler::new(48).unwrap();
}
