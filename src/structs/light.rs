use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlight9)\]
/// D3DLIGHT9
///
/// Defines a set of lighting properties.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Light {
    /// Type of the light source.
    pub light_type:     LightType,

    /// [Diffuse](https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/diffuse-lighting) color emitted by the light.
    pub diffuse:        ColorValue,

    /// [Specular](https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/specular-lighting) color emitted by the light.
    pub specular:       ColorValue,

    /// [Ambient](https://docs.microsoft.com/en-us/windows/uwp/graphics-concepts/ambient-lighting) color emitted by the light.
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
    /// [Light Properties (Direct3D 9)]:    https://docs.microsoft.com/en-us/windows/win32/direct3d9/light-properties
    pub attenuation0:   f32,

    /// Value specifying how the light intensity changes over distance.
    /// Attenuation values are ignored for [Directional] lights.
    /// This member represents an attenuation constant.
    /// For information about attenuation, see [Light Properties (Direct3D 9)].
    /// Valid values for this member range from 0.0 to infinity.
    /// For non-[Directional] lights, all three attenuation values should not be set to 0.0 at the same time.
    ///
    /// [Directional]:                      crate::LightType::Directional
    /// [Light Properties (Direct3D 9)]:    https://docs.microsoft.com/en-us/windows/win32/direct3d9/light-properties
    pub attenuation1:   f32,

    /// Value specifying how the light intensity changes over distance.
    /// Attenuation values are ignored for [Directional] lights.
    /// This member represents an attenuation constant.
    /// For information about attenuation, see [Light Properties (Direct3D 9)].
    /// Valid values for this member range from 0.0 to infinity.
    /// For non-[Directional] lights, all three attenuation values should not be set to 0.0 at the same time.
    ///
    /// [Directional]:                      crate::LightType::Directional
    /// [Light Properties (Direct3D 9)]:    https://docs.microsoft.com/en-us/windows/win32/direct3d9/light-properties
    pub attenuation2:   f32,

    /// Angle, in radians, of a spotlight's inner cone - that is, the fully illuminated spotlight cone.
    /// This value must be in the range from 0 through the value specified by [phi](#structfield.phi).
    pub theta:          f32,

    /// Angle, in radians, defining the outer edge of the spotlight's outer cone.
    /// Points outside this cone are not lit by the spotlight.
    /// This value must be between 0 and [std::f32::consts::PI].
    pub phi:            f32,
}

impl Deref    for Light { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DLIGHT9; }
impl DerefMut for Light { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DLIGHT9> for Light { fn from(value: D3DLIGHT9) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Light> for D3DLIGHT9 { fn from(value: Light    ) -> Self { unsafe { std::mem::transmute(value) } } }

#[test] fn layout() {
    let thin = Light::default();
    let d3d  = unsafe { std::mem::zeroed::<D3DLIGHT9>() };

    assert_eq!(std::mem::size_of_val (&thin), std::mem::size_of_val (&d3d));
    assert_eq!(std::mem::align_of_val(&thin), std::mem::align_of_val(&d3d));
    assert_eq!(offset(&thin, &thin.light_type   ), offset(&d3d, &d3d.Type           ));
    assert_eq!(offset(&thin, &thin.diffuse      ), offset(&d3d, &d3d.Diffuse        ));
    assert_eq!(offset(&thin, &thin.specular     ), offset(&d3d, &d3d.Specular       ));
    assert_eq!(offset(&thin, &thin.ambient      ), offset(&d3d, &d3d.Ambient        ));
    assert_eq!(offset(&thin, &thin.position     ), offset(&d3d, &d3d.Position       ));
    assert_eq!(offset(&thin, &thin.direction    ), offset(&d3d, &d3d.Direction      ));
    assert_eq!(offset(&thin, &thin.range        ), offset(&d3d, &d3d.Range          ));
    assert_eq!(offset(&thin, &thin.falloff      ), offset(&d3d, &d3d.Falloff        ));
    assert_eq!(offset(&thin, &thin.attenuation0 ), offset(&d3d, &d3d.Attenuation0   ));
    assert_eq!(offset(&thin, &thin.attenuation1 ), offset(&d3d, &d3d.Attenuation1   ));
    assert_eq!(offset(&thin, &thin.attenuation2 ), offset(&d3d, &d3d.Attenuation2   ));
    assert_eq!(offset(&thin, &thin.theta        ), offset(&d3d, &d3d.Theta          ));
    assert_eq!(offset(&thin, &thin.phi          ), offset(&d3d, &d3d.Phi            ));

