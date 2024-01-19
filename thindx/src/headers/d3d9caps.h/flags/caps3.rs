use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dcaps3)\]
/// D3DCAPS3_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Caps3(DWORD);

flags! { Caps3 => DWORD; None, AlphaFullscreenFlipOrDiscard, CopyToVidMem, CopyToSystemMem, DxvaHD, LinearToSrgbPresentation, Reserved }

#[allow(non_upper_case_globals)] impl Caps3 {
    pub const None                          : Caps3 = Caps3(0);
    pub const AlphaFullscreenFlipOrDiscard  : Caps3 = Caps3(D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD);
    pub const CopyToVidMem                  : Caps3 = Caps3(D3DCAPS3_COPY_TO_VIDMEM);
    pub const CopyToSystemMem               : Caps3 = Caps3(D3DCAPS3_COPY_TO_SYSTEMMEM);
    pub const DxvaHD                        : Caps3 = Caps3(D3DCAPS3_DXVAHD);
    pub const LinearToSrgbPresentation      : Caps3 = Caps3(D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION);
    #[doc(hidden)]
    pub const Reserved                      : Caps3 = Caps3(D3DCAPS3_RESERVED);
}

//#cpp2rust D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD   = d3d::Caps3::AlphaFullscreenFlipOrDiscard
//#cpp2rust D3DCAPS3_COPY_TO_VIDMEM                     = d3d::Caps3::CopyToVidMem
//#cpp2rust D3DCAPS3_COPY_TO_SYSTEMMEM                  = d3d::Caps3::CopyToSystemMem
//#cpp2rust D3DCAPS3_DXVAHD                             = d3d::Caps3::DxvaHD
//#cpp2rust D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION        = d3d::Caps3::LinearToSrgbPresentation
//#cpp2ignore D3DCAPS3_RESERVED
