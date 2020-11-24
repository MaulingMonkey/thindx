#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcmpfunc)\]
/// D3DCMPFUNC
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CmpFunc(D3DCMPFUNC);

impl CmpFunc {
    /// Convert a raw [D3DCMPFUNC] value into a [CmpFunc].  This is *probably* safe... probably....
    ///
    /// [D3DCMPFUNC]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcmpfunc
    pub const fn from_unchecked(cmpfunc: D3DCMPFUNC) -> Self { Self(cmpfunc) }

    /// Convert a [CmpFunc] into a raw [D3DCMPFUNC].
    ///
    /// [D3DCMPFUNC]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcmpfunc
    pub const fn into(self) -> D3DCMPFUNC { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl CmpFunc {
    pub const Never         : CmpFunc = CmpFunc(D3DCMP_NEVER);
    pub const Less          : CmpFunc = CmpFunc(D3DCMP_LESS);
    pub const Equal         : CmpFunc = CmpFunc(D3DCMP_EQUAL);
    pub const LessEqual     : CmpFunc = CmpFunc(D3DCMP_LESSEQUAL);
    pub const Greater       : CmpFunc = CmpFunc(D3DCMP_GREATER);
    pub const NotEqual      : CmpFunc = CmpFunc(D3DCMP_NOTEQUAL);
    pub const GreaterEqual  : CmpFunc = CmpFunc(D3DCMP_GREATEREQUAL);
    pub const Always        : CmpFunc = CmpFunc(D3DCMP_ALWAYS);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for CmpFunc {
    fn default() -> Self { CmpFunc(0) }
}

impl Debug for CmpFunc {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            CmpFunc::Never          => write!(f, "CmpFunc::Never"),
            CmpFunc::Less           => write!(f, "CmpFunc::Less"),
            CmpFunc::Equal          => write!(f, "CmpFunc::Equal"),
            CmpFunc::LessEqual      => write!(f, "CmpFunc::LessEqual"),
            CmpFunc::Greater        => write!(f, "CmpFunc::Greater"),
            CmpFunc::NotEqual       => write!(f, "CmpFunc::NotEqual"),
            CmpFunc::GreaterEqual   => write!(f, "CmpFunc::GreaterEqual"),
            CmpFunc::Always         => write!(f, "CmpFunc::Always"),
            other                   => write!(f, "CmpFunc({})", other.0),
        }
    }
}

impl From<CmpFunc> for D3DCMPFUNC {
    fn from(value: CmpFunc) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DCMPFUNC> for CmpFunc {
    fn from(value: D3DCMPFUNC) -> Self { Self(value) }
}
