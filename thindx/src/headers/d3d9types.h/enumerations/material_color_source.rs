#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterialcolorsource)\]
/// D3DMATERIALCOLORSOURCE
///
/// Defines the location at which a color or color component must be accessed for lighting calculations.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct MaterialColorSource(D3DMATERIALCOLORSOURCE);
pub use MaterialColorSource as MCS;

enumish! { MCS => D3DMATERIALCOLORSOURCE; Material, Color1, Color2 }

#[allow(non_upper_case_globals)] impl MaterialColorSource { // These are enum-like
    pub const Material  : MaterialColorSource = MaterialColorSource(D3DMCS_MATERIAL); // 0
    pub const Color1    : MaterialColorSource = MaterialColorSource(D3DMCS_COLOR1);
    pub const Color2    : MaterialColorSource = MaterialColorSource(D3DMCS_COLOR2);
}

impl Default for MaterialColorSource {
    fn default() -> Self { MaterialColorSource::Material } // 0
}

//#cpp2rust D3DMATERIALCOLORSOURCE  = d3d::MaterialColorSource
//#cpp2rust D3DMCS_MATERIAL         = d3d::MCS::Material
//#cpp2rust D3DMCS_COLOR1           = d3d::MCS::Color1
//#cpp2rust D3DMCS_COLOR2           = d3d::MCS::Color2
