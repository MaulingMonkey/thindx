#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlighttype)\]
/// D3DLIGHTTYPE
///
/// Defines the [light type](https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types).
///
/// Directional lights are slightly faster than point light sources, but point lights look a little better.
/// Spotlights offer interesting visual effects but are computationally time-consuming.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct LightType(D3DLIGHTTYPE);

impl LightType {
    /// Convert a raw [D3DLIGHTTYPE] value into a [LightType].  This is *probably* safe... probably...
    ///
    /// [D3DLIGHTTYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlighttype
    pub const fn from_unchecked(lighttype: D3DLIGHTTYPE) -> Self { Self(lighttype) }

    /// Convert a [LightType] into a raw [D3DLIGHTTYPE].
    ///
    /// [D3DLIGHTTYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlighttype
    pub const fn into(self) -> D3DLIGHTTYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl LightType {
    /// Light is a [point] source. The light has a position in space and radiates light in all directions.
    ///
    /// [point]:        https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#point-light
    pub const Point         : LightType = LightType(D3DLIGHT_POINT);

    /// Light is a [spotlight] source.
    /// This light is like a point light, except that the illumination is limited to a cone.
    /// This light type has a direction and several other parameters that determine the shape of the cone it produces.
    /// For information about these parameters, see the [Light] structure.
    ///
    /// [spotlight]:    https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#spotlight
    pub const Spot          : LightType = LightType(D3DLIGHT_SPOT);

    /// Light is a [directional] light source. This is equivalent to using a point light source at an infinite distance.
    ///
    /// [directional]:  https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#directional-light
    pub const Directional   : LightType = LightType(D3DLIGHT_DIRECTIONAL);
}

impl Debug for LightType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            LightType::Point        => write!(f, "LightType::Point"),
            LightType::Spot         => write!(f, "LightType::Spot"),
            LightType::Directional  => write!(f, "LightType::Directional"),
            other                   => write!(f, "LightType({})", other.0 as u32),
        }
    }
}

impl Default for LightType {
    fn default() -> Self { LightType::from_unchecked(0) }
}

impl From<LightType> for D3DLIGHTTYPE {
    fn from(value: LightType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DLIGHTTYPE> for LightType {
    fn from(value: D3DLIGHTTYPE) -> Self { Self(value) }
}
