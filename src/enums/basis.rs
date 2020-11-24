#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbasistype)\]
/// D3DBASISTYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Basis(D3DBASISTYPE);

impl Basis {
    /// Convert a raw [D3DBASISTYPE] value into a [Basis].  This is *probably* safe... probably....
    ///
    /// [D3DBASISTYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbasistype
    pub const fn from_unchecked(basistype: D3DBASISTYPE) -> Self { Self(basistype) }

    /// Convert a [Basis] into a raw [D3DBASISTYPE].
    ///
    /// [D3DBASISTYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbasistype
    pub const fn into(self) -> D3DBASISTYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Basis {
    pub const Bezier        : Basis = Basis(D3DBASIS_BEZIER);
    pub const BSpline       : Basis = Basis(D3DBASIS_BSPLINE);
    pub const CatlmullRom   : Basis = Basis(D3DBASIS_CATMULL_ROM);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for Basis {
    fn default() -> Self { Basis::Bezier }
}

impl Debug for Basis {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Basis::Bezier       => write!(f, "Basis::Bezier"),
            Basis::BSpline      => write!(f, "Basis::BSpline"),
            Basis::CatlmullRom  => write!(f, "Basis::CatlmullRom"),
            other               => write!(f, "Basis({})", other.0),
        }
    }
}

impl From<Basis> for D3DBASISTYPE {
    fn from(value: Basis) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DBASISTYPE> for Basis {
    fn from(value: D3DBASISTYPE) -> Self { Self(value) }
}
