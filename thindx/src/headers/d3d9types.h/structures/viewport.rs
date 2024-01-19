use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dviewport9)\]
/// D3DVIEWPORT9
///
/// Defines Viewport properties.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::set_viewport]
/// *   [IDirect3DDevice9Ext::get_viewport]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Viewport {
    /// Pixel coordinate of the upper-left corner of the viewport on the render-target surface.
    /// Unless you want to render to a subset of the surface, this member can be set to 0.
    pub x:      u32,

    /// Pixel coordinate of the upper-left corner of the viewport on the render-target surface.
    /// Unless you want to render to a subset of the surface, this member can be set to 0.
    pub y:      u32,

    /// Width dimension of the clip volume, in pixels.
    /// Unless you are rendering only to a subset of the surface, this member should be set to the width dimension of the render-target surface.
    pub width:  u32,

    /// Height dimension of the clip volume, in pixels.
    /// Unless you are rendering only to a subset of the surface, this member should be set to the height dimension of the render-target surface.
    pub height: u32,

    /// Together with [max_z], value describing the range of depth values into which a scene is to be rendered, the minimum and maximum values of the clip volume.
    /// Most applications set this value to 0.0.
    /// Clipping is performed after applying the projection matrix.
    ///
    /// [max_z]:        #structfield.max_z
    pub min_z:  f32,

    /// Together with [min_z], value describing the range of depth values into which a scene is to be rendered, the minimum and maximum values of the clip volume.
    /// Most applications set this value to 1.0. Clipping is performed after applying the projection matrix.
    ///
    /// [min_z]:        #structfield.min_z
    pub max_z:  f32,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    Viewport => D3DVIEWPORT9 {
        x       => X,
        y       => Y,
        width   => Width,
        height  => Height,
        min_z   => MinZ,
        max_z   => MaxZ,
    }
}

//#cpp2rust D3DVIEWPORT9 = d3d9::Viewport
