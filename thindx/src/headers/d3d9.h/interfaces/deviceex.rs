#![cfg_attr(not(feature = "9ex"), allow(unused_imports))]

use crate::*;
use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9::{IDirect3DDevice9Ex, IDirect3DDevice9};
use winapi::um::unknwnbase::IUnknown;
use winapi::um::wingdi::*;

use std::convert::TryInto;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9ex)\]
/// (extends [Device])
/// Core interface used for general rendering, resource creation, etc.
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct DeviceEx(pub(crate) mcom::Rc<IDirect3DDevice9Ex>);

#[cfg(feature = "9ex")] unsafe impl AsSafe<IUnknown             > for DeviceEx { fn as_safe(&self) -> &IUnknown            { &***self.0 } }
#[cfg(feature = "9ex")] unsafe impl AsSafe<IDirect3DDevice9     > for DeviceEx { fn as_safe(&self) -> &IDirect3DDevice9    { &**self.0 } }
#[cfg(feature = "9ex")] unsafe impl AsSafe<IDirect3DDevice9Ex   > for DeviceEx { fn as_safe(&self) -> &IDirect3DDevice9Ex  { &*self.0 } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9ex)\]
/// IDirect3DDevice9Ex extension methods
///
/// ### Methods
/// | thindx                                                                        | docs.microsoft.com                | Description |
/// | ----------------------------------------------------------------------------- | --------------------------------- | ----------- |
/// | [check_device_state](Self::check_device_state)                                | [CheckDeviceState]                | Reports the current cooperative-level status of the Direct3D device for a windowed or full-screen application.
/// | [check_resource_residency](Self::check_resource_residency)                    | [CheckResourceResidency]          | Checks an array of resources to determine if it is likely that they will cause a large stall at Draw time because the system must make the resources GPU-accessible.
/// | [compose_rects](Self::compose_rects)                                          | [ComposeRects]                    | Copy a text string to one surface using an alphabet of glyphs on another surface. Composition is done by the GPU using bitwise operations.
/// | [create_depth_stencil_surface_ex](Self::create_depth_stencil_surface_ex)      | [CreateDepthStencilSurfaceEx]     | Creates a depth-stencil surface.
/// | [create_offscreen_plain_surface_ex](Self::create_offscreen_plain_surface_ex)  | [CreateOffscreenPlainSurfaceEx]   | Create an off-screen surface.
/// | [create_render_target_ex](Self::create_render_target_ex)                      | [CreateRenderTargetEx]            | Creates a render-target surface.
/// | [get_display_mode_ex](Self::get_display_mode_ex)                              | [GetDisplayModeEx]                | Retrieves the display mode's spatial resolution, color resolution, refresh frequency, and rotation settings.
/// | [get_gpu_thread_priority](Self::get_gpu_thread_priority)                      | [GetGPUThreadPriority]            | Get the priority of the GPU thread.
/// | [get_maximum_frame_latency](Self::get_maximum_frame_latency)                  | [GetMaximumFrameLatency]          | Retrieves the number of frames of data that the system is allowed to queue.
/// | [present_ex](Self::present_ex)                                                | [PresentEx]                       | Swap the swapchain's next buffer with the front buffer.
/// | [reset_ex](Self::reset_ex)                                                    | [ResetEx]                         | Resets the type, size, and format of the swap chain with all other surfaces persistent.
/// | [set_convolution_mono_kernel](Self::set_convolution_mono_kernel)              | [SetConvolutionMonoKernel]        | Prepare the texture sampler for monochrome convolution filtering on a single-color texture.
/// | [set_gpu_thread_priority](Self::set_gpu_thread_priority)                      | [SetGPUThreadPriority]            | Set the priority on the GPU thread.
/// | [set_maximum_frame_latency](Self::set_maximum_frame_latency)                  | [SetMaximumFrameLatency]          | Set the number of frames that the system is allowed to queue for rendering.
/// | [test_cooperative_level](Self::test_cooperative_level)                        | [TestCooperativeLevel]            | Reports the current cooperative-level status of the Direct3D device for a windowed or full-screen application.
/// | [wait_for_vblank](Self::wait_for_vblank)                                      | [WaitForVBlank]                   | Suspend execution of the calling thread until the next vertical blank signal.
///
/// [CheckDeviceState]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-checkdevicestate
/// [CheckResourceResidency]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-checkresourceresidency
/// [ComposeRects]:                     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-composerects
/// [CreateDepthStencilSurfaceEx]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-createdepthstencilsurfaceex
/// [CreateOffscreenPlainSurfaceEx]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-createoffscreenplainsurfaceex
/// [CreateRenderTargetEx]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-createrendertargetex
/// [GetDisplayModeEx]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-getdisplaymodeex
/// [GetGPUThreadPriority]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-getgputhreadpriority
/// [GetMaximumFrameLatency]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-getmaximumframelatency
/// [PresentEx]:                        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-presentex
/// [ResetEx]:                          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-resetex
/// [SetConvolutionMonoKernel]:         https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-setconvolutionmonokernel
/// [SetGPUThreadPriority]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-setgputhreadpriority
/// [SetMaximumFrameLatency]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-setmaximumframelatency
/// [TestCooperativeLevel]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-testcooperativelevel
/// [WaitForVBlank]:                    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-waitforvblank
///
#[cfg(feature = "9ex")]
pub trait IDirect3DDevice9ExExt : AsSafe<IDirect3DDevice9Ex> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-checkdevicestate)\]
    /// IDirect3DDevice9Ex::CheckDeviceState
    ///
    /// Reports the current cooperative-level status of the Direct3D device for a windowed or full-screen application.
    ///
    /// ### Returns
    /// *   [D3DERR::DEVICEHUNG]
    /// *   [D3DERR::DEVICELOST]
    /// *   [D3DERR::DEVICEREMOVED]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3D::S_PRESENT_OCCLUDED]
    /// *   [D3D::S_PRESENT_MODE_CHANGED]
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn check_device_state(&self, destination_window: impl AsHWND) -> ErrorKind {
        fn_context!(d3d9::IDirect3DDevice9ExExt::check_device_state => IDirect3DDevice9Ex::CheckDeviceState);
        ErrorKind(unsafe { self.as_winapi().CheckDeviceState(destination_window.as_hwnd()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-checkresourceresidency )\]
    /// IDirect3DDevice9Ex::CheckResourceResidency
    ///
    /// Checks an array of resources to determine if it is likely that they will cause a large stall at Draw time because the system must make the resources GPU-accessible.
    ///
    /// ### Returns
    /// *   [THINERR::SLICE_OVERFLOW]   if `resources.len()` > `65535`
    /// *   [D3DERR::INVALIDCALL]       ???
    /// *   Ok(
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    #[doc(hidden)]
    fn _xxx_check_resource_residency(&self, resources: &mut [Resource]) -> ErrorKind {
        fn_context!(d3d9::IDirect3DDevice9ExExt::_xxx_check_resource_residency => IDirect3DDevice9Ex::CheckResourceResidency);
        // FIXME: Taking resources as a value slice is bloody annoying, but we can't cast `&[&Resource]` sanely.
        // FIXME: mut casts bellow are sketch as heck

        // "... up to a maximum of 65535."
        let len : u16 = match resources.len().try_into() {
            Ok(len) => len,
            Err(_) => return THINERR::SLICE_OVERFLOW,
        };
        let len = u32::from(len);
        let resources = resources.as_mut_ptr().cast(); // XXX
        ErrorKind(unsafe { self.as_winapi().CheckResourceResidency(resources, len) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-composerects)\]
    /// IDirect3DDevice9Ex::ComposeRects
    ///
    /// Copy a text string to one surface using an alphabet of glyphs on another surface.
    /// Composition is done by the GPU using bitwise operations.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn compose_rects(&self, src: &Surface, dst: &Surface, src_rect_descs: &VertexBuffer, num_rects: u32, dst_rect_descs: &VertexBuffer, operation: impl Into<ComposeRectsOp>, xoffset: i32, yoffset: i32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::compose_rects => IDirect3DDevice9Ex::ComposeRects);
        let (src, dst, src_rect_descs, dst_rect_descs) = (src.as_raw(), dst.as_raw(), src_rect_descs.as_raw(), dst_rect_descs.as_raw());
        let operation = operation.into().into();

        fn_check_hr!(unsafe { self.as_winapi().ComposeRects(src, dst, src_rect_descs, num_rects, dst_rect_descs, operation, xoffset, yoffset) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-createdepthstencilsurfaceex)\]
    /// IDirect3DDevice9Ex::CreateDepthStencilSurfaceEx
    ///
    /// Creates a depth-stencil surface.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Surface])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn create_depth_stencil_surface_ex(&self, width: u32, height: u32, format: impl Into<Format>, multi_sample: impl Into<MultiSampleType>, multi_sample_quality: u32, discard: bool, _shared_handle: impl SharedHandleParam, usage: impl Into<Usage>) -> Result<Surface, Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::create_depth_stencil_surface_ex => IDirect3DDevice9Ex::CreateDepthStencilSurfaceEx);
        let format = format.into().into();
        let multi_sample = multi_sample.into().into();
        let discard = discard.into();
        let shared_handle = null_mut();
        let usage = usage.into().into();

        let mut surface = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateDepthStencilSurfaceEx(width, height, format, multi_sample, multi_sample_quality, discard, &mut surface, shared_handle, usage) })?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-createoffscreenplainsurfaceex)\]
    /// IDirect3DDevice9Ex::CreateOffscreenPlainSurfaceEx
    ///
    /// Create an off-screen surface.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Surface])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn create_offscreen_plain_surface_ex(&self, width: u32, height: u32, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam, usage: impl Into<Usage>) -> Result<Surface, Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::create_offscreen_plain_surface_ex => IDirect3DDevice9Ex::CreateOffscreenPlainSurfaceEx);
        let format = format.into().into();
        let pool = pool.into().into();
        let shared_handle = null_mut();
        let usage = usage.into().into();

        let mut surface = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateOffscreenPlainSurfaceEx(width, height, format, pool, &mut surface, shared_handle, usage) })?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-createrendertargetex)\]
    /// IDirect3DDevice9Ex::CreateRenderTargetEx
    ///
    /// Creates a render-target surface.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([Surface])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn create_render_target_ex(&self, width: u32, height: u32, format: impl Into<Format>, multi_sample: impl Into<MultiSampleType>, multi_sample_quality: u32, lockable: bool, _shared_handle: impl SharedHandleParam, usage: impl Into<Usage>) -> Result<Surface, Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::create_render_target_ex => IDirect3DDevice9Ex::CreateRenderTargetEx);
        let format = format.into().into();
        let multi_sample = multi_sample.into().into();
        let lockable = lockable.into();
        let shared_handle = null_mut();
        let usage = usage.into().into();

        let mut surface = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateRenderTargetEx(width, height, format, multi_sample, multi_sample_quality, lockable, &mut surface, shared_handle, usage) })?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-getdisplaymodeex)\]
    /// IDirect3DDevice9Ex::GetDisplayModeEx
    ///
    /// Retrieves the display mode's spatial resolution, color resolution, refresh frequency, and rotation settings.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([DisplayModeEx])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn get_display_mode_ex(&self, swap_chain: u32) -> Result<(DisplayModeEx, DisplayRotation), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::get_display_mode_ex => IDirect3DDevice9Ex::GetDisplayModeEx);
        let mut display_mode_ex = DisplayModeEx::zeroed();
        display_mode_ex.size = std::mem::size_of_val(&display_mode_ex).try_into().unwrap();
        let mut rotation = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetDisplayModeEx(swap_chain, &mut *display_mode_ex, &mut rotation) })?;
        Ok((display_mode_ex, DisplayRotation::from_unchecked(rotation)))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-getgputhreadpriority)\]
    /// IDirect3DDevice9Ex::GetGPUThreadPriority
    ///
    /// Get the priority of the GPU thread.
    ///
    /// ### Returns
    /// *   [D3DERR::DEVICEREMOVED]
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`priority`) where `-7` <= `priority` <= `7`
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn get_gpu_thread_priority(&self) -> Result<i32, Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::get_gpu_thread_priority => IDirect3DDevice9Ex::GetGPUThreadPriority);
        let mut pri = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetGPUThreadPriority(&mut pri) })?;
        Ok(pri)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-getmaximumframelatency)\]
    /// IDirect3DDevice9Ex::GetMaximumFrameLatency
    ///
    /// Retrieves the number of frames of data that the system is allowed to queue.
    ///
    /// ### Returns
    /// *   [D3DERR::DEVICELOST]
    /// *   [D3DERR::DEVICEREMOVED]
    /// *   [D3DERR::DRIVERINTERNALERROR]
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   Ok(`frames`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn get_maximum_frame_latency(&self) -> Result<u32, Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::get_maximum_frame_latency => IDirect3DDevice9Ex::GetMaximumFrameLatency);
        let mut max_latency = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetMaximumFrameLatency(&mut max_latency) })?;
        Ok(max_latency)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-presentex)\]
    /// IDirect3DDevice9Ex::PresentEx
    ///
    /// Swap the swapchain's next buffer with the front buffer.
    ///
    /// ### Returns
    /// *   [D3DERR::DEVICELOST]
    /// *   [D3DERR::DEVICEHUNG]
    /// *   [D3DERR::DEVICEREMOVED]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn present_ex<'r>(&self, source_rect: impl IntoRectOrFull, dest_rect: impl IntoRectOrFull, dest_window_override: impl AsHWND, dirty_region: impl Into<Option<&'r RgnData>>, flags: impl Into<Present>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::present_ex => IDirect3DDevice9Ex::PresentEx);
        let source_rect = source_rect.into_rect();
        let dest_rect   = dest_rect.into_rect();
        let source_rect = source_rect.as_ref().map_or(null(), |r| &**r);
        let dest_rect   = dest_rect.as_ref().map_or(null(), |r| &**r);
        let dirty_region= dirty_region.into();
        let flags       = flags.into().into();

        let dirty_region = match dirty_region {
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

        fn_check_hr!(unsafe { self.as_winapi().PresentEx(source_rect.cast(), dest_rect.cast(), dest_window_override.as_hwnd(), dirty_region, flags) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-resetex)\]
    /// IDirect3DDevice9Ex::ResetEx
    ///
    /// Resets the type, size, and format of the swap chain with all other surfaces persistent.
    ///
    /// ### Returns
    /// *   [D3DERR::DEVICELOST]
    /// *   [D3DERR::DEVICEHUNG]
    /// *   [D3DERR::INVALIDCALL] ?
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn reset_ex<'mode>(&self, presentation_parameters: &mut PresentParameters<'static>, fullscreen_display_mode: impl Into<Option<&'mode mut DisplayModeEx>>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::reset_ex => IDirect3DDevice9Ex::ResetEx);
        let fullscreen_display_mode = fullscreen_display_mode.into().map_or(null_mut(), |dm| dm);
        fn_check_hr!(unsafe { self.as_winapi().ResetEx(presentation_parameters.as_mut(), fullscreen_display_mode.cast()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-setconvolutionmonokernel)\]
    /// IDirect3DDevice9Ex::SetConvolutionMonoKernel
    ///
    /// Prepare the texture sampler for monochrome convolution filtering on a single-color texture.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn set_convolution_mono_kernel_unweighted(&self, width: u32, height: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::set_convolution_mono_kernel_unweighted => IDirect3DDevice9Ex::SetConvolutionMonoKernel);
        fn_check_hr!(unsafe { self.as_winapi().SetConvolutionMonoKernel(width, height, null_mut(), null_mut()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-setconvolutionmonokernel)\]
    /// IDirect3DDevice9Ex::SetConvolutionMonoKernel
    ///
    /// Prepare the texture sampler for monochrome convolution filtering on a single-color texture.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn set_convolution_mono_kernel(&self, rows: &mut [f32], cols: &mut [f32]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::set_convolution_mono_kernel => IDirect3DDevice9Ex::SetConvolutionMonoKernel);
        // XXX: should rows/cols be non-mut?  Not sure if d3d *actually* writes those values or not...
        let width  : u32 = rows.len().try_into().map_err(|_| fn_param_error!(rows, THINERR::SLICE_OVERFLOW))?;
        let height : u32 = cols.len().try_into().map_err(|_| fn_param_error!(cols, THINERR::SLICE_OVERFLOW))?;
        fn_check_hr!(unsafe { self.as_winapi().SetConvolutionMonoKernel(width, height, rows.as_mut_ptr(), cols.as_mut_ptr()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-setgputhreadpriority)\]
    /// IDirect3DDevice9Ex::SetGPUThreadPriority
    ///
    /// Set the priority on the GPU thread.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::DEVICEREMOVED]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn set_gpu_thread_priority(&self, priority: i32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::set_gpu_thread_priority => IDirect3DDevice9Ex::SetGPUThreadPriority);
        fn_check_hr!(unsafe { self.as_winapi().SetGPUThreadPriority(priority) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-setmaximumframelatency)\]
    /// IDirect3DDevice9Ex::SetMaximumFrameLatency
    ///
    /// Set the number of frames that the system is allowed to queue for rendering.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn set_maximum_frame_latency(&self, max_latency: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::set_maximum_frame_latency => IDirect3DDevice9Ex::SetMaximumFrameLatency);
        fn_check_hr!(unsafe { self.as_winapi().SetMaximumFrameLatency(max_latency) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-testcooperativelevel)\]
    /// IDirect3DDevice9Ex::TestCooperativeLevel
    ///
    /// Reports the current cooperative-level status of the Direct3D device for a windowed or full-screen application.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    #[deprecated = "docs claim test_cooperative_level is no longer available for use - instead, use check_device_state"]
    fn test_cooperative_level(&self) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::test_cooperative_level => IDirect3DDevice9Ex::TestCooperativeLevel);
        fn_check_hr!(unsafe { self.as_winapi().TestCooperativeLevel() })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9ex-waitforvblank)\]
    /// IDirect3DDevice9Ex::WaitForVBlank
    ///
    ///
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_ex_test();
    /// // TODO
    /// ```
    fn wait_for_vblank(&self, swap_chain: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9ExExt::wait_for_vblank => IDirect3DDevice9Ex::WaitForVBlank);
        fn_check_hr!(unsafe { self.as_winapi().WaitForVBlank(swap_chain) })
    }
}

#[cfg(feature = "9ex")]
impl<T: AsSafe<IDirect3DDevice9Ex>> IDirect3DDevice9ExExt for T {}



//#cpp2rust IDirect3DDevice9Ex                                  = d3d9::DeviceEx
//#cpp2rust IDirect3DDevice9Ex                                  = d3d9::IDirect3DDevice9ExExt
