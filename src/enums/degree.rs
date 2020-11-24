#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddegreetype)\]
/// D3DDEGREETYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Degree(D3DDEGREETYPE);

impl Degree {
    /// Convert a raw [D3DDEGREETYPE] value into a [Degree].  This is *probably* safe... probably....
    ///
    /// [D3DDEGREETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddegreetype
    pub const fn from_unchecked(degreetype: D3DDEGREETYPE) -> Self { Self(degreetype) }

    /// Convert a [Degree] into a raw [D3DDEGREETYPE].
    ///
    /// [D3DDEGREETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddegreetype
    pub const fn into(self) -> D3DDEGREETYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Degree {
    pub const Linear        : Degree = Degree(D3DDEGREE_LINEAR);
    pub const Quadratic     : Degree = Degree(D3DDEGREE_QUADRATIC);
    pub const Cubic         : Degree = Degree(D3DDEGREE_CUBIC);
    pub const Quintic       : Degree = Degree(D3DDEGREE_QUINTIC);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for Degree {
    fn default() -> Self { Degree(0) }
}

impl Debug for Degree {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Degree::Linear      => write!(f, "Degree::Linear"),
            Degree::Quadratic   => write!(f, "Degree::Quadratic"),
            Degree::Cubic       => write!(f, "Degree::Cubic"),
            Degree::Quintic     => write!(f, "Degree::Quintic"),
            other               => write!(f, "Degree({})", other.0),
        }
    }
}

impl From<Degree> for D3DDEGREETYPE {
    fn from(value: Degree) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DDEGREETYPE> for Degree {
    fn from(value: D3DDEGREETYPE) -> Self { Self(value) }
}
