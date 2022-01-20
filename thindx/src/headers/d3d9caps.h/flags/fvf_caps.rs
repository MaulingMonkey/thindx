use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DFVFCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct FvfCaps(DWORD);

flags! {
    FvfCaps => DWORD;
    None, DoNotStripElements, PSize, TexCoordCountMask,
}

#[allow(non_upper_case_globals)] impl FvfCaps {
    pub const None                          : FvfCaps = FvfCaps(0);
    pub const DoNotStripElements            : FvfCaps = FvfCaps(D3DFVFCAPS_DONOTSTRIPELEMENTS);
    pub const PSize                         : FvfCaps = FvfCaps(D3DFVFCAPS_PSIZE);
    pub const TexCoordCountMask             : FvfCaps = FvfCaps(D3DFVFCAPS_TEXCOORDCOUNTMASK);
}
