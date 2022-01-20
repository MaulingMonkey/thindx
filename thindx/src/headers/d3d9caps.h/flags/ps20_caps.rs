use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



// TODO: fix upstream docs (broken as heck double "d3d" prefix, misnamed constants, etc.)
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dps20caps)\]
/// D3DPS20CAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct Ps20Caps(DWORD);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct Reserved5(DWORD);

flags! { Ps20Caps => DWORD; None, ArbitrarySwizzle, GradientInstructions, Predication, NoDependentReadLimit, NoTexInstructionLimit }

#[allow(non_upper_case_globals)] impl Ps20Caps {
    pub const None                          : Ps20Caps = Ps20Caps(0);
    pub const ArbitrarySwizzle              : Ps20Caps = Ps20Caps(D3DPS20CAPS_ARBITRARYSWIZZLE);
    pub const GradientInstructions          : Ps20Caps = Ps20Caps(D3DPS20CAPS_GRADIENTINSTRUCTIONS);
    pub const Predication                   : Ps20Caps = Ps20Caps(D3DPS20CAPS_PREDICATION);
    pub const NoDependentReadLimit          : Ps20Caps = Ps20Caps(D3DPS20CAPS_NODEPENDENTREADLIMIT);
    pub const NoTexInstructionLimit         : Ps20Caps = Ps20Caps(D3DPS20CAPS_NOTEXINSTRUCTIONLIMIT);
}
