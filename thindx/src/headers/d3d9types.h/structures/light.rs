use crate::d3d9::*;

use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dlight9)\]
/// D3DLIGHT9
///
/// Defines a set of lighting properties.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::set_light]
/// *   [IDirect3DDevice9Ext::light_enable]
/// *   [IDirect3DDevice9Ext::get_light]
/// *   [IDirect3DDevice9Ext::get_light_enable]
/// ---
/// *   [IDirect3DDevice9Ext::set_light_32_unchecked]
/// *   [IDirect3DDevice9Ext::light_enable_32_unchecked]
/// *   [IDirect3DDevice9Ext::get_light_32]
/// *   [IDirect3DDevice9Ext::get_light_enable_32]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Light {
    /// Type of the light source.
    pub ty:             LightType,

    /// [Diffuse](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/diffuse-lighting) color emitted by the light.
    pub diffuse:        ColorValue,

    /// [Specular](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/specular-lighting) color emitted by the light.
    pub specular:       ColorValue,

    /// [Ambient](https://learn.microsoft.com/en-us/windows/uwp/graphics-concepts/ambient-lighting) color emitted by the light.
    pub ambient:        ColorValue,

    /// Position of the light in world space.  This member has no meaning for [Directional] lights, and is ignored in that case.
    ///
    /// [Directional]:                      crate::LightType::Directional
    pub position:       Vector,

    /// Direction that the light is pointing in world space.
    /// This member has meaning only for [Directional] and [Spot]lights.
    /// This vector need not be normalized, but it should have a nonzero length.
    ///
    /// [Directional]:      crate::LightType::Directional
    /// [Spot]:             crate::LightType::Spot
    pub direction:      Vector,

    /// Distance beyond which the light has no effect.
    /// The maximum allowable value for this member is the square root of [f32::MAX].
    /// This member does not affect [Directional] lights.
    ///
    /// [Directional]:                      crate::LightType::Directional
    pub range:          f32,

    /// Decrease in illumination between a spotlight's inner cone (the angle specified by [theta](#structfield.theta)) and the outer edge of the outer cone (the angle specified by [phi](#structfield.phi)).
    ///
    /// The effect of falloff on the lighting is subtle.
    /// Furthermore, a small performance penalty is incurred by shaping the falloff curve.
    /// For these reasons, most developers set this value to 1.0.
    pub falloff:        f32,

    /// Value specifying how the light intensity changes over distance.
    /// Attenuation values are ignored for [Directional] lights.
    /// This member represents an attenuation constant.
    /// For information about attenuation, see [Light Properties (Direct3D 9)].
    /// Valid values for this member range from 0.0 to infinity.
    /// For non-directional lights, all three attenuation values should not be set to 0.0 at the same time.
    ///
    /// [Directional]:                      crate::LightType::Directional
    /// [Light Properties (Direct3D 9)]:    https://learn.microsoft.com/en-us/windows/win32/direct3d9/light-properties
    pub attenuation0:   f32,

    /// Value specifying how the light intensity changes over distance.
    /// Attenuation values are ignored for [Directional] lights.
    /// This member represents an attenuation constant.
    /// For information about attenuation, see [Light Properties (Direct3D 9)].
    /// Valid values for this member range from 0.0 to infinity.
    /// For non-[Directional] lights, all three attenuation values should not be set to 0.0 at the same time.
    ///
    /// [Directional]:                      crate::LightType::Directional
    /// [Light Properties (Direct3D 9)]:    https://learn.microsoft.com/en-us/windows/win32/direct3d9/light-properties
    pub attenuation1:   f32,

    /// Value specifying how the light intensity changes over distance.
    /// Attenuation values are ignored for [Directional] lights.
    /// This member represents an attenuation constant.
    /// For information about attenuation, see [Light Properties (Direct3D 9)].
    /// Valid values for this member range from 0.0 to infinity.
    /// For non-[Directional] lights, all three attenuation values should not be set to 0.0 at the same time.
    ///
    /// [Directional]:                      crate::LightType::Directional
    /// [Light Properties (Direct3D 9)]:    https://learn.microsoft.com/en-us/windows/win32/direct3d9/light-properties
    pub attenuation2:   f32,

    /// Angle, in radians, of a spotlight's inner cone - that is, the fully illuminated spotlight cone.
    /// This value must be in the range from 0 through the value specified by [phi](#structfield.phi).
    pub theta:          f32,

    /// Angle, in radians, defining the outer edge of the spotlight's outer cone.
    /// Points outside this cone are not lit by the spotlight.
    /// This value must be between 0 and [std::f32::consts::PI].
    pub phi:            f32,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    Light => D3DLIGHT9 {
        ty              => Type,
        diffuse         => Diffuse,
        specular        => Specular,
        ambient         => Ambient,
        position        => Position,
        direction       => Direction,
        range           => Range,
        falloff         => Falloff,
        attenuation0    => Attenuation0,
        attenuation1    => Attenuation1,
        attenuation2    => Attenuation2,
        theta           => Theta,
        phi             => Phi,
    }
}

//#cpp2rust D3DLIGHT9 = d3d9::Light
