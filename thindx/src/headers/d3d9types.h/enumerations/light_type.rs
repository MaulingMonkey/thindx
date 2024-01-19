use crate::d3d9::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dlighttype)\]
/// D3DLIGHTTYPE
///
/// Defines the [light type](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types).
///
/// Directional lights are slightly faster than point light sources, but point lights look a little better.
/// Spotlights offer interesting visual effects but are computationally time-consuming.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct LightType(D3DLIGHTTYPE);

enumish! { LightType => D3DLIGHTTYPE; Point, Spot, Directional }

#[allow(non_upper_case_globals)] impl LightType { // These are enum-like
    /// Light is a [point] source. The light has a position in space and radiates light in all directions.
    ///
    /// [point]:        https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#point-light
    pub const Point         : LightType = LightType(D3DLIGHT_POINT); // 1

    /// Light is a [spotlight] source.
    /// This light is like a point light, except that the illumination is limited to a cone.
    /// This light type has a direction and several other parameters that determine the shape of the cone it produces.
    /// For information about these parameters, see the [Light] structure.
    ///
    /// [spotlight]:    https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#spotlight
    pub const Spot          : LightType = LightType(D3DLIGHT_SPOT);

    /// Light is a [directional] light source. This is equivalent to using a point light source at an infinite distance.
    ///
    /// [directional]:  https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#directional-light
    pub const Directional   : LightType = LightType(D3DLIGHT_DIRECTIONAL);
}

#[allow(non_upper_case_globals)] impl Light { // These are enum-like
    /// Light is a [point] source. The light has a position in space and radiates light in all directions.
    ///
    /// [point]:        https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#point-light
    pub const Point         : LightType = LightType(D3DLIGHT_POINT); // 1

    /// Light is a [spotlight] source.
    /// This light is like a point light, except that the illumination is limited to a cone.
    /// This light type has a direction and several other parameters that determine the shape of the cone it produces.
    /// For information about these parameters, see the [Light] structure.
    ///
    /// [spotlight]:    https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#spotlight
    pub const Spot          : LightType = LightType(D3DLIGHT_SPOT);

    /// Light is a [directional] light source. This is equivalent to using a point light source at an infinite distance.
    ///
    /// [directional]:  https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/light-types#directional-light
    pub const Directional   : LightType = LightType(D3DLIGHT_DIRECTIONAL);
}

impl Default for LightType {
    fn default() -> Self { LightType(0) }
}

//#cpp2rust D3DLIGHTTYPE            = d3d::LightType
//#cpp2rust D3DLIGHT_POINT          = d3d::Light::Point
//#cpp2rust D3DLIGHT_SPOT           = d3d::Light::Spot
//#cpp2rust D3DLIGHT_DIRECTIONAL    = d3d::Light::Directional