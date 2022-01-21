#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcmpfunc)\]
/// D3DCMPFUNC
///
/// Defines the supported compare functions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CmpFunc(D3DCMPFUNC);
pub use CmpFunc as Cmp;

enumish! { Cmp => D3DCMPFUNC; Never, Less, Equal, LessEqual, Greater, NotEqual, GreaterEqual, Always }

#[allow(non_upper_case_globals)] impl CmpFunc { // These are enum-like
    pub const Never         : CmpFunc = CmpFunc(D3DCMP_NEVER); // 1
    pub const Less          : CmpFunc = CmpFunc(D3DCMP_LESS);
    pub const Equal         : CmpFunc = CmpFunc(D3DCMP_EQUAL);
    pub const LessEqual     : CmpFunc = CmpFunc(D3DCMP_LESSEQUAL);
    pub const Greater       : CmpFunc = CmpFunc(D3DCMP_GREATER);
    pub const NotEqual      : CmpFunc = CmpFunc(D3DCMP_NOTEQUAL);
    pub const GreaterEqual  : CmpFunc = CmpFunc(D3DCMP_GREATEREQUAL);
    pub const Always        : CmpFunc = CmpFunc(D3DCMP_ALWAYS);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for CmpFunc {
    fn default() -> Self { CmpFunc(0) }
}

//#cpp2rust D3DCMPFUNC          = d3d::CmpFunc
//#cpp2rust D3DCMP_NEVER        = d3d::Cmp::Never
//#cpp2rust D3DCMP_LESS         = d3d::Cmp::Less
//#cpp2rust D3DCMP_EQUAL        = d3d::Cmp::Equal
//#cpp2rust D3DCMP_LESSEQUAL    = d3d::Cmp::LessEqual
//#cpp2rust D3DCMP_GREATER      = d3d::Cmp::Greater
//#cpp2rust D3DCMP_NOTEQUAL     = d3d::Cmp::NotEqual
//#cpp2rust D3DCMP_GREATEREQUAL = d3d::Cmp::GreaterEqual
//#cpp2rust D3DCMP_ALWAYS       = d3d::Cmp::Always
