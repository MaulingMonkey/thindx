#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;

// missing from winapi
const D3D_NAME_BARYCENTRICS     : D3D_NAME = 23;
const D3D_NAME_SHADINGRATE      : D3D_NAME = 24;
const D3D_NAME_CULLPRIMITIVE    : D3D_NAME = 25;
const D3D_NAME_STENCIL_REF      : D3D_NAME = 69;
const D3D_NAME_INNER_COVERAGE   : D3D_NAME = 70;

// Note: D3D10_NAME_* aliases D3D_NAME_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D11_NAME_* aliases D3D_NAME_* despite poor docs kinda implying otherwise (see d3dcommon.h)
// Note: D3D12_NAME_* aliases D3D_NAME_* despite poor docs kinda implying otherwise (see d3dcommon.h)



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_name)\]
/// D3D_NAME
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Name(D3D_NAME);

enumish! {
    Name => D3D_NAME;
    Undefined, Position, ClipDistance, CullDistance, RenderTargetArrayIndex, ViewportArrayIndex, VertexId, PrimitiveId,
    InstanceId, IsFrontFace, SampleIndex, FinalQuadEdgeTessFactor, FinalQuadInsideTessFactor, FinalTriEdgeTessFactor,
    FinalTriInsideTessFactor, FinalLineDetailTessFactor, FinalLineDensityTessFactor, Barycentrics, ShadingRate,
    CullPrimitive, Target, Depth, Coverage, DepthGreaterEqual, DepthLessEqual, StencilRef, InnerCoverage,
}

#[allow(non_upper_case_globals)] impl Name { // These are enum-like
    pub const Undefined                        : Name = Name(D3D_NAME_UNDEFINED); // 0
    pub const Position                         : Name = Name(D3D_NAME_POSITION);
    pub const ClipDistance                     : Name = Name(D3D_NAME_CLIP_DISTANCE);
    pub const CullDistance                     : Name = Name(D3D_NAME_CULL_DISTANCE);
    pub const RenderTargetArrayIndex           : Name = Name(D3D_NAME_RENDER_TARGET_ARRAY_INDEX);
    pub const ViewportArrayIndex               : Name = Name(D3D_NAME_VIEWPORT_ARRAY_INDEX);
    pub const VertexId                         : Name = Name(D3D_NAME_VERTEX_ID);
    pub const PrimitiveId                      : Name = Name(D3D_NAME_PRIMITIVE_ID);
    pub const InstanceId                       : Name = Name(D3D_NAME_INSTANCE_ID);
    pub const IsFrontFace                      : Name = Name(D3D_NAME_IS_FRONT_FACE);
    pub const SampleIndex                      : Name = Name(D3D_NAME_SAMPLE_INDEX);
    pub const FinalQuadEdgeTessFactor          : Name = Name(D3D_NAME_FINAL_QUAD_EDGE_TESSFACTOR);
    pub const FinalQuadInsideTessFactor        : Name = Name(D3D_NAME_FINAL_QUAD_INSIDE_TESSFACTOR);
    pub const FinalTriEdgeTessFactor           : Name = Name(D3D_NAME_FINAL_TRI_EDGE_TESSFACTOR);
    pub const FinalTriInsideTessFactor         : Name = Name(D3D_NAME_FINAL_TRI_INSIDE_TESSFACTOR);
    pub const FinalLineDetailTessFactor        : Name = Name(D3D_NAME_FINAL_LINE_DETAIL_TESSFACTOR);
    pub const FinalLineDensityTessFactor       : Name = Name(D3D_NAME_FINAL_LINE_DENSITY_TESSFACTOR);
    pub const Barycentrics                     : Name = Name(D3D_NAME_BARYCENTRICS);
    pub const ShadingRate                      : Name = Name(D3D_NAME_SHADINGRATE);
    pub const CullPrimitive                    : Name = Name(D3D_NAME_CULLPRIMITIVE);
    pub const Target                           : Name = Name(D3D_NAME_TARGET);
    pub const Depth                            : Name = Name(D3D_NAME_DEPTH);
    pub const Coverage                         : Name = Name(D3D_NAME_COVERAGE);
    pub const DepthGreaterEqual                : Name = Name(D3D_NAME_DEPTH_GREATER_EQUAL);
    pub const DepthLessEqual                   : Name = Name(D3D_NAME_DEPTH_LESS_EQUAL);
    pub const StencilRef                       : Name = Name(D3D_NAME_STENCIL_REF);
    pub const InnerCoverage                    : Name = Name(D3D_NAME_INNER_COVERAGE);
}

