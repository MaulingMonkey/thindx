use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsurface-desc)\]
/// D3DSURFACE_DESC
///
/// Describes a [Surface].
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct SurfaceDesc {
    pub format:                 Format,
    pub ty:                     ResourceType,
    pub usage:                  Usage,
    pub pool:                   Pool,
    pub multi_sample_type:      MultiSampleType,
    pub multi_sample_quality:   u32,
    pub width:                  u32,
    pub height:                 u32,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    SurfaceDesc => D3DSURFACE_DESC {
        format                  => Format,
        ty                      => Type,
        usage                   => Usage,
        pool                    => Pool,
        multi_sample_type       => MultiSampleType,
        multi_sample_quality    => MultiSampleQuality,
        width                   => Width,
        height                  => Height,
    }
}

//#cpp2rust D3DSURFACE_DESC = d3d::SurfaceDesc
