#![warn(clippy::undocumented_unsafe_blocks)]

use bytemuck::{Pod, Zeroable};

use winapi::shared::guiddef::GUID;

use std::cmp::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid)\]
/// A 128-bit identifier used for COM interfaces, COM class objects, and various other purpouses.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Guid(GUID);

/// = [Guid]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/wmformat/interface-identifiers)\]
/// **I**nterface **Id**entifier - used for uniquely identifiying COM interfaces
pub type IID    = Guid;

/// = [Guid]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/com/com-class-objects-and-clsids)\]
/// **Cl**as**s** **Id**entifier - used for uniquely identifying COM classes
pub type ClsID  = Guid;

/// = [Guid]
/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/stg/format-identifiers)\]
/// **F**or**m**a**t** **Id**entifier - used for tagging sections in e.g. structured storage property sets
pub type FmtID  = Guid;

/// Define a [Guid] constant.
///
/// ### Examples
/// ```rust
/// # use thindx::*;
/// const IID_NULL : Guid = guid!("00000000-0000-0000-0000-000000000000");
/// ```
#[macro_export]
macro_rules! guid {
    ( $s:literal ) => {{
        // ensure macro contents are evaluated at compile time
        const GUID : $crate::Guid = {
            use $crate::*;
            const STR : &'static str = $s;
            let mut bytes = STR.as_bytes();

            guid!(@expect-hex  bytes, 8, "1st");
            guid!(@expect-dash bytes, 8, "1st");
            guid!(@expect-hex  bytes, 4, "2nd");
            guid!(@expect-dash bytes, 4, "2nd");
            guid!(@expect-hex  bytes, 4, "3rd");
            guid!(@expect-dash bytes, 4, "3rd");
            guid!(@expect-hex  bytes, 4, "4th");
            guid!(@expect-dash bytes, 4, "4th");
            guid!(@expect-hex  bytes,12, "5th");
            guid!(@expect-eof  bytes,12, "5th");

            $crate::Guid::_zzz_from_literal(STR)
        };
        GUID
    }};
    ( @expect-dash $bytes:ident, $expected_digits:literal, $group:literal ) => {
        if let Some((ch, rest)) = $bytes.split_first() {
            if ch.is_ascii_hexdigit() {
                panic!("{}", concat!("expected `-` after ", $expected_digits, " hex digits in ", $group, " group, but found more than that"));
            } else if *ch == b'-' {
                $bytes = rest; // OK
            } else {
                panic!("{}", concat!("expected `-` after ", $group, " group of hex digits, but found something else"));
            }
        } else {
            panic!("{}", concat!("expected `-` after ", $group, " group of hex digits, but reached the end of the string instead"));
        }
    };
    ( @expect-eof $bytes:ident, $expected_digits:literal, $group:literal ) => {
        if let Some((ch, _rest)) = $bytes.split_first() {
            if ch.is_ascii_hexdigit() {
                panic!("{}", concat!("expected end of string after ", $expected_digits, " hex digits in ", $group, " group, but found more than that"));
            } else if *ch == b'-' {
                panic!("{}", concat!("expected end of string after ", $expected_digits, " hex digits in ", $group, " group, but found a trailing dash instead"));
            } else {
                panic!("{}", concat!("expected end of string after ", $group, " group of hex digits, but found something else"));
            }
        } else {
            // OK
        }
    };
    ( @expect-hex $bytes:ident, $expected_digits:literal, $group:literal ) => {
        let mut digits : usize = $expected_digits;
        while digits > 0 {
            digits -= 1;
            if let Some((ch, rest)) = $bytes.split_first() {
                if ch.is_ascii_hexdigit() {
                    $bytes = rest; // OK
                } else if *ch == b'-' {
                    panic!("{}", concat!("expected ", $expected_digits, " hex digits in ", $group, " group, but found a dash before that"));
                } else {
                    panic!("{}", concat!("expected ", $expected_digits, " hex digits in ", $group, " group, but found something else"));
                }
            } else {
                panic!("{}", concat!("expected ", $expected_digits, " hex digits in ", $group, " group, but reached the end of the string instead"));
            }
        }
    };
}

