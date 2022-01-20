use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddtcaps)\]
/// D3DDTCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct DtCaps(DWORD);

flags! {
    DtCaps => DWORD;
    None, UByte4, UByte4N, Short2N, Short4N, UShort2N, UShort4N, UDec3, Dec3N, Float16_2, Float16_4,
}

#[allow(non_upper_case_globals)] impl DtCaps {
    pub const None                          : DtCaps = DtCaps(0);
    pub const UByte4                        : DtCaps = DtCaps(D3DDTCAPS_UBYTE4);
    pub const UByte4N                       : DtCaps = DtCaps(D3DDTCAPS_UBYTE4N);
    pub const Short2N                       : DtCaps = DtCaps(D3DDTCAPS_SHORT2N);
    pub const Short4N                       : DtCaps = DtCaps(D3DDTCAPS_SHORT4N);
    pub const UShort2N                      : DtCaps = DtCaps(D3DDTCAPS_USHORT2N);
    pub const UShort4N                      : DtCaps = DtCaps(D3DDTCAPS_USHORT4N);
    pub const UDec3                         : DtCaps = DtCaps(D3DDTCAPS_UDEC3);
    pub const Dec3N                         : DtCaps = DtCaps(D3DDTCAPS_DEC3N);
    pub const Float16_2                     : DtCaps = DtCaps(D3DDTCAPS_FLOAT16_2);
    pub const Float16_4                     : DtCaps = DtCaps(D3DDTCAPS_FLOAT16_4);
}
