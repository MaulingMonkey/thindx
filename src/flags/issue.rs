#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;
type D3DISSUE = u32; // there's no actual type

use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlock)\]
/// DWORD / D3DISSUE_*
///
/// Controls how [Query::issue] behaves.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Issue(D3DISSUE);

impl Issue {
    pub const fn from_unchecked(lock: D3DISSUE) -> Self { Self(lock) }
    pub const fn into(self) -> D3DISSUE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Issue {
    /// No flags
    pub const None          : Issue = Issue(0);

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dissue-begin)\]
    /// D3DISSUE_BEGIN
    ///
    /// [Issue::Begin] is valid for [QueryType::Occlusion].
    pub const Begin         : Issue = Issue(D3DISSUE_BEGIN);

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dissue-end)\]
    /// D3DISSUE_END
    ///
    /// [Issue::End] is valid for [QueryType::VCache], [QueryType::ResourceManager], [QueryType::VertexStats], [QueryType::Event], and [QueryType::Occlusion]
    pub const End           : Issue = Issue(D3DISSUE_END);
}

impl BitOrAssign for Issue {
    fn bitor_assign(&mut self, other: Self) { self.0 |= other.0 }
}

impl BitOr for Issue {
    type Output = Self;
    fn bitor(self, other: Self) -> Self { Self(self.0 | other.0) }
}

impl Debug for Issue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Issue::None     => write!(f, "Issue::None"),
            Issue::Begin    => write!(f, "Issue::Begin"),
            Issue::End      => write!(f, "Issue::End"),
            other           => write!(f, "Issue({})", other.0),
        }
    }
}

impl From<Issue> for D3DISSUE {
    fn from(value: Issue) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DISSUE> for Issue {
    fn from(value: D3DISSUE) -> Self { Self(value) }
}
