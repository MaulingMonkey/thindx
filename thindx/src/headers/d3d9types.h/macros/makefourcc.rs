/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows-hardware/drivers/ddi/d3dukmdt/nf-d3dukmdt-makefourcc)\]
/// MAKEFOURCC
///
/// Used by the D3D SDK to create various oddball identifiers.
/// Usually not used by API consumers, but it's occasionally used to create oddball identifiers for vendor specific extensions or formats.
///
/// This is really just a fancy [u32::from_le_bytes].
///
/// ### Examples
/// ```rust
/// // These are all equivalent:
/// # mod a {
/// # use thindx::*;
/// const D3DFMT_UYVY : u32 = make_four_cc(b"UYVY"); // author's preference
/// # } mod b {
/// # use thindx::*;
/// const D3DFMT_UYVY : u32 = make_four_cc(&[b'U', b'Y', b'V', b'Y']);
/// # } mod c {
/// # use thindx::*;
/// const D3DFMT_UYVY : u32 = u32::from_le_bytes(*b"UYVY");
/// # } mod d {
/// # use thindx::*;
/// const D3DFMT_UYVY : u32 = u32::from_le_bytes([b'U', b'Y', b'V', b'Y']);
/// # }
/// ```
pub const fn make_four_cc(ch: &[u8; 4]) -> u32 { u32::from_le_bytes(*ch) }

//#cpp2rust MAKEFOURCC      = make_four_cc
