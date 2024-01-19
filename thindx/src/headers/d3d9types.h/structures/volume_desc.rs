use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dvolume-desc)\]
/// D3DVOLUME_DESC
///
/// Describes a [Volume].
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct VolumeDesc {
    pub format:     Format,
    pub ty:         ResourceType,
    pub usage:      Usage,
    pub pool:       Pool,
    pub width:      u32,
    pub height:     u32,
    pub depth:      u32,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    VolumeDesc => D3DVOLUME_DESC {
        format  => Format,
        ty      => Type,
        usage   => Usage,
        pool    => Pool,
        width   => Width,
        height  => Height,
        depth   => Depth,
    }
}

//#cpp2rust D3DVOLUME_DESC = d3d::VolumeDesc
