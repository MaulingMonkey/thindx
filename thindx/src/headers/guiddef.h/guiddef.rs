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

/// = [Guid]
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/wmformat/interface-identifiers)\]
/// **I**interface **Id**entifier - used for uniquely identifiying COM interfaces
pub type IID    = Guid;

/// = [Guid]
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/com/com-class-objects-and-clsids)\]
/// **Cl**as**s** **Id**entifier - used for uniquely identifying COM classes
pub type ClsID  = Guid;

/// = [Guid]
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/stg/format-identifiers)\]
/// **F**or**m**a**t** **Id**entifier - used for tagging sections in e.g. structured storage property sets
pub type FmtID  = Guid;

/// Define a [Guid] constant.
///
/// ### Examples
/// ```rust
/// # use thindx::*;
/// const IID_NULL : Guid = guid!{00000000-0000-0000-0000-000000000000};
/// ```
#[macro_export]
macro_rules! guid {
    ( $a:tt - $b:tt - $c:tt - $d:tt - $e:tt ) => {{
        // ensure macro contents are evaluated at compile time
        const GUID : $crate::Guid = {
            const A : &'static [u8] = stringify!($a).as_bytes();
            const B : &'static [u8] = stringify!($b).as_bytes();
            const C : &'static [u8] = stringify!($c).as_bytes();
            const D : &'static [u8] = stringify!($d).as_bytes();
            const E : &'static [u8] = stringify!($e).as_bytes();
            assert!(A.len() ==  8,  "expected 8 hex characters for 1st group of guid digits");
            assert!(B.len() ==  4,  "expected 4 hex characters for 2nd group of guid digits");
            assert!(C.len() ==  4,  "expected 4 hex characters for 3rd group of guid digits");
            assert!(D.len() ==  4,  "expected 4 hex characters for 4th group of guid digits");
            assert!(E.len() == 12, "expected 12 hex characters for 5th group of guid digits");
            assert!($crate::Guid::_zzz_all_hex_digits_8(A),  "non-hexidecimal characters in 1st group of guid digits");
            assert!($crate::Guid::_zzz_all_hex_digits_4(B),  "non-hexidecimal characters in 2nd group of guid digits");
            assert!($crate::Guid::_zzz_all_hex_digits_4(C),  "non-hexidecimal characters in 3rd group of guid digits");
            assert!($crate::Guid::_zzz_all_hex_digits_4(D),  "non-hexidecimal characters in 4th group of guid digits");
            assert!($crate::Guid::_zzz_all_hex_digits_12(E), "non-hexidecimal characters in 5th group of guid digits");
            $crate::Guid::_zzz_from_macro_contents(A, B, C, D, E)
        };
        GUID
    }};
}

impl Guid {
    /// `{00000000-0000-0000-0000-000000000000}` - the "null" guid
    pub const NULL : Self = guid!{00000000-0000-0000-0000-000000000000};
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
    #[doc(hidden)] pub const fn _zzz_all_hex_digits_4 (s: &[u8]) -> bool { s[0].is_ascii_hexdigit() && s[1].is_ascii_hexdigit() && s[2].is_ascii_hexdigit() && s[3].is_ascii_hexdigit() }
    #[doc(hidden)] pub const fn _zzz_all_hex_digits_8 (s: &[u8]) -> bool { Self::_zzz_all_hex_digits_4(&[s[0], s[1], s[2], s[3]]) && Self::_zzz_all_hex_digits_4(&[s[4], s[5], s[6], s[7]]) }
    #[doc(hidden)] pub const fn _zzz_all_hex_digits_12(s: &[u8]) -> bool { Self::_zzz_all_hex_digits_4(&[s[0], s[1], s[2], s[3]]) && Self::_zzz_all_hex_digits_4(&[s[4], s[5], s[6], s[7]]) && Self::_zzz_all_hex_digits_4(&[s[8], s[9], s[10], s[11]]) }

    #[doc(hidden)] pub const fn _zzz_from_macro_contents(
        a: &[u8],
        b: &[u8],
        c: &[u8],
        d: &[u8],
        e: &[u8],
    ) -> Self {
        #![allow(non_snake_case)]

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
            hex2b(a[0], a[1]),
            hex2b(a[2], a[3]),
            hex2b(a[4], a[5]),
            hex2b(a[6], a[7]),
        ]);

        let Data2 = u16::from_be_bytes([
            hex2b(b[0], b[1]),
            hex2b(b[2], b[3]),
        ]);

        let Data3 = u16::from_be_bytes([
            hex2b(c[0], c[1]),
            hex2b(c[2], c[3]),
        ]);

        let Data4 = [
            hex2b(d[ 0], d[ 1]),
            hex2b(d[ 2], d[ 3]),
            // -
            hex2b(e[ 0], e[ 1]),
            hex2b(e[ 2], e[ 3]),
            hex2b(e[ 4], e[ 5]),
            hex2b(e[ 6], e[ 7]),
            hex2b(e[ 8], e[ 9]),
            hex2b(e[10], e[11]),
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
    assert_eq!("{6B29FC40-CA47-1067-B31D-00DD010662DA}", guid!{6B29FC40-CA47-1067-B31D-00DD010662DA}.to_string());
    assert_eq!("{6B29FC40-CA47-1067-B31D-00DD010662DA}", guid!{6b29fc40-ca47-1067-b31d-00dd010662da}.to_string());
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
