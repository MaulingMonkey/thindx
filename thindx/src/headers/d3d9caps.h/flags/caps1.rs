use crate::d3d9::*;
use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct Caps1(DWORD);

flags! { Caps1 => DWORD; None, ReadScanline, Overlay }

#[allow(non_upper_case_globals)] impl Caps {
    pub const None          : Caps1 = Caps1(0);
    pub const ReadScanline  : Caps1 = Caps1(D3DCAPS_READ_SCANLINE);
    pub const Overlay       : Caps1 = Caps1(D3DCAPS_OVERLAY);
}

#[allow(non_upper_case_globals)] impl Caps1 {
    pub const None          : Caps1 = Caps1(0);
    pub const ReadScanline  : Caps1 = Caps1(D3DCAPS_READ_SCANLINE);
    pub const Overlay       : Caps1 = Caps1(D3DCAPS_OVERLAY);
}
