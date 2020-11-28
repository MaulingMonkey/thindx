use minidl::*;

use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;

use winapi::um::d3dcommon::*;
use winapi::um::d3dcompiler::*;

mod compile_from_file;              pub use compile_from_file::*;
mod compile;                        pub use compile::*;
mod compile2;                       pub use compile2::*;
mod compress_shaders;               pub use compress_shaders::*;


/// Lazily-loaded `d3dcompiler_NN.dll`
#[allow(non_snake_case)] // fn ptrs
pub struct D3DCompiler {
    // d3dcompiler_33.dll only exposes:
    //      D3DCompileFromMemory
    //      D3DDisassembleCode
    //      D3DDisassembleEffect
    //      D3DGetCodeDebugInfo
    //      D3DGetInputAndOutputSignatureBlob
    //      D3DGetInputSignatureBlob
    //      D3DGetOutputSignatureBlob
    //      D3DPreprocessFromMemory
    //      D3DReflectCode
    //      DebugSetMute

    // ..= D3DCompiler_39.dll
    // D3DCompileFromMemory

    /// D3DCompiler_40.dll ..=
    pub(crate) D3DCompile: Option<unsafe extern "system" fn (
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
    ) -> HRESULT>,

    /// D3DCompiler_47.dll
    /// Possibly as early as D3DCompiler_44.dll ?  Need more install
    pub(crate) D3DCompile2: Option<unsafe extern "system" fn (
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
    ) -> HRESULT>,

    /// D3DCompiler_47.dll
    /// Possibly as early as D3DCompiler_44.dll ?  Need more install
    pub(crate) D3DCompileFromFile: Option<unsafe extern "system" fn (
        pFileName:          LPCWSTR,
        pDefines:           *const D3D_SHADER_MACRO,
        pInclude:           *mut ID3DInclude,
        pEntrypoint:        LPCSTR,
        pTarget:            LPCSTR,
        Flags1:             UINT,
        Flags2:             UINT,
        ppCode:             *mut *mut ID3DBlob,
        ppErrorMsgs:        *mut *mut ID3DBlob,
    ) -> HRESULT>,

    pub(crate) D3DCompressShaders: Option<unsafe extern "system" fn (
        uNumShaders:        UINT,
        pShaderData:        *mut D3D_SHADER_DATA,
        uFlags:             UINT,
        ppCompressedData:   *mut *mut ID3DBlob,
    ) -> HRESULT>,
}

impl D3DCompiler {
    pub fn new(version: u32) -> Result<Self> {
        let name = format!("d3dcompiler_{}.dll", version);
        let lib = Library::load(name)?;
        unsafe{Ok(Self{
            D3DCompile:             lib.sym_opt("D3DCompile\0"),
            D3DCompile2:            lib.sym_opt("D3DCompile2\0"),
            D3DCompileFromFile:     lib.sym_opt("D3DCompileFromFile\0"),
            D3DCompressShaders:     lib.sym_opt("D3DCompressShaders\0"),
        })}
    }
}


#[test] fn d3dcompiler_nn() {
    for nn in [
        // All versions I have installed
        33, 34, 35, 36, 37, 38, 38, 40, 41, 42, 43, 47,
    ].iter().copied() {
        println!("loading d3dcompiler_{}.dll", nn);
        let _ = D3DCompiler::new(nn).unwrap();
    }
}

#[test] fn d3dcompiler_47() {
    let _d3dc47 = D3DCompiler::new(47).unwrap();
}

#[should_panic] #[test] fn d3dcompiler_48() {
    let _d3dc48 = D3DCompiler::new(48).unwrap();
}
