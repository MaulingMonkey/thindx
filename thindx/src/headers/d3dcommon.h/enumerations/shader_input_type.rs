#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;

// not exposed by winapi 0.3.9
const D3D_SIT_RTACCELERATIONSTRUCTURE   : D3D_RESOURCE_RETURN_TYPE = 12;
const D3D_SIT_UAV_FEEDBACKTEXTURE       : D3D_RESOURCE_RETURN_TYPE = 13;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_type)\]
/// D3D_SHADER_INPUT_TYPE / D3D_SIT_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ShaderInputType(D3D_SHADER_INPUT_TYPE);
#[doc(hidden)] pub use ShaderInputType as SIT;

// Note: D3D10_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_* aliases D3D_* despite poor docs kinda implying otherwise (see d3dcommon.h)

enumish! {
    SIT => D3D_SHADER_INPUT_TYPE;
    default: CBuffer == 0;
    CBuffer, TBuffer, Texture, Sampler, UavRWTyped, Structured, UavRWStructured,
    ByteAddress, UavRWByteAddress, UavAppendStructured, UavConsumeStructured,
    UavRWStructuredWithCounter, RTAccelerationStructure, UavFeedbackTexture,
}

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl SIT { // These are enum-like
    pub const CBuffer                       : SIT = SIT(D3D_SIT_CBUFFER); // 0
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

//#cpp2rust D3D_SHADER_INPUT_TYPE                   = d3d::ShaderInputType

//#cpp2rust D3D_SIT_CBUFFER                         = d3d::SIT::CBuffer
//#cpp2rust D3D_SIT_TBUFFER                         = d3d::SIT::TBuffer
//#cpp2rust D3D_SIT_TEXTURE                         = d3d::SIT::Texture
//#cpp2rust D3D_SIT_SAMPLER                         = d3d::SIT::Sampler
//#cpp2rust D3D_SIT_UAV_RWTYPED                     = d3d::SIT::UavRWTyped
//#cpp2rust D3D_SIT_STRUCTURED                      = d3d::SIT::Structured
//#cpp2rust D3D_SIT_UAV_RWSTRUCTURED                = d3d::SIT::UavRWStructured
//#cpp2rust D3D_SIT_BYTEADDRESS                     = d3d::SIT::ByteAddress
//#cpp2rust D3D_SIT_UAV_RWBYTEADDRESS               = d3d::SIT::UavRWByteAddress
//#cpp2rust D3D_SIT_UAV_APPEND_STRUCTURED           = d3d::SIT::UavAppendStructured
//#cpp2rust D3D_SIT_UAV_CONSUME_STRUCTURED          = d3d::SIT::UavConsumeStructured
//#cpp2rust D3D_SIT_UAV_RWSTRUCTURED_WITH_COUNTER   = d3d::SIT::UavRWStructuredWithCounter
//#cpp2rust D3D_SIT_RTACCELERATIONSTRUCTURE         = d3d::SIT::RTAccelerationStructure
//#cpp2rust D3D_SIT_UAV_FEEDBACKTEXTURE             = d3d::SIT::UavFeedbackTexture

//#cpp2rust D3D10_SIT_CBUFFER                       = d3d::SIT::CBuffer
//#cpp2rust D3D10_SIT_TBUFFER                       = d3d::SIT::TBuffer
//#cpp2rust D3D10_SIT_TEXTURE                       = d3d::SIT::Texture
//#cpp2rust D3D10_SIT_SAMPLER                       = d3d::SIT::Sampler

//#cpp2rust D3D11_SIT_UAV_RWTYPED                   = d3d::SIT::UavRWTyped
//#cpp2rust D3D11_SIT_STRUCTURED                    = d3d::SIT::Structured
//#cpp2rust D3D11_SIT_UAV_RWSTRUCTURED              = d3d::SIT::UavRWStructured
//#cpp2rust D3D11_SIT_BYTEADDRESS                   = d3d::SIT::ByteAddress
//#cpp2rust D3D11_SIT_UAV_RWBYTEADDRESS             = d3d::SIT::UavRWByteAddress
//#cpp2rust D3D11_SIT_UAV_APPEND_STRUCTURED         = d3d::SIT::UavAppendStructured
//#cpp2rust D3D11_SIT_UAV_CONSUME_STRUCTURED        = d3d::SIT::UavConsumeStructured
//#cpp2rust D3D11_SIT_UAV_RWSTRUCTURED_WITH_COUNTER = d3d::SIT::UavRWStructuredWithCounter
