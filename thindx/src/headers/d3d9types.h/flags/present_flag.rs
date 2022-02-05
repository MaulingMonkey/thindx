#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;
type D3DPRESENTFLAG = u32; // there's no actual type



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpresentflag)\]
/// DWORD / D3DPRESENTFLAG_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct PresentFlag(D3DPRESENTFLAG);

flags! {
    PresentFlag => D3DPRESENTFLAG;
    None, LockableBackBuffer, DiscardDepthStencil, DeviceClip, Video, NoAutoRotate, UnprunedMode,
    OverlayLimitedRgb, OverlayYCbCrBt709, OverlayYCbCrXvYCC, RestrictedContent, RestrictSharedResourceDriver,
}

#[allow(non_upper_case_globals)] impl PresentFlag { // These are enum-like
    pub const None                          : PresentFlag = PresentFlag(0);
    pub const LockableBackBuffer            : PresentFlag = PresentFlag(D3DPRESENTFLAG_LOCKABLE_BACKBUFFER);
    pub const DiscardDepthStencil           : PresentFlag = PresentFlag(D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL);
    pub const DeviceClip                    : PresentFlag = PresentFlag(D3DPRESENTFLAG_DEVICECLIP);
    pub const Video                         : PresentFlag = PresentFlag(D3DPRESENTFLAG_VIDEO);

    pub const NoAutoRotate                  : PresentFlag = PresentFlag(D3DPRESENTFLAG_NOAUTOROTATE);
    pub const UnprunedMode                  : PresentFlag = PresentFlag(D3DPRESENTFLAG_UNPRUNEDMODE);
    pub const OverlayLimitedRgb             : PresentFlag = PresentFlag(D3DPRESENTFLAG_OVERLAY_LIMITEDRGB);
    pub const OverlayYCbCrBt709             : PresentFlag = PresentFlag(D3DPRESENTFLAG_OVERLAY_YCbCr_BT709);
    pub const OverlayYCbCrXvYCC             : PresentFlag = PresentFlag(D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC);
    pub const RestrictedContent             : PresentFlag = PresentFlag(D3DPRESENTFLAG_RESTRICTED_CONTENT);
    pub const RestrictSharedResourceDriver  : PresentFlag = PresentFlag(D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER);
}

//#cpp2rust D3DPRESENTFLAG_LOCKABLE_BACKBUFFER              = d3d::PresentFlag::LockableBackBuffer
//#cpp2rust D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL             = d3d::PresentFlag::DiscardDepthStencil
//#cpp2rust D3DPRESENTFLAG_DEVICECLIP                       = d3d::PresentFlag::DeviceClip
//#cpp2rust D3DPRESENTFLAG_VIDEO                            = d3d::PresentFlag::Video

//#cpp2rust D3DPRESENTFLAG_NOAUTOROTATE                     = d3d::PresentFlag::NoAutoRotate
//#cpp2rust D3DPRESENTFLAG_UNPRUNEDMODE                     = d3d::PresentFlag::UnprunedMode
//#cpp2rust D3DPRESENTFLAG_OVERLAY_LIMITEDRGB               = d3d::PresentFlag::OverlayLimitedRgb
//#cpp2rust D3DPRESENTFLAG_OVERLAY_YCbCr_BT709              = d3d::PresentFlag::OverlayYCbCrBt709
//#cpp2rust D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC              = d3d::PresentFlag::OverlayYCbCrXvYCC
//#cpp2rust D3DPRESENTFLAG_RESTRICTED_CONTENT               = d3d::PresentFlag::RestrictedContent
//#cpp2rust D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER  = d3d::PresentFlag::RestrictSharedResourceDriver
