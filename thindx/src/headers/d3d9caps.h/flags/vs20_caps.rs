use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvs20caps)\]
/// D3DVS20CAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct Vs20Caps(DWORD);

flags! { Vs20Caps => DWORD; None, Predication }

#[allow(non_upper_case_globals)] impl Vs20Caps {
    pub const None                          : Vs20Caps = Vs20Caps(0);
    pub const Predication                   : Vs20Caps = Vs20Caps(D3DVS20CAPS_PREDICATION);
}
