#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcompiler::*;

const D3D_BLOB_DEBUG_NAME : D3D_BLOB_PART = 12; // not part of winapi 0.3.9



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3d_blob_part)\]
/// D3D_BLOB_PART
///
/// Values that identify parts of the content of an arbitrary length data buffer.
///
/// ### Remarks
/// These values are passed to the D3DGetBlobPart or D3DSetBlobPart function.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BlobPart(D3D_BLOB_PART);
#[doc(hidden)] pub use BlobPart as Blob;

enumish! {
    BlobPart => D3D_BLOB_PART;
    InputSignatureBlob, OutputSignatureBlob, InputAndOutputSignatureBlob, PatchConstantSignatureBlob, AllSignatureBlob,
    DebugInfo, LegacyShader, XnaPrepassShader, XnaShader, Pdb, PrivateData, RootSignature, DebugName,
    TestAlternateShader, TestCompileDetails, TestCompilePerf, TestCompileReport,
}

#[allow(non_upper_case_globals)] impl Blob { // These are enum-like
    pub const InputSignatureBlob               : Blob = Blob(D3D_BLOB_INPUT_SIGNATURE_BLOB);
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
    /// <div class="note"><b>Note:</b> This value is supported by the d3dcompiler_44.dll or later version of the file.</div>
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "Blob::Pdb wasn't added until d3dcompiler_44.dll"))]
    pub const Pdb                              : Blob = Blob(D3D_BLOB_PDB);

    /// The blob part is private data.
    ///
    /// <div class="note"><b>Note:</b> This value is supported by the d3dcompiler_44.dll or later version of the file.</div>
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "Blob::PrivateData wasn't added until d3dcompiler_44.dll"))]
    pub const PrivateData                      : Blob = Blob(D3D_BLOB_PRIVATE_DATA);

    /// The blob part is a root signature. Refer to [Specifying Root Signatures in HLSL] for more information on using Direct3D12 with HLSL.
    ///
    /// <div class="note"><b>Note:</b> This value is supported by the d3dcompiler_47.dll or later version of the file.</div>
    ///
    /// [Specifying Root Signatures in HLSL]:   https://docs.microsoft.com/en-us/windows/desktop/direct3d12/specifying-root-signatures-in-hlsl
    #[cfg_attr(not(d3dcompiler="47"), deprecated(note = "Blob::RootSignature wasn't added until d3dcompiler_47.dll"))]
    pub const RootSignature                    : Blob = Blob(D3D_BLOB_ROOT_SIGNATURE);

    /// The blob part is the debug name of the shader. If the application does not specify the debug name itself,
    /// an auto-generated name matching the PDB file of the shader is provided instead.
    ///
    /// <div class="note"><b>Note:</b> This value is supported by the d3dcompiler_47.dll, as available on the Windows 10 Fall Creators Update and its SDK, or later version of the file.</div>
    #[cfg_attr(not(d3dcompiler="47"), deprecated(note = "Blob::DebugName wasn't added until d3dcompiler_47.dll"))]
    pub const DebugName                        : Blob = Blob(D3D_BLOB_DEBUG_NAME);

    /// <div class="note"><b>Note:</b> This value identifies a test part and is only produced by special compiler versions. Therefore, this part type is typically not present in shaders.</div>
    #[cfg_attr(not(d3dcompiler="test"), deprecated(note = "Blob::TestAlternateShader is only produced by special compiler versions."))]
    pub const TestAlternateShader              : Blob = Blob(D3D_BLOB_TEST_ALTERNATE_SHADER);


    /// <div class="note"><b>Note:</b> This value identifies a test part and is only produced by special compiler versions. Therefore, this part type is typically not present in shaders.</div>
    #[cfg_attr(not(d3dcompiler="test"), deprecated(note = "Blob::TestCompileDetails is only produced by special compiler versions."))]
    pub const TestCompileDetails               : Blob = Blob(D3D_BLOB_TEST_COMPILE_DETAILS);


    /// <div class="note"><b>Note:</b> This value identifies a test part and is only produced by special compiler versions. Therefore, this part type is typically not present in shaders.</div>
    #[cfg_attr(not(d3dcompiler="test"), deprecated(note = "Blob::TestCompilePerf is only produced by special compiler versions."))]
    pub const TestCompilePerf                  : Blob = Blob(D3D_BLOB_TEST_COMPILE_PERF);


    /// <div class="note"><b>Note:</b> This value identifies a test part and is only produced by special compiler versions. Therefore, this part type is typically not present in shaders.</div>
    #[cfg_attr(not(d3dcompiler="test"), deprecated(note = "Blob::TestCompileReport is only produced by special compiler versions."))]
    pub const TestCompileReport                : Blob = Blob(D3D_BLOB_TEST_COMPILE_REPORT);
}

