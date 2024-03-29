#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcompiler::*;

const D3D_BLOB_DEBUG_NAME : D3D_BLOB_PART = 12; // not part of winapi 0.3.9



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3d_blob_part)\]
/// D3D_BLOB_PART
///
/// Values that identify parts of the content of an arbitrary length data buffer.
///
/// ### Remarks
/// These values are passed to the [d3d::Compiler::get_blob_part] or [d3d::Compiler::set_blob_part] functions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct BlobPart(D3D_BLOB_PART);
#[doc(hidden)] pub use BlobPart as Blob;

enumish! {
    BlobPart => D3D_BLOB_PART;
    default: InputSignatureBlob == 0;
    InputSignatureBlob, OutputSignatureBlob, InputAndOutputSignatureBlob, PatchConstantSignatureBlob, AllSignatureBlob,
    DebugInfo, LegacyShader, XnaPrepassShader, XnaShader, Pdb, PrivateData, RootSignature, DebugName,
    TestAlternateShader, TestCompileDetails, TestCompilePerf, TestCompileReport,
}

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl Blob { // These are enum-like
    pub const InputSignatureBlob               : Blob = Blob(D3D_BLOB_INPUT_SIGNATURE_BLOB); // 0
    pub const OutputSignatureBlob              : Blob = Blob(D3D_BLOB_OUTPUT_SIGNATURE_BLOB);
    pub const InputAndOutputSignatureBlob      : Blob = Blob(D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB);
    pub const PatchConstantSignatureBlob       : Blob = Blob(D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB);
    pub const AllSignatureBlob                 : Blob = Blob(D3D_BLOB_ALL_SIGNATURE_BLOB);
    pub const DebugInfo                        : Blob = Blob(D3D_BLOB_DEBUG_INFO);
    pub const LegacyShader                     : Blob = Blob(D3D_BLOB_LEGACY_SHADER);
    pub const XnaPrepassShader                 : Blob = Blob(D3D_BLOB_XNA_PREPASS_SHADER);
    pub const XnaShader                        : Blob = Blob(D3D_BLOB_XNA_SHADER);

    /// The blob part is program database (PDB) information.
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    pub const Pdb                              : Blob = Blob(D3D_BLOB_PDB);

    /// The blob part is private data.
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_44.dll, and is unavailable in earlier versions.
    pub const PrivateData                      : Blob = Blob(D3D_BLOB_PRIVATE_DATA);

    /// The blob part is a root signature. Refer to [Specifying Root Signatures in HLSL] for more information on using Direct3D12 with HLSL.
    ///
    /// [Specifying Root Signatures in HLSL]:   https://learn.microsoft.com/en-us/windows/desktop/direct3d12/specifying-root-signatures-in-hlsl
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_47.dll, and is unavailable in earlier versions.
    pub const RootSignature                    : Blob = Blob(D3D_BLOB_ROOT_SIGNATURE);

    /// The blob part is the debug name of the shader. If the application does not specify the debug name itself,
    /// an auto-generated name matching the PDB file of the shader is provided instead.
    ///
    /// ### Remarks
    /// *   This was introduced by d3dcompiler_47.dll, and is unavailable in earlier versions.
    pub const DebugName                        : Blob = Blob(D3D_BLOB_DEBUG_NAME);

    /// ### Remarks
    /// *   This requires a special test version of d3dcompiler.
    pub const TestAlternateShader              : Blob = Blob(D3D_BLOB_TEST_ALTERNATE_SHADER);

    /// ### Remarks
    /// *   This requires a special test version of d3dcompiler.
    pub const TestCompileDetails               : Blob = Blob(D3D_BLOB_TEST_COMPILE_DETAILS);

    /// ### Remarks
    /// *   This requires a special test version of d3dcompiler.
    pub const TestCompilePerf                  : Blob = Blob(D3D_BLOB_TEST_COMPILE_PERF);

    /// ### Remarks
    /// *   This requires a special test version of d3dcompiler.
    pub const TestCompileReport                : Blob = Blob(D3D_BLOB_TEST_COMPILE_REPORT);
}

//#cpp2rust D3D_BLOB_PART                               = d3d::BlobPart

//#cpp2rust D3D_BLOB_INPUT_SIGNATURE_BLOB               = d3d::Blob::InputSignatureBlob
//#cpp2rust D3D_BLOB_OUTPUT_SIGNATURE_BLOB              = d3d::Blob::OutputSignatureBlob
//#cpp2rust D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB    = d3d::Blob::InputAndOutputSignatureBlob
//#cpp2rust D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB      = d3d::Blob::PatchConstantSignatureBlob
//#cpp2rust D3D_BLOB_ALL_SIGNATURE_BLOB                 = d3d::Blob::AllSignatureBlob
//#cpp2rust D3D_BLOB_DEBUG_INFO                         = d3d::Blob::DebugInfo
//#cpp2rust D3D_BLOB_LEGACY_SHADER                      = d3d::Blob::LegacyShader
//#cpp2rust D3D_BLOB_XNA_PREPASS_SHADER                 = d3d::Blob::XnaPrepassShader
//#cpp2rust D3D_BLOB_XNA_SHADER                         = d3d::Blob::XnaShader
//#cpp2rust D3D_BLOB_PDB                                = d3d::Blob::Pdb
//#cpp2rust D3D_BLOB_PRIVATE_DATA                       = d3d::Blob::PrivateData
//#cpp2rust D3D_BLOB_ROOT_SIGNATURE                     = d3d::Blob::RootSignature
//#cpp2rust D3D_BLOB_DEBUG_NAME                         = d3d::Blob::DebugName
//#cpp2rust D3D_BLOB_TEST_ALTERNATE_SHADER              = d3d::Blob::TestAlternateShader
//#cpp2rust D3D_BLOB_TEST_COMPILE_DETAILS               = d3d::Blob::TestCompileDetails
//#cpp2rust D3D_BLOB_TEST_COMPILE_PERF                  = d3d::Blob::TestCompilePerf
//#cpp2rust D3D_BLOB_TEST_COMPILE_REPORT                = d3d::Blob::TestCompileReport
