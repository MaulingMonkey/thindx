#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;

// not exposed by winapi 0.3.9
const D3D_SIT_RTACCELERATIONSTRUCTURE   : D3D_RESOURCE_RETURN_TYPE = 12;
const D3D_SIT_UAV_FEEDBACKTEXTURE       : D3D_RESOURCE_RETURN_TYPE = 13;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_type)\]
/// D3D_SHADER_INPUT_TYPE / D3D_SIT_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderInputType(D3D_SHADER_INPUT_TYPE);
#[doc(hidden)] pub use ShaderInputType as SIT;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! {
    SIT => D3D_SHADER_INPUT_TYPE;
    CBuffer, TBuffer, Texture, Sampler, UavRWTyped, Structured, UavRWStructured,
    ByteAddress, UavRWByteAddress, UavAppendStructured, UavConsumeStructured,
    UavRWStructuredWithCounter, RTAccelerationStructure, UavFeedbackTexture,
}

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl SIT { // These are enum-like
    pub const CBuffer                       : SIT = SIT(D3D_SIT_CBUFFER);
    pub const TBuffer                       : SIT = SIT(D3D_SIT_TBUFFER);
    pub const Texture                       : SIT = SIT(D3D_SIT_TEXTURE);
    pub const Sampler                       : SIT = SIT(D3D_SIT_SAMPLER);
    pub const UavRWTyped                    : SIT = SIT(D3D_SIT_UAV_RWTYPED);
    pub const Structured                    : SIT = SIT(D3D_SIT_STRUCTURED);
    pub const UavRWStructured               : SIT = SIT(D3D_SIT_UAV_RWSTRUCTURED);
    pub const ByteAddress                   : SIT = SIT(D3D_SIT_BYTEADDRESS);
    pub const UavRWByteAddress              : SIT = SIT(D3D_SIT_UAV_RWBYTEADDRESS);
    pub const UavAppendStructured           : SIT = SIT(D3D_SIT_UAV_APPEND_STRUCTURED);
    pub const UavConsumeStructured          : SIT = SIT(D3D_SIT_UAV_CONSUME_STRUCTURED);
    pub const UavRWStructuredWithCounter    : SIT = SIT(D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER);
    pub const RTAccelerationStructure       : SIT = SIT(D3D_SIT_RTACCELERATIONSTRUCTURE);
    pub const UavFeedbackTexture            : SIT = SIT(D3D_SIT_UAV_FEEDBACKTEXTURE);
}

#[doc(hidden)] impl SIT { // Ctrl+C Ctrl+V support
    pub const CBUFFER                       : SIT = SIT(D3D_SIT_CBUFFER);
    pub const TBUFFER                       : SIT = SIT(D3D_SIT_TBUFFER);
    pub const TEXTURE                       : SIT = SIT(D3D_SIT_TEXTURE);
    pub const SAMPLER                       : SIT = SIT(D3D_SIT_SAMPLER);
    pub const UAV_RWTYPED                   : SIT = SIT(D3D_SIT_UAV_RWTYPED);
    pub const STRUCTURED                    : SIT = SIT(D3D_SIT_STRUCTURED);
    pub const UAV_RWSTRUCTURED              : SIT = SIT(D3D_SIT_UAV_RWSTRUCTURED);
    pub const BYTEADDRESS                   : SIT = SIT(D3D_SIT_BYTEADDRESS);
    pub const UAV_RWBYTEADDRESS             : SIT = SIT(D3D_SIT_UAV_RWBYTEADDRESS);
    pub const UAV_APPEND_STRUCTURED         : SIT = SIT(D3D_SIT_UAV_APPEND_STRUCTURED);
    pub const UAV_CONSUME_STRUCTURED        : SIT = SIT(D3D_SIT_UAV_CONSUME_STRUCTURED);
    pub const UAV_RWSTRUCTURED_WITH_COUNTER : SIT = SIT(D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER);
    pub const RTACCELERATIONSTRUCTURE       : SIT = SIT(D3D_SIT_RTACCELERATIONSTRUCTURE);
    pub const UAV_FEEDBACKTEXTURE           : SIT = SIT(D3D_SIT_UAV_FEEDBACKTEXTURE);
}

impl Default for SIT {
    fn default() -> Self { SIT(0) }
}
