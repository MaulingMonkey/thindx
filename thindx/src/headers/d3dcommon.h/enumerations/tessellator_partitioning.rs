#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_partitioning)\]
/// D3D_TESSELLATOR_PARTITIONING
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct TessellatorPartitioning(D3D_TESSELLATOR_PARTITIONING);

enumish! { TessellatorPartitioning => D3D_TESSELLATOR_PARTITIONING; default: Undefined == 0; Undefined, Integer, Pow2, FractionalOdd, FractionalEven }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl TessellatorPartitioning { // These are enum-like
    pub const Undefined         : TessellatorPartitioning = TessellatorPartitioning(D3D_TESSELLATOR_PARTITIONING_UNDEFINED); // 0
    pub const Integer           : TessellatorPartitioning = TessellatorPartitioning(D3D_TESSELLATOR_PARTITIONING_INTEGER);
    pub const Pow2              : TessellatorPartitioning = TessellatorPartitioning(D3D_TESSELLATOR_PARTITIONING_POW2);
    pub const FractionalOdd     : TessellatorPartitioning = TessellatorPartitioning(D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD);
    pub const FractionalEven    : TessellatorPartitioning = TessellatorPartitioning(D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN);
}



//#cpp2rust D3D_TESSELLATOR_PARTITIONING                    = d3d::TessellatorPartitioning

//#cpp2rust D3D_TESSELLATOR_PARTITIONING_UNDEFINED          = d3d::TessellatorPartitioning::Undefined
//#cpp2rust D3D_TESSELLATOR_PARTITIONING_INTEGER            = d3d::TessellatorPartitioning::Integer
//#cpp2rust D3D_TESSELLATOR_PARTITIONING_POW2               = d3d::TessellatorPartitioning::Pow2
//#cpp2rust D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD     = d3d::TessellatorPartitioning::FractionalOdd
//#cpp2rust D3D_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN    = d3d::TessellatorPartitioning::FractionalEven

//#cpp2rust D3D11_TESSELLATOR_PARTITIONING_UNDEFINED        = d3d::TessellatorPartitioning::Undefined
//#cpp2rust D3D11_TESSELLATOR_PARTITIONING_INTEGER          = d3d::TessellatorPartitioning::Integer
//#cpp2rust D3D11_TESSELLATOR_PARTITIONING_POW2             = d3d::TessellatorPartitioning::Pow2
//#cpp2rust D3D11_TESSELLATOR_PARTITIONING_FRACTIONAL_ODD   = d3d::TessellatorPartitioning::FractionalOdd
//#cpp2rust D3D11_TESSELLATOR_PARTITIONING_FRACTIONAL_EVEN  = d3d::TessellatorPartitioning::FractionalEven
