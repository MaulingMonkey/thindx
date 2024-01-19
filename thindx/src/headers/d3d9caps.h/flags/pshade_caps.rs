#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DPSHADECAPS_*
///
/// Shading operations capabilities.
/// It is assumed, in general, that if a device supports a given command at all, it supports the [d3d::Shade::Flat] mode (as specified in the [d3d::ShadeMode] enumerated type).
/// This flag specifies whether the driver can also support Gouraud shading and whether alpha color components are supported.
/// When alpha components are not supported, the alpha value of colors generated is implicitly 255.
/// This is the maximum possible alpha (that is, the alpha component is at full intensity).
///
/// The color, specular highlights, fog, and alpha interpolants of a triangle each have capability flags that an application can use to find out how they are implemented by the device driver.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct PShadeCaps(DWORD);

flags! {
    PShadeCaps => DWORD;
    None, AlphaGouraudBlend, ColorGouraudRgb, FogGouraud, SpecularGouraudRgb,
}

#[allow(non_upper_case_globals)] impl PShadeCaps {
    pub const None                          : PShadeCaps = PShadeCaps(0);
    pub const AlphaGouraudBlend             : PShadeCaps = PShadeCaps(D3DPSHADECAPS_ALPHAGOURAUDBLEND);
    pub const ColorGouraudRgb               : PShadeCaps = PShadeCaps(D3DPSHADECAPS_COLORGOURAUDRGB);
    pub const FogGouraud                    : PShadeCaps = PShadeCaps(D3DPSHADECAPS_FOGGOURAUD);
    pub const SpecularGouraudRgb            : PShadeCaps = PShadeCaps(D3DPSHADECAPS_SPECULARGOURAUDRGB);
}

//#cpp2rust D3DPSHADECAPS_ALPHAGOURAUDBLEND     = d3d::PShadeCaps::AlphaGouraudBlend
//#cpp2rust D3DPSHADECAPS_COLORGOURAUDRGB       = d3d::PShadeCaps::ColorGouraudRgb
//#cpp2rust D3DPSHADECAPS_FOGGOURAUD            = d3d::PShadeCaps::FogGouraud
//#cpp2rust D3DPSHADECAPS_SPECULARGOURAUDRGB    = d3d::PShadeCaps::SpecularGouraudRgb
