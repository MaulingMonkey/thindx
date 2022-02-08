use bytemuck::*;

use winapi::shared::d3d9types::*;
use winapi::shared::minwindef::WORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dgammaramp)\]
/// D3DGAMMARAMP
///
/// Contains red, green, and blue ramp data.
///
/// ### See Also
/// *   [d3d::IDirect3DDevice9Ext::get_gamma_ramp]
/// *   [d3d::IDirect3DDevice9Ext::set_gamma_ramp]
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct GammaRamp {
    pub red:    [WORD; 256],
    pub green:  [WORD; 256],
    pub blue:   [WORD; 256],
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    GammaRamp => D3DGAMMARAMP {
        red     => red,
        green   => green,
        blue    => blue,
    }
}

//#cpp2rust D3DGAMMARAMP = d3d::GammaRamp