#[doc(hidden)] impl Name { // Ctrl+C Ctrl+V support
    pub const UNDEFINED                        : Name = Name(D3D_NAME_UNDEFINED); // 0
    pub const POSITION                         : Name = Name(D3D_NAME_POSITION);
    pub const CLIP_DISTANCE                    : Name = Name(D3D_NAME_CLIP_DISTANCE);
    pub const CULL_DISTANCE                    : Name = Name(D3D_NAME_CULL_DISTANCE);
    pub const RENDER_TARGET_ARRAY_INDEX        : Name = Name(D3D_NAME_RENDER_TARGET_ARRAY_INDEX);
    pub const VIEWPORT_ARRAY_INDEX             : Name = Name(D3D_NAME_VIEWPORT_ARRAY_INDEX);
    pub const VERTEX_ID                        : Name = Name(D3D_NAME_VERTEX_ID);
    pub const PRIMITIVE_ID                     : Name = Name(D3D_NAME_PRIMITIVE_ID);
    pub const INSTANCE_ID                      : Name = Name(D3D_NAME_INSTANCE_ID);
    pub const IS_FRONT_FACE                    : Name = Name(D3D_NAME_IS_FRONT_FACE);
    pub const SAMPLE_INDEX                     : Name = Name(D3D_NAME_SAMPLE_INDEX);
    pub const FINAL_QUAD_EDGE_TESSFACTOR       : Name = Name(D3D_NAME_FINAL_QUAD_EDGE_TESSFACTOR);
    pub const FINAL_QUAD_INSIDE_TESSFACTOR     : Name = Name(D3D_NAME_FINAL_QUAD_INSIDE_TESSFACTOR);
    pub const FINAL_TRI_EDGE_TESSFACTOR        : Name = Name(D3D_NAME_FINAL_TRI_EDGE_TESSFACTOR);
    pub const FINAL_TRI_INSIDE_TESSFACTOR      : Name = Name(D3D_NAME_FINAL_TRI_INSIDE_TESSFACTOR);
    pub const FINAL_LINE_DETAIL_TESSFACTOR     : Name = Name(D3D_NAME_FINAL_LINE_DETAIL_TESSFACTOR);
    pub const FINAL_LINE_DENSITY_TESSFACTOR    : Name = Name(D3D_NAME_FINAL_LINE_DENSITY_TESSFACTOR);
    pub const BARYCENTRICS                     : Name = Name(D3D_NAME_BARYCENTRICS);
    pub const SHADINGRATE                      : Name = Name(D3D_NAME_SHADINGRATE);
    pub const CULLPRIMITIVE                    : Name = Name(D3D_NAME_CULLPRIMITIVE);
    pub const TARGET                           : Name = Name(D3D_NAME_TARGET);
    pub const DEPTH                            : Name = Name(D3D_NAME_DEPTH);
    pub const COVERAGE                         : Name = Name(D3D_NAME_COVERAGE);
    pub const DEPTH_GREATER_EQUAL              : Name = Name(D3D_NAME_DEPTH_GREATER_EQUAL);
    pub const DEPTH_LESS_EQUAL                 : Name = Name(D3D_NAME_DEPTH_LESS_EQUAL);
    pub const STENCIL_REF                      : Name = Name(D3D_NAME_STENCIL_REF);
    pub const INNER_COVERAGE                   : Name = Name(D3D_NAME_INNER_COVERAGE);
}


impl Default for Name {
    fn default() -> Self { Name::Undefined }
}
