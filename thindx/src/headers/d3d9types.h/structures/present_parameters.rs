use crate::SafeHWND;
use crate::ctypes::*;
use crate::d3d9::*;

use bytemuck::Zeroable;

use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dpresent-parameters)\]
/// D3DPRESENT_PARAMETERS
///
/// Describes presentation parameters.
#[derive(Clone, Debug)]
#[repr(C)] pub struct PresentParameters<'w> {
    pub back_buffer_width:              u32,
    pub back_buffer_height:             u32,
    pub back_buffer_format:             Format,
    pub back_buffer_count:              u32,
    pub multi_sample_type:              MultiSampleType,
    pub multi_sample_quality:           u32,
    pub swap_effect:                    SwapEffect,
    pub device_window:                  Option<SafeHWND<'w>>,
    pub windowed:                       BOOL,
    pub enable_auto_depth_stencil:      BOOL,
    pub auto_depth_stencil_format:      Format,
    pub flags:                          PresentFlag,
    pub full_screen_refresh_rate_in_hz: u32,
    pub presentation_interval:          Present,
}

unsafe impl Zeroable for PresentParameters<'_> {} // can't derive thanks to Option<SafeHWND<'w>>, but should be safe (tm)... probably...

impl PresentParameters<'_> {
    pub const fn zeroed() -> Self { Self {
        back_buffer_width:              0,
        back_buffer_height:             0,
        back_buffer_format:             Format::zeroed(),
        back_buffer_count:              0,
        multi_sample_type:              MultiSampleType::zeroed(),
        multi_sample_quality:           0,
        swap_effect:                    SwapEffect::zeroed(),
        device_window:                  None,
        windowed:                       BOOL::FALSE,
        enable_auto_depth_stencil:      BOOL::FALSE,
        auto_depth_stencil_format:      Format::zeroed(),
        flags:                          PresentFlag::zeroed(),
        full_screen_refresh_rate_in_hz: 0,
        presentation_interval:          Present::zeroed(),
    }}

    /// ### ⚠️ Safety ⚠️
    /// Mostly safe, but this could be used to bypass `device_window`'s [SafeHWND] requirements.
    pub unsafe fn as_mut(&mut self) -> &mut D3DPRESENT_PARAMETERS { unsafe { std::mem::transmute(self) } }
}

struct_mapping! {
    #[derive(unsafe { AsRefD3D, Deref, IntoD3D })]
    PresentParameters<'_> => D3DPRESENT_PARAMETERS {
        back_buffer_width               => BackBufferWidth,
        back_buffer_height              => BackBufferHeight,
        back_buffer_format              => BackBufferFormat,
        back_buffer_count               => BackBufferCount,
        multi_sample_type               => MultiSampleType,
        multi_sample_quality            => MultiSampleQuality,
        swap_effect                     => SwapEffect,
        device_window                   => hDeviceWindow,
        windowed                        => Windowed,
        enable_auto_depth_stencil       => EnableAutoDepthStencil,
        auto_depth_stencil_format       => AutoDepthStencilFormat,
        flags                           => Flags,
        full_screen_refresh_rate_in_hz  => FullScreen_RefreshRateInHz,
        presentation_interval           => PresentationInterval,
    }
}

//#cpp2rust D3DPRESENT_PARAMETERS = d3d::PresentParameters
