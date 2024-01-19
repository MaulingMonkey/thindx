use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dcursorcaps)\]
/// D3DCURSORCAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct CursorCaps(DWORD);

flags! { CursorCaps => DWORD; None, Color, LowRes }

#[allow(non_upper_case_globals)] impl CursorCaps {
    pub const None                          : CursorCaps = CursorCaps(0);
    pub const Color                         : CursorCaps = CursorCaps(D3DCURSORCAPS_COLOR);
    pub const LowRes                        : CursorCaps = CursorCaps(D3DCURSORCAPS_LOWRES);
}

//#cpp2rust D3DCURSORCAPS_COLOR     = d3d::CursorCaps::Color
//#cpp2rust D3DCURSORCAPS_LOWRES    = d3d::CursorCaps::LowRes
