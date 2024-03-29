use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dvs20caps)\]
/// D3DVS20CAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Vs20Caps(DWORD);

flags! { Vs20Caps => DWORD; None, Predication }

#[allow(non_upper_case_globals)] impl Vs20Caps {
    pub const None                          : Vs20Caps = Vs20Caps(0);
    pub const Predication                   : Vs20Caps = Vs20Caps(D3DVS20CAPS_PREDICATION);
}

//#cpp2rust D3DVS20CAPS_PREDICATION = d3d::Vs20Caps::Predication
