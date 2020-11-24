#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterialcolorsource)\]
/// D3DMATERIALCOLORSOURCE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MaterialColorSource(D3DMATERIALCOLORSOURCE);
pub type MCS = MaterialColorSource;

impl MaterialColorSource {
    /// Convert a raw [D3DMATERIALCOLORSOURCE] value into a [MaterialColorSource].  This is *probably* safe... probably....
    ///
    /// [D3DMATERIALCOLORSOURCE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterialcolorsource
    pub const fn from_unchecked(materialcolorsource: D3DMATERIALCOLORSOURCE) -> Self { Self(materialcolorsource) }

    /// Convert a [MaterialColorSource] into a raw [D3DMATERIALCOLORSOURCE].
    ///
    /// [D3DMATERIALCOLORSOURCE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmaterialcolorsource
    pub const fn into(self) -> D3DMATERIALCOLORSOURCE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl MaterialColorSource {
    pub const Material  : MaterialColorSource = MaterialColorSource(D3DMCS_MATERIAL);
    pub const Color1    : MaterialColorSource = MaterialColorSource(D3DMCS_COLOR1);
    pub const Color2    : MaterialColorSource = MaterialColorSource(D3DMCS_COLOR2);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for MaterialColorSource {
    fn default() -> Self { MaterialColorSource::Default }
}

impl Debug for MaterialColorSource {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            MaterialColorSource::Material   => write!(f, "MaterialColorSource::Material"),
            MaterialColorSource::Color1     => write!(f, "MaterialColorSource::Color1"),
            MaterialColorSource::Color2     => write!(f, "MaterialColorSource::Color2"),
            other                           => write!(f, "MaterialColorSource({})", other.0),
        }
    }
}

impl From<MaterialColorSource> for D3DMATERIALCOLORSOURCE {
    fn from(value: MaterialColorSource) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DMATERIALCOLORSOURCE> for MaterialColorSource {
    fn from(value: D3DMATERIALCOLORSOURCE) -> Self { Self(value) }
}
