#![allow(dead_code)] // TODO: remove

use super::*;

use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9)\]
/// Manages swapping buffers for a view.
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChain(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DSwapChain9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9ex)\]
/// (extends [SwapChain])
/// Adds more querying options.
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChainEx(pub(super) mcom::Rc<winapi::shared::d3d9::IDirect3DSwapChain9Ex>);



impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createadditionalswapchain)\]
    /// IDirect3DDevice9::CreateAdditionalSwapChain
    ///
    /// Creates an additional swap chain for rendering multiple views.
    ///
    /// there is always at least one swap chain (the implicit swap chain) for each device because Direct3D 9 has one swap chain as a property of the device.
    /// Note that any given device can support only one full-screen swap chain.
    /// [Format::UNKNOWN] can be specified for the windowed mode back buffer format when calling [Direct3D::create_device], [Device::reset] and [Device::create_additional_swap_chain].
    /// This means the application does not have to query the current desktop format before calling [create_device] for windowed mode.
    /// For full-screen mode, the back buffer format must be specified.
    ///
    /// ### Safety
    ///
    /// *   The caller's codebase is responsible for ensuring any [HWND]s inside [D3DPRESENT_PARAMETERS] outlive the resulting [SwapChain]s that depend on them.
    ///     See [Direct3D::create_device] for details and guidance about dealing with this lifetime issue.
    ///
    /// ### Returns
    ///
    /// * [D3DERR::NOTAVAILABLE]
    /// * [D3DERR::DEVICELOST]
    /// * [D3DERR::INVALIDCALL]
    /// * [D3DERR::OUTOFVIDEOMEMORY]
    /// * [D3DERR::OUTOFMEMORY]
    /// * Ok([SwapChain])
    ///
    /// ### See Also
    ///
    /// * [Presenting Multiple Views in Windowed Mode (Direct3D 9)](https://docs.microsoft.com/en-us/windows/desktop/direct3d9/presenting-multiple-views-in-windowed-mode)
    ///
    /// [create_device]:            #method.create_device
    pub unsafe fn create_additional_swap_chain(&self, presentation_parameters: &mut D3DPRESENT_PARAMETERS) -> Result<SwapChain, MethodError> {
        let mut swap_chain = null_mut();
        let hr = self.0.CreateAdditionalSwapChain(presentation_parameters, &mut swap_chain);
        MethodError::check("IDirect3DDevice9::CreateAdditionalSwapChain", hr)?;
        Ok(SwapChain::from_raw(swap_chain))
    }
}

// #[test] fn create_additional_swap_chain() {} // TODO
