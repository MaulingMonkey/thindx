//#![deny(broken_intra_doc_links)]

use super::*;
use crate::*;
use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9::*;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::wingdi::{RDH_RECTANGLES, RGNDATA, RGNDATAHEADER};

use std::ptr::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9)\]
/// Manages swapping buffers for a view.
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChain(pub(crate) mcom::Rc<IDirect3DSwapChain9>);

unsafe impl AsSafe<IUnknown             > for SwapChain { fn as_safe(&self) -> &IUnknown             { &**self.0 } }
unsafe impl AsSafe<IDirect3DSwapChain9  > for SwapChain { fn as_safe(&self) -> &IDirect3DSwapChain9  { &*self.0 } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9)\]
/// IDirect3DSwapChain9 extension methods
///
/// ### Methods
/// | thindx                                                    | microsoft.com             | Description   |
/// | --------------------------------------------------------- | ------------------------- | ------------- |
/// | [get_back_buffer](Self::get_back_buffer)                  | [GetBackBuffer]           | Retrieves a back buffer from the swap chain of the device.
/// | [get_device](Self::get_device)                            | [GetDevice]               | Retrieves the device associated with the swap chain.
/// | [get_display_mode](Self::get_display_mode)                | [GetDisplayMode]          | Retrieves the current resolution / refresh rate.
/// | [get_front_buffer_data](Self::get_front_buffer_data)      | [GetFrontBufferData]      | Retrieves a copy of the swapchain's front buffer
/// | [get_present_parameters](Self::get_present_parameters)    | [GetPresentParameters]    | Retrieves the presentation parameters associated with a swap chain.
/// | [get_raster_status](Self::get_raster_status)              | [GetRasterStatus]         | Returns vblank/scanline status for the swap chain's presentation monitor.
/// | [present](Self::present)                                  | [Present]                 | Presents the contents of the next back buffer.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::create_additional_swap_chain]
///
/// [GetBackBuffer]:        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getbackbuffer
/// [GetDevice]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getdevice
/// [GetDisplayMode]:       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getdisplaymode
/// [GetFrontBufferData]:   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getfrontbufferdata
/// [GetPresentParameters]: https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getpresentparameters
/// [GetRasterStatus]:      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getrasterstatus
/// [Present]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-present
///
pub trait IDirect3DSwapChain9Ext : AsSafe<IDirect3DSwapChain9> {
    type Device     : From<Device>;
    type Surface    : From<Surface>;

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getbackbuffer)\]
    /// IDirect3DSwapChain9::GetBackBuffer
    ///
    /// Retrieves a back buffer from the swap chain of the device.
    fn get_back_buffer(&self, i_back_buffer: u32, ty: impl Into<BackBufferType>) -> Result<Self::Surface, Error> {
        fn_context!(d3d9::IDirect3DSwapChain9Ext::get_back_buffer => IDirect3DSwapChain9::GetBackBuffer);
        let ty = ty.into().into();
        let mut back_buffer = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetBackBuffer(i_back_buffer, ty, &mut back_buffer) })?;
        Ok(Self::Surface::from(unsafe { Surface::from_raw(back_buffer) }))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getdevice)\]
    /// IDirect3DSwapChain9::GetDevice
    ///
    /// Retrieves the device associated with the swap chain.
    fn get_device(&self) -> Result<Self::Device, Error> {
        fn_context!(d3d9::IDirect3DSwapChain9Ext::get_device => IDirect3DSwapChain9::GetDevice);
        let mut device = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetDevice(&mut device) })?;
        Ok(Self::Device::from(unsafe { Device::from_raw(device) }))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getdisplaymode)\]
    /// IDirect3DSwapChain9::GetDisplayMode
    ///
    /// Retrieves the display mode's spatial resolution, color resolution, and refresh frequency.
    fn get_display_mode(&self) -> Result<DisplayMode, Error> {
        fn_context!(d3d9::IDirect3DSwapChain9Ext::get_display_mode => IDirect3DSwapChain9::GetDisplayMode);
        let mut dm = DisplayMode::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetDisplayMode(dm.as_mut()) })?;
        Ok(dm)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getfrontbufferdata)\]
    /// IDirect3DSwapChain9::GetFrontBufferData
    ///
    /// Retrieves a copy of the swapchain's front buffer
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `dest_surface` may need to belong to the same [`Device`] as `self`
    /// *   `dest_surface` may need to be the size of the entire desktop if the [`Device`] is in windowed mode
    unsafe fn get_front_buffer_data(&self, dest_surface: &impl IDirect3DSurface9Ext) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DSwapChain9Ext::get_front_buffer_data => IDirect3DSwapChain9::GetFrontBufferData);
        fn_check_hr!(unsafe { self.as_winapi().GetFrontBufferData(dest_surface.as_winapi() as *const _ as *mut _) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getpresentparameters)\]
    /// IDirect3DSwapChain9::GetPresentParameters
    ///
    /// Retrieves the presentation parameters associated with a swap chain.
    fn get_present_parameters(&self) -> Result<d3d::PresentParameters<'_>, Error> {
        fn_context!(d3d9::IDirect3DSwapChain9Ext::get_present_parameters => IDirect3DSwapChain9::GetPresentParameters);
        let mut pp = d3d::PresentParameters::zeroed();
        let hr = unsafe { self.as_winapi().GetPresentParameters(pp.as_mut()) };
        debug_assert!(pp.device_window.as_ref().map_or(true, |w| w.is_valid()), "IDirect3DSwapChain9Ext::get_present_parameters: device_window isn't valid");
        fn_check_hr!(hr)?;
        Ok(pp)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-getrasterstatus)\]
    /// IDirect3DSwapChain9::GetRasterStatus
    ///
    /// Returns information describing the raster of the monitor on which the swap chain is presented.
    fn get_raster_status(&self) -> Result<RasterStatus, Error> {
        fn_context!(d3d9::IDirect3DSwapChain9Ext::get_raster_status => IDirect3DSwapChain9::GetRasterStatus);
        let mut rs = RasterStatus::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetRasterStatus(rs.as_mut()) })?;
        Ok(rs)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9-present)\]
    /// IDirect3DSwapChain9::Present
    ///
    /// Presents the contents of the next buffer in the sequence of back buffers owned by the swap chain.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   It's likely unsound to use an invalid, non-null `hwnd`
    /// *   It's likely unsound to use a null `hwnd` if the original `presentation_parameters.hDeviceWindow` is an invalid, non-null HWND
    /// *   Out of bounds rects might also be an issue IDK?
    ///
    /// ### Arguments
    /// *   `source_rect`           - "Must be `..`" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be `..` even then (the entire source surface is presented.)
    /// *   `dest_rect`             - "Must be `..`" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be `..` even then (the entire client area is filled.)
    /// *   `dest_window_override`  - The destination window to render to.  If null / `()`, the runtime uses the `device_window` member of [d3d::PresentParameters] for the presentation.
    /// *   `dirty_region`          - "Must be [None]" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be [None] even then (the entire region will be considered dirty.)  The implementation is free to copy more than the exact dirty region.
    /// *   `flags`                 - Valid values are [Present::None], [Present::DoNotWait], or [Present::LinearContent].
    ///
    /// ### Returns
    /// *   [D3DERR::DEVICEREMOVED]     When you least expect it
    /// *   [D3DERR::DEVICELOST]        When switching into/out-of fullscreen, or when invoking `C:\Windows\System32\DXCap.exe -forcetdr`
    /// *   [D3DERR::INVALIDCALL]       If called within a [IDirect3DDevice9Ext::begin_scene] .. [IDirect3DDevice9Ext::end_scene] section, if the render target is the current render target.
    /// *   Ok(`()`)
    // TODO: ### Example
    fn present<'r>(&self, source_rect: impl IntoRectOrFull, dest_rect: impl IntoRectOrFull, dest_window_override: impl AsHWND, dirty_region: impl Into<Option<&'r RgnData>>, flags: impl Into<Present>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DSwapChain9Ext::present => IDirect3DSwapChain9::Present);
        let source_rect     = source_rect.into_rect();
        let dest_rect       = dest_rect.into_rect();
        let hwnd            = dest_window_override.as_hwnd();
        let dirty_region    = dirty_region.into();
        let flags           = flags.into().into();

        let source_rect     = source_rect   .map_or(null(), |r| &*r).cast();
        let dest_rect       = dest_rect     .map_or(null(), |r| &*r).cast();
        let dirty_region    = match dirty_region {
            None => null::<RGNDATA>(),
            Some(dr) => {
                if dr.rdh.dwSize as usize   != std::mem::size_of::<RGNDATAHEADER>() { return Err(fn_param_error!(dirty_region, THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.iType             != RDH_RECTANGLES                       { return Err(fn_param_error!(dirty_region, THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.nCount as usize   > dr.buffer.len()                       { return Err(fn_param_error!(dirty_region, THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.nRgnSize as usize > std::mem::size_of_val(dr)             { return Err(fn_param_error!(dirty_region, THINERR::INVALID_STRUCT_FIELD)); }
                let dr : *const RgnData = dr;
                dr.cast()
            },
        };

        fn_check_hr!(unsafe { self.as_winapi().Present(source_rect, dest_rect, hwnd, dirty_region, flags) })
    }
}

impl IDirect3DSwapChain9Ext for mcom::Rc<IDirect3DSwapChain9>   { type Device = mcom::Rc<IDirect3DDevice9>; type Surface = mcom::Rc<IDirect3DSurface9>;   }
impl IDirect3DSwapChain9Ext for super::SwapChain                { type Device = super::Device;              type Surface = super::Surface;                }



//#cpp2rust IDirect3DSwapChain9                         = d3d9::SwapChain
//#cpp2rust IDirect3DSwapChain9                         = d3d9::IDirect3DSwapChain9Ext
