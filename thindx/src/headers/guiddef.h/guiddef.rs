#![warn(clippy::undocumented_unsafe_blocks)]

use bytemuck::{Pod, Zeroable};

use winapi::shared::guiddef::GUID;

use std::cmp::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)\]
/// A 128-bit identifier used for COM interfaces, COM class objects, and various other purpouses.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Guid(GUID);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/wmformat/interface-identifiers)\]
/// **I**interface **Id**entifier - used for uniquely identifiying COM interfaces
pub type IID    = Guid;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/com/com-class-objects-and-clsids)\]
/// **Cl**as**s** **Id**entifier - used for uniquely identifying COM classes
pub type ClsID  = Guid;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/stg/format-identifiers)\]
/// **F**or**m**a**t** **Id**entifier - used for tagging sections in e.g. structured storage property sets
pub type FmtID  = Guid;

impl Guid {
    /// `{00000000-0000-0000-0000-000000000000}` - the "null" guid
    pub const NULL : Self = Self(GUID { Data1: 0, Data2: 0, Data3: 0, Data4: [0, 0, 0, 0, 0, 0, 0, 0] });
}

unsafe impl Pod         for Guid {}
unsafe impl Zeroable    for Guid {}
impl Default            for Guid { fn default() -> Self { Self::zeroed() } }

impl Debug              for Guid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.fmt_impl(fmt) } }
impl Display            for Guid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.fmt_impl(fmt) } }

impl From<GUID> for Guid { fn from(guid: GUID) -> Self { Self(guid) } }
impl From<Guid> for GUID { fn from(guid: Guid) -> Self { guid.0 } }

impl Eq                 for Guid {}
impl PartialEq          for Guid { fn eq(&self, other: &Self) -> bool { self.as_bytes() == other.as_bytes() } }
impl Ord                for Guid { fn cmp(&self, other: &Self) -> Ordering { self.as_bytes().cmp(other.as_bytes()) } }
impl PartialOrd         for Guid { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.as_bytes().partial_cmp(other.as_bytes()) } }
impl Hash               for Guid { fn hash<H: Hasher>(&self, state: &mut H) { self.as_bytes().hash(state) } }



impl Guid {
    fn fmt_impl(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
            self.0.Data1,
            self.0.Data2,
            self.0.Data3,
            self.0.Data4[0], self.0.Data4[1],
            self.0.Data4[2], self.0.Data4[3], self.0.Data4[4], self.0.Data4[5], self.0.Data4[6], self.0.Data4[7],
        )
    }

    fn as_bytes(&self) -> &[u8] { bytemuck::bytes_of(self) }
}

#[test] fn test_display() {
    assert_eq!("{6B29FC40-CA47-1067-B31D-00DD010662DA}", Guid(GUID { Data1: 0x6B29FC40, Data2: 0xCA47, Data3: 0x1067, Data4: *b"\xB3\x1D\x00\xDD\x01\x06\x62\xDA" }).to_string());
}

//#cpp2rust GUID                = Guid
//#cpp2rust IID                 = IID
//#cpp2rust CLSID               = ClsID
//#cpp2rust FMTID               = FmtID

//#cpp2rust REFGUID             = &Guid
//#cpp2rust REFIID              = &IID
//#cpp2rust REFCLSID            = &ClsID
//#cpp2rust REFFMTID            = &FmtID

//#cpp2rust GUID_NULL           = Guid::NULL
//#cpp2rust IID_NULL            = IID::NULL
//#cpp2rust CLSID_NULL          = ClsID::NULL
//#cpp2rust FMTID_NULL          = FmtID::NULL

//#cpp2rust InlineIsEqualGUID   = Guid::eq
//#cpp2rust IsEqualCLSID        = ClsID::eq
//#cpp2rust IsEqualFMTID        = FmtID::eq
//#cpp2rust IsEqualGUID         = Guid::eq
//#cpp2rust IsEqualIID          = IID::eq

// ABI/linker cruft
//#cpp2ignore EXTERN_C
//#cpp2ignore DECLSPEC_SELECTANY

// 16-bit Windows cruft
//#cpp2ignore FAR
