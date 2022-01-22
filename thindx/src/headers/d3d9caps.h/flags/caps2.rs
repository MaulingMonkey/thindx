use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcaps2)\]
/// D3DCAPS2_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct Caps2(DWORD);

flags! { Caps2 => DWORD; None, CanAutoGenMipMap, CanCalibrateGamma, CanShareResource, CanManageResource, DynamicTextures, FullScreenGamma, Reserved }

#[allow(non_upper_case_globals)] impl Caps2 {
    pub const None              : Caps2 = Caps2(0);
    pub const CanAutoGenMipMap  : Caps2 = Caps2(D3DCAPS2_CANAUTOGENMIPMAP);
    pub const CanCalibrateGamma : Caps2 = Caps2(D3DCAPS2_CANCALIBRATEGAMMA);
    pub const CanShareResource  : Caps2 = Caps2(D3DCAPS2_CANSHARERESOURCE);
    pub const CanManageResource : Caps2 = Caps2(D3DCAPS2_CANMANAGERESOURCE);
    pub const DynamicTextures   : Caps2 = Caps2(D3DCAPS2_DYNAMICTEXTURES);
    pub const FullScreenGamma   : Caps2 = Caps2(D3DCAPS2_FULLSCREENGAMMA);
    #[doc(hidden)]
    pub const Reserved          : Caps2 = Caps2(D3DCAPS2_RESERVED);
}

//#cpp2rust D3DCAPS2_CANAUTOGENMIPMAP       = d3d::Caps2::CanAutoGenMipMap
//#cpp2rust D3DCAPS2_CANCALIBRATEGAMMA      = d3d::Caps2::CanCalibrateGamma
//#cpp2rust D3DCAPS2_CANSHARERESOURCE       = d3d::Caps2::CanShareResource
//#cpp2rust D3DCAPS2_CANMANAGERESOURCE      = d3d::Caps2::CanManageResource
//#cpp2rust D3DCAPS2_DYNAMICTEXTURES        = d3d::Caps2::DynamicTextures
//#cpp2rust D3DCAPS2_FULLSCREENGAMMA        = d3d::Caps2::FullScreenGamma
//#cpp2ignore D3DCAPS2_RESERVED

