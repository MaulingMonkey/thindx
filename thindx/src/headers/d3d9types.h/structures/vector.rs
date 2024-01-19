use bytemuck::*;

use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dvector)\]
/// D3DVECTOR
///
/// Defines a 3-component [f32] vector.
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    Vector => D3DVECTOR { x => x, y => y, z => z }
}

//#cpp2rust D3DVECTOR = d3d::Vector