    fn offset<S, F>(s: &S, f: &F) -> usize {
        let s : *const S = s;
        let f : *const F = f;
        (f as usize) - (s as usize)
    }
}



/// # Lighting (16-bit)
///
/// Configure (and query) [Light]ing.
///
/// While the underlying Direct3D API takes 32-bit light indicies, I expose 16-bit light indicies by default for soundness.
/// For details about when and why 32-bit light indicies are unsound, see [Lighting (32-bit)](#lighting-32-bit)
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setlight)\]
    /// IDirect3DDevice9::SetLight
    ///
    /// Assigns a set of lighting properties for this device.
    ///
    /// For soundness, this limits `index` to [u16], instead of accepting [u32] like the underlying API does.
    ///
    /// ### Returns
    ///
    /// *   <span class="inaccurate">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// device.set_light(0,  light).unwrap(); // Ok
    /// device.set_light(1,  light).unwrap(); // Ok
    /// device.set_light(!0, light).unwrap(); // Ok (16-bit)
    /// ```
    pub fn set_light(&self, index: u16, light: impl Into<Light>) -> Result<(), MethodError> {
        // Safe: `index` max == `u16::MAX` == `0xFFFF`, well bellow when GArrayT starts buffer overflowing (`0x1000_0000 - 8`)
        unsafe { self.set_light_32_unchecked(index.into(), light.into()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-lightenable)\]
    /// IDirect3DDevice9::LightEnable
    ///
    /// Enables or disables a set of lighting parameters within a device.
    ///
    /// For soundness, this limits `index` to [u16], instead of accepting [u32] like the underlying API does.
    ///
    /// ### Returns
    ///
    /// *   <span class="inaccurate">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// device.light_enable(0,  true).unwrap(); // Ok
    /// device.light_enable(1,  true).unwrap(); // Ok
    /// device.light_enable(!0, true).unwrap(); // Ok (16-bit)
    /// ```
    pub fn light_enable(&self, index: u16, enable: bool) -> Result<(), MethodError> {
        // Safe: `index` max == `u16::MAX` == `0xFFFF`, well bellow when GArrayT starts buffer overflowing (`0x1000_0000 - 8`)
        unsafe { self.light_enable_32_unchecked(index.into(), enable) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlight)\]
    /// IDirect3DDevice9::GetLight
    ///
    /// Get the [Light] that was previously set for this device.
    ///
    /// This API mirrors [set_light] by accepting [u16], instead of the underlying [u32].
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   - if no light was previously set at `index`
    /// *   Ok([Light])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// // Since there's no real way to clear previously set lights,
    /// // I recommend not treating untouched lights special:
    /// let light = device.get_light( 0).unwrap_or(Light::default());
    /// let light = device.get_light(!0).unwrap_or(Light::default());
    ///
    /// // That said, you can:
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light( 0));
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light(!0));
    /// device.set_light(0, Light::default()).unwrap();
    ///
    /// let light0 = device.get_light(0).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light(!0));
    /// ```
    ///
    /// [set_light]:    #fn.set_light
    pub fn get_light(&self, index: u16) -> Result<Light, MethodError> {
        self.get_light_32(index.into())
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlightenable)
    /// IDirect3DDevice9::GetLightEnable
    ///
    /// Queries if the light is enabled.
    ///
    /// This API mirrors [light_enable] by accepting [u16], instead of the underlying [u32].
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the light was never explicitly previously enabled or disabled
    /// *   [D3DERR::INVALIDCALL]       The device is a pure device?
    /// *   Ok(`true`)                  The light is enabled
    /// *   Ok(`false`)                 The light is disabled
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// // Since there's no real way to invalidate previously enabled/disabled lights,
    /// // I recommend not treating untouched lights special:
    /// let enabled = device.get_light_enable( 0).unwrap_or(false);
    /// let enabled = device.get_light_enable(!0).unwrap_or(false);
    ///
    /// // That said, you can:
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable( 0));
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable(!0));
    ///
    /// device.light_enable(!0, false).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable( 0));
    /// assert_eq!(false,               device.get_light_enable(!0).unwrap());
    /// ```
    ///
    /// [light_enable]:    #fn.light_enable
    pub fn get_light_enable(&self, index: u16) -> Result<bool, MethodError> {
        self.get_light_enable_32(index.into())
    }
}



