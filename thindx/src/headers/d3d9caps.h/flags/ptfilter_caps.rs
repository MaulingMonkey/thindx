#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dptfiltercaps)\]
/// D3DPTFILTERCAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct PTFilterCaps(DWORD);

flags! {
    PTFilterCaps => DWORD;
    None,
    ConvolutionMono,
    MagFPoint, MagFLinear, MagFAnisotropic, MagFPyramidalQuad, MagFGaussianQuad,
    MinFPoint, MinFLinear, MinFAnisotropic, MinFPyramidalQuad, MinFGaussianQuad,
    MipFPoint, MipFLinear,
}

#[allow(non_upper_case_globals)] impl PTFilterCaps {
    pub const None                          : PTFilterCaps = PTFilterCaps(0);
    pub const ConvolutionMono               : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_CONVOLUTIONMONO);
    pub const MagFPoint                     : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MAGFPOINT);
    pub const MagFLinear                    : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MAGFLINEAR);
    pub const MagFAnisotropic               : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MAGFANISOTROPIC);
    pub const MagFPyramidalQuad             : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD);
    pub const MagFGaussianQuad              : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MAGFGAUSSIANQUAD);
    pub const MinFPoint                     : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MINFPOINT);
    pub const MinFLinear                    : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MINFLINEAR);
    pub const MinFAnisotropic               : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MINFANISOTROPIC);
    pub const MinFPyramidalQuad             : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MINFPYRAMIDALQUAD);
    pub const MinFGaussianQuad              : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MINFGAUSSIANQUAD);
    pub const MipFPoint                     : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MIPFPOINT);
    pub const MipFLinear                    : PTFilterCaps = PTFilterCaps(D3DPTFILTERCAPS_MIPFLINEAR);
}

//#cpp2rust D3DPTFILTERCAPS_CONVOLUTIONMONO     = d3d::PTFilterCaps::ConvolutionMono
//#cpp2rust D3DPTFILTERCAPS_MAGFPOINT           = d3d::PTFilterCaps::MagFPoint
//#cpp2rust D3DPTFILTERCAPS_MAGFLINEAR          = d3d::PTFilterCaps::MagFLinear
//#cpp2rust D3DPTFILTERCAPS_MAGFANISOTROPIC     = d3d::PTFilterCaps::MagFAnisotropic
//#cpp2rust D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD   = d3d::PTFilterCaps::MagFPyramidalQuad
//#cpp2rust D3DPTFILTERCAPS_MAGFGAUSSIANQUAD    = d3d::PTFilterCaps::MagFGaussianQuad
//#cpp2rust D3DPTFILTERCAPS_MINFPOINT           = d3d::PTFilterCaps::MinFPoint
//#cpp2rust D3DPTFILTERCAPS_MINFLINEAR          = d3d::PTFilterCaps::MinFLinear
//#cpp2rust D3DPTFILTERCAPS_MINFANISOTROPIC     = d3d::PTFilterCaps::MinFAnisotropic
//#cpp2rust D3DPTFILTERCAPS_MINFPYRAMIDALQUAD   = d3d::PTFilterCaps::MinFPyramidalQuad
//#cpp2rust D3DPTFILTERCAPS_MINFGAUSSIANQUAD    = d3d::PTFilterCaps::MinFGaussianQuad
//#cpp2rust D3DPTFILTERCAPS_MIPFPOINT           = d3d::PTFilterCaps::MipFPoint
//#cpp2rust D3DPTFILTERCAPS_MIPFLINEAR          = d3d::PTFilterCaps::MipFLinear
