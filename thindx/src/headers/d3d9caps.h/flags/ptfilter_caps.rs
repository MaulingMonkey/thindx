#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dptfiltercaps)\]
/// D3DPTFILTERCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct PTFilterCaps(DWORD);

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
