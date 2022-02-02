#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcommon::*;

const D3D_FEATURE_LEVEL_1_0_CORE : D3D_FEATURE_LEVEL = 0x1000; // Not in winapi (yet?)



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_feature_level)\]
/// D3D_FEATURE_LEVEL / D3D_FEATURE_LEVEL_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct FeatureLevel(D3D_FEATURE_LEVEL);

enumish! { FeatureLevel => D3D_FEATURE_LEVEL; _1_0_Core, _9_1, _9_2, _9_3, _10_0, _10_1, _11_0, _11_1, _12_0, _12_1 }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl FeatureLevel { // These are enum-like
    pub const _1_0_Core : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_1_0_CORE);
    pub const _9_1      : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_9_1);
    pub const _9_2      : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_9_2);
    pub const _9_3      : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_9_3);
    pub const _10_0     : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_10_0);
    pub const _10_1     : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_10_1);
    pub const _11_0     : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_11_0);
    pub const _11_1     : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_11_1);
    pub const _12_0     : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_12_0);
    pub const _12_1     : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_12_1);
}

#[doc(hidden)] impl FeatureLevel { // Ctrl+C Ctrl+V support
    pub const _1_0_CORE : FeatureLevel = FeatureLevel(D3D_FEATURE_LEVEL_1_0_CORE);
}

impl Default for FeatureLevel {
    fn default() -> Self { FeatureLevel(0) }
}

//#cpp2rust D3D_FEATURE_LEVEL           = d3d::FeatureLevel
//#cpp2rust D3D_FEATURE_LEVEL_1_0_CORE  = d3d::FeatureLevel::_1_0_Core
//#cpp2rust D3D_FEATURE_LEVEL_9_1       = d3d::FeatureLevel::_9_1
//#cpp2rust D3D_FEATURE_LEVEL_9_2       = d3d::FeatureLevel::_9_2
//#cpp2rust D3D_FEATURE_LEVEL_9_3       = d3d::FeatureLevel::_9_3
//#cpp2rust D3D_FEATURE_LEVEL_10_0      = d3d::FeatureLevel::_10_0
//#cpp2rust D3D_FEATURE_LEVEL_10_1      = d3d::FeatureLevel::_10_1
//#cpp2rust D3D_FEATURE_LEVEL_11_0      = d3d::FeatureLevel::_11_0
//#cpp2rust D3D_FEATURE_LEVEL_11_1      = d3d::FeatureLevel::_11_1
//#cpp2rust D3D_FEATURE_LEVEL_12_0      = d3d::FeatureLevel::_12_0
//#cpp2rust D3D_FEATURE_LEVEL_12_1      = d3d::FeatureLevel::_12_1