/// # Lighting (32-bit)
///
/// Configure (and query) [Light]ing with 32-bit indicies.
///
/// The `set_*` APIs can buffer overflow and crash (or worse!) if `index` >= `0x1000_0000 - 8` even as recently as Windows 10, with call stacks like:
///
/// ```text
/// d3d9.dll!CHandle::CHandle(void)
/// d3d9.dll!`eh vector constructor iterator'(void *,unsigned __int64,int,void (*)(void *),void (*)(void *))
/// d3d9.dll!GArrayT<struct CHandle>::AllocArray(unsigned long)
/// d3d9.dll!GArrayT<CHandle>::Grow()
/// d3d9.dll!CD3DHal::SetLightI()
/// d3d9.dll!CD3DBase::SetLight()
/// ```
///
/// It appears that some array of 16-byte structures (`CHandle`s?) + header size overflows a 32-bit size.
/// This successfully allocates, and then is copied into, resulting in a buffer overflow.
/// Other, smaller sizes may also crash on other system I haven't tested.
/// Prefer the 16-bit API variations (above) which are 100% sound.
///
/// The `get_*` APIs appear sound, and will not allocate.
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setlight)\]
    /// IDirect3DDevice9::SetLight
    ///
    /// Assigns a set of lighting properties for this device.
    ///
    /// ### Safety
    ///
    /// *   This will buffer overflow and crash (or worse!) if `index` >= `0x1000_0000 - 8` on some systems!
    /// *   Other, smaller sizes may also crash on other system I haven't tested.
    /// *   Prefer [set_light] (limits the index to [u16], and is thus 100% infalliable.)
    ///
    /// [set_light]:                #fn.set_light
    ///
    /// ### Returns
    ///
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// unsafe {
    ///     device.set_light_32_unchecked(0, light).unwrap(); // Ok
    ///     device.set_light_32_unchecked(1, light).unwrap(); // Ok
    ///     // device.set_light_32_unchecked(!0, light).unwrap();
    ///     // XXX: Buffer overflow, crash, or worse!
    /// }
    /// ```
    pub unsafe fn set_light_32_unchecked(&self, index: u32, light: impl Into<Light>) -> Result<(), MethodError> {
        let light = light.into();
        let hr = self.0.SetLight(index, &*light);
        MethodError::check("IDirect3DDevice9::SetLight", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-lightenable)\]
    /// IDirect3DDevice9::LightEnable
    ///
    /// Enables or disables a set of lighting parameters within a device.
    ///
    /// ### Safety
    ///
    /// *   This will buffer overflow and crash (or worse!) if `index` >= `0x1000_0000 - 8` on some systems!
    /// *   Other, smaller sizes may also crash on other system I haven't tested.
    /// *   Prefer [light_enable] (limits the index to [u16], and is thus 100% infalliable.)
    ///
    /// [light_enable]:                #fn.light_enable
    ///
    /// ### Returns
    ///
    /// *   <span class="inaccurate">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// unsafe {
    ///     device.light_enable_32_unchecked(0, true).unwrap();
    ///     device.light_enable_32_unchecked(0, false).unwrap();
    ///     // device.light_enable_32_unchecked(!0, true).unwrap();
    ///     // XXX: Buffer overflow, crash, or worse!
    /// }
    /// ```
    pub unsafe fn light_enable_32_unchecked(&self, index: u32, enable: bool) -> Result<(), MethodError> {
        let hr = self.0.LightEnable(index, enable.into());
        MethodError::check("IDirect3DDevice9::LightEnable", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlight)\]
    /// IDirect3DDevice9::GetLight
    ///
    /// This API appears sound despite the 32-bit indicies
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   - if no light was previously set at `index`
    /// *   Ok([Light])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// for light in [0, 1, 100, 10000, 1000000, !0].iter().copied() {
    ///     assert_eq!(D3DERR::INVALIDCALL, device.get_light_32(light));
    /// }
    ///
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// device.set_light(0, light).unwrap();
    ///
    /// let light = device.get_light_32(0).unwrap();
    /// for light in [1, 100, 10000, 1000000, !0].iter().copied() {
    ///     assert_eq!(D3DERR::INVALIDCALL, device.get_light_32(light));
    /// }
    /// ```
    pub fn get_light_32(&self, index: u32) -> Result<Light, MethodError> {
        let mut light = Light::default();
        let hr = unsafe { self.0.GetLight(index.into(), &mut *light) };
        MethodError::check("IDirect3DDevice9::GetLight", hr)?;
        Ok(light)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlightenable)\]
    /// IDirect3DDevice9::GetLightEnable
    ///
    /// This API appears sound despite the 32-bit indicies
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the light was never explicitly previously enabled or disabled
    /// *   [D3DERR::INVALIDCALL]       The device is a pure device?
    /// *   Ok(`true`)                  The light is enabled
    /// *   Ok(`false`)                 The light is disabled
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// for light in [0, 1, 100, 10000, 1000000, !0].iter().copied() {
    ///     assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable_32(light));
    /// }
    ///
    /// device.light_enable(0, false).unwrap();
    ///
    /// let enabled0 = device.get_light_enable_32(0).unwrap();
    /// assert_eq!(enabled0, false);
    /// for light in [1, 100, 10000, 1000000, !0].iter().copied() {
    ///     assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable_32(light));
    /// }
    /// ```
    pub fn get_light_enable_32(&self, index: u32) -> Result<bool, MethodError> {
        let mut enable = 0;
        let hr = unsafe { self.0.GetLightEnable(index, &mut enable) };
        MethodError::check("IDirect3DDevice9::GetLightEnable", hr)?;
        Ok(enable != 0)
    }
}