#[doc(hidden)] impl Blob { // Ctrl+C Ctrl+V support
    pub const INPUT_SIGNATURE_BLOB             : Blob = Blob(D3D_BLOB_INPUT_SIGNATURE_BLOB);
    pub const OUTPUT_SIGNATURE_BLOB            : Blob = Blob(D3D_BLOB_OUTPUT_SIGNATURE_BLOB);
    pub const INPUT_AND_OUTPUT_SIGNATURE_BLOB  : Blob = Blob(D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB);
    pub const PATCH_CONSTANT_SIGNATURE_BLOB    : Blob = Blob(D3D_BLOB_PATCH_CONSTANT_SIGNATURE_BLOB);
    pub const ALL_SIGNATURE_BLOB               : Blob = Blob(D3D_BLOB_ALL_SIGNATURE_BLOB);
    pub const DEBUG_INFO                       : Blob = Blob(D3D_BLOB_DEBUG_INFO);
    pub const LEGACY_SHADER                    : Blob = Blob(D3D_BLOB_LEGACY_SHADER);
    pub const XNA_PREPASS_SHADER               : Blob = Blob(D3D_BLOB_XNA_PREPASS_SHADER);
    pub const XNA_SHADER                       : Blob = Blob(D3D_BLOB_XNA_SHADER);

    /// The blob part is program database (PDB) information.
    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "Blob::PDB wasn't added until d3dcompiler_44.dll"))]
    pub const PDB                              : Blob = Blob(D3D_BLOB_PDB);

    #[cfg_attr(not(d3dcompiler="44"), deprecated(note = "Blob::PRIVATE_DATA wasn't added until d3dcompiler_44.dll"))]
    pub const PRIVATE_DATA                     : Blob = Blob(D3D_BLOB_PRIVATE_DATA);

    /// The blob part is a root signature. Refer to [Specifying Root Signatures in HLSL] for more information on using Direct3D12 with HLSL.
    ///
    /// [Specifying Root Signatures in HLSL]:   https://docs.microsoft.com/en-us/windows/desktop/direct3d12/specifying-root-signatures-in-hlsl
    #[cfg_attr(not(d3dcompiler="47"), deprecated(note = "Blob::ROOT_SIGNATURE wasn't added until d3dcompiler_47.dll"))]
    pub const ROOT_SIGNATURE                   : Blob = Blob(D3D_BLOB_ROOT_SIGNATURE);

    /// The blob part is the debug name of the shader. If the application does not specify the debug name itself,
    /// an auto-generated name matching the PDB file of the shader is provided instead.
    #[cfg_attr(not(d3dcompiler="47"), deprecated(note = "Blob::DEBUG_NAME wasn't added until d3dcompiler_47.dll"))]
    pub const DEBUG_NAME                       : Blob = Blob(D3D_BLOB_DEBUG_NAME);

    #[cfg_attr(not(d3dcompiler="test"), deprecated(note = "Blob::TEST_ALTERNATE_SHADER is only produced by special compiler versions."))]
    pub const TEST_ALTERNATE_SHADER            : Blob = Blob(D3D_BLOB_TEST_ALTERNATE_SHADER);

    #[cfg_attr(not(d3dcompiler="test"), deprecated(note = "Blob::TEST_COMPILE_DETAILS is only produced by special compiler versions."))]
    pub const TEST_COMPILE_DETAILS             : Blob = Blob(D3D_BLOB_TEST_COMPILE_DETAILS);

    #[cfg_attr(not(d3dcompiler="test"), deprecated(note = "Blob::TEST_COMPILE_PERF is only produced by special compiler versions."))]
    pub const TEST_COMPILE_PERF                : Blob = Blob(D3D_BLOB_TEST_COMPILE_PERF);

    #[cfg_attr(not(d3dcompiler="test"), deprecated(note = "Blob::TEST_COMPILE_REPORT is only produced by special compiler versions."))]
    pub const TEST_COMPILE_REPORT              : Blob = Blob(D3D_BLOB_TEST_COMPILE_REPORT);
}

impl Default for Blob {
    fn default() -> Self { Blob(0) }
}
