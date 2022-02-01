#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;
use winapi::shared::minwindef::DWORD;

use std::fmt::Debug;
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9)\]
/// D3DCLIPSTATUS9
///
/// Describes the current clip status.
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct ClipStatus {
    /// Default: [CS::None]
    pub clip_union:         ClipStatusFlags,

    /// Default: [CS::All] \| 0xfffff000
    pub clip_intersection:  ClipStatusFlags,
}

impl ClipStatus {
    #[allow(missing_docs)]
    pub const fn zeroed() -> Self {
        Self {
            clip_union:         ClipStatusFlags::zeroed(),
            clip_intersection:  ClipStatusFlags::zeroed(),
        }
    }

    // "Initial values are zero for ClipUnion and 0xFFFFFFFF for ClipIntersection"
    // Ref: https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9
    // TODO: impl const fn default() / Default?

    /// Convert a raw [D3DCLIPSTATUS9] value into a [ClipStatus].
    ///
    /// [D3DCLIPSTATUS9]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9
    pub const fn from_unchecked(value: D3DCLIPSTATUS9) -> Self { unsafe { std::mem::transmute(value) } }

    /// Convert a [ClipStatus] into a raw [D3DCLIPSTATUS9].
    ///
    /// [D3DCLIPSTATUS9]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9
    pub const fn into(self) -> D3DCLIPSTATUS9 { unsafe { std::mem::transmute(self) } }
}

impl Deref    for ClipStatus { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DCLIPSTATUS9; }
impl DerefMut for ClipStatus { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl AsRef<D3DCLIPSTATUS9> for ClipStatus { fn as_ref(&    self) -> &    D3DCLIPSTATUS9 { unsafe { std::mem::transmute(self) } } }
impl AsMut<D3DCLIPSTATUS9> for ClipStatus { fn as_mut(&mut self) -> &mut D3DCLIPSTATUS9 { unsafe { std::mem::transmute(self) } } }
impl AsRef<ClipStatus> for D3DCLIPSTATUS9 { fn as_ref(&    self) -> &    ClipStatus     { unsafe { std::mem::transmute(self) } } }
impl AsMut<ClipStatus> for D3DCLIPSTATUS9 { fn as_mut(&mut self) -> &mut ClipStatus     { unsafe { std::mem::transmute(self) } } }
impl From<ClipStatus> for D3DCLIPSTATUS9 { fn from(value: ClipStatus    ) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<D3DCLIPSTATUS9> for ClipStatus { fn from(value: D3DCLIPSTATUS9) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! {
    ClipStatus => D3DCLIPSTATUS9 {
        clip_union          => ClipUnion,
        clip_intersection   => ClipIntersection,
    }
}

//#cpp2rust D3DCLIPSTATUS9  = d3d9::ClipStatus



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9)\]
/// D3DCS_\*
///
/// [d3d9::ClipStatus] flags
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ClipStatusFlags(DWORD);
pub use ClipStatusFlags as CS;

flags! { CS => DWORD; None, All, Left, Right, Top, Bottom, Front, Back, Plane0, Plane1, Plane2, Plane3, Plane4, Plane5 }

#[allow(non_upper_case_globals)] impl CS {
    /// No clip flags.
    pub const fn zeroed() -> Self { Self(0) }

    /// No clip flags.
    pub const None : CS = CS(0);

    /// A combination of "all" clip flags
    ///
    /// **NOTE:** This is equivalent to `0xFFF` (all individual flags), not `0xFFFFFFFF` (all bits.)
    ///
    /// Additionally, note that the default for e.g. [d3d::ClipStatus::clip_intersection] is `0xFFFFFFFF` (all bits!)
    pub const All : CS = CS(D3DCS_ALL);

    /// All verticies are clipped by the left plane of the viewing frustrum.
    pub const Left : CS = CS(D3DCS_LEFT);

    /// All verticies are clipped by the right plane of the viewing frustrum.
    pub const Right : CS = CS(D3DCS_RIGHT);

    /// All verticies are clipped by the top plane of the viewing frustrum.
    pub const Top : CS = CS(D3DCS_TOP);

    /// All verticies are clipped by the bottom plane of the viewing frustrum.
    pub const Bottom : CS = CS(D3DCS_BOTTOM);

    /// All verticies are clipped by the front plane of the viewing frustrum.
    pub const Front : CS = CS(D3DCS_FRONT);

    /// All verticies are clipped by the back plane of the viewing frustrum.
    pub const Back : CS = CS(D3DCS_BACK);

    /// All verticies are clipped by application defined clipping plane 0.
    pub const Plane0 : CS = CS(D3DCS_PLANE0);

    /// All verticies are clipped by application defined clipping plane 1.
    pub const Plane1 : CS = CS(D3DCS_PLANE1);

    /// All verticies are clipped by application defined clipping plane 2.
    pub const Plane2 : CS = CS(D3DCS_PLANE2);

    /// All verticies are clipped by application defined clipping plane 3.
    pub const Plane3 : CS = CS(D3DCS_PLANE3);

    /// All verticies are clipped by application defined clipping plane 4.
    pub const Plane4 : CS = CS(D3DCS_PLANE4);

    /// All verticies are clipped by application defined clipping plane 5.
    pub const Plane5 : CS = CS(D3DCS_PLANE5);
}

//#cpp2rust D3DCS_ALL       = d3d::CS::All
//#cpp2rust D3DCS_LEFT      = d3d::CS::Left
//#cpp2rust D3DCS_RIGHT     = d3d::CS::Right
//#cpp2rust D3DCS_TOP       = d3d::CS::Top
//#cpp2rust D3DCS_BOTTOM    = d3d::CS::Bottom
//#cpp2rust D3DCS_FRONT     = d3d::CS::Front
//#cpp2rust D3DCS_BACK      = d3d::CS::Back
//#cpp2rust D3DCS_PLANE0    = d3d::CS::Plane0
//#cpp2rust D3DCS_PLANE1    = d3d::CS::Plane1
//#cpp2rust D3DCS_PLANE2    = d3d::CS::Plane2
//#cpp2rust D3DCS_PLANE3    = d3d::CS::Plane3
//#cpp2rust D3DCS_PLANE4    = d3d::CS::Plane4
//#cpp2rust D3DCS_PLANE5    = d3d::CS::Plane5