impl Guid {
    /// `{00000000-0000-0000-0000-000000000000}` - the "null" guid
    pub const NULL : Self = guid!("00000000-0000-0000-0000-000000000000");
}

unsafe impl Pod         for Guid {}
unsafe impl Zeroable    for Guid {}
impl Default            for Guid { fn default() -> Self { Self::zeroed() } }

impl Debug              for Guid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.fmt_impl(fmt) } }
impl Display            for Guid { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { self.fmt_impl(fmt) } }

impl AsRef<Guid> for Guid { fn as_ref(&    self) -> &    Guid {      self } }
impl AsMut<Guid> for Guid { fn as_mut(&mut self) -> &mut Guid {      self } }
impl AsRef<GUID> for Guid { fn as_ref(&    self) -> &    GUID { &    self.0 } }
impl AsMut<GUID> for Guid { fn as_mut(&mut self) -> &mut GUID { &mut self.0 } }
impl AsRef<Guid> for GUID { fn as_ref(&    self) -> &    Guid { unsafe { std::mem::transmute(self) } } }
impl AsMut<Guid> for GUID { fn as_mut(&mut self) -> &mut Guid { unsafe { std::mem::transmute(self) } } }
impl From<GUID> for Guid { fn from(guid: GUID) -> Self { Self(guid) } }
impl From<Guid> for GUID { fn from(guid: Guid) -> Self { guid.0 } }

impl Eq                 for Guid {}
impl PartialEq          for Guid { fn eq(&self, other: &Self) -> bool { self.as_bytes() == other.as_bytes() } }
impl Ord                for Guid { fn cmp(&self, other: &Self) -> Ordering { self.as_bytes().cmp(other.as_bytes()) } }
impl PartialOrd         for Guid { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.as_bytes().partial_cmp(other.as_bytes()) } }
impl Hash               for Guid { fn hash<H: Hasher>(&self, state: &mut H) { self.as_bytes().hash(state) } }

impl Guid {
    #[doc(hidden)] pub const fn _zzz_from_literal(s: &'static str) -> Self {
        #![allow(non_snake_case)]
        let s = s.as_bytes();

        const fn hex2b(hi: u8, lo: u8) -> u8 {
            let hi = match hi {
                b'0' ..= b'9'   => hi-b'0',
                b'a' ..= b'f'   => hi-b'a'+10,
                b'A' ..= b'F'   => hi-b'A'+10,
                _               => 0, // already paniced at macro site
            };
            let lo = match lo {
                b'0' ..= b'9'   => lo-b'0',
                b'a' ..= b'f'   => lo-b'a'+10,
                b'A' ..= b'F'   => lo-b'A'+10,
                _               => 0, // already paniced at macro site
            };
            (hi << 4) | lo
        }

        let Data1 = u32::from_be_bytes([
            hex2b(s[ 0], s[ 1]),
            hex2b(s[ 2], s[ 3]),
            hex2b(s[ 4], s[ 5]),
            hex2b(s[ 6], s[ 7]),
        ]);

        let Data2 = u16::from_be_bytes([
            hex2b(s[ 9], s[10]),
            hex2b(s[11], s[12]),
        ]);

        let Data3 = u16::from_be_bytes([
            hex2b(s[14], s[15]),
            hex2b(s[16], s[17]),
        ]);

        let Data4 = [
            hex2b(s[19], s[20]),
            hex2b(s[21], s[22]),
            // -
            hex2b(s[24], s[25]),
            hex2b(s[26], s[27]),
            hex2b(s[28], s[29]),
            hex2b(s[30], s[31]),
            hex2b(s[32], s[33]),
            hex2b(s[34], s[35]),
        ];

        Self(GUID { Data1, Data2, Data3, Data4 })
    }

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
    assert_eq!("{6B29FC40-CA47-1067-B31D-00DD010662DA}", guid!("6B29FC40-CA47-1067-B31D-00DD010662DA").to_string());
    assert_eq!("{6B29FC40-CA47-1067-B31D-00DD010662DA}", guid!("6b29fc40-ca47-1067-b31d-00dd010662da").to_string());
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

//#cpp2rust DEFINE_GUID         = guid!
//#cpp2rust DEFINE_OLEGUID      = guid!

//#cpp2rust DECLSPEC_UUID       = impl winapi::Interface

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
