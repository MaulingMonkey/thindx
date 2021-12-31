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
