#![cfg(feature = "9ex")]

use super::*;

use winapi::shared::d3d9::*;
use winapi::shared::minwindef::UINT;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9ex)\]
/// (extends [SwapChain])
/// Adds more querying options.
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChainEx(pub(crate) mcom::Rc<IDirect3DSwapChain9Ex>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9ex)\]
/// IDirect3DSwapChain9Ex extension methods
///
/// ### Methods
///
/// | thin3d9                                                   | docs.microsoft.com        | description   |
/// | --------------------------------------------------------- | ------------------------- | ------------- |
/// | [get_display_mode_ex](Self::get_display_mode_ex)          | [GetDisplayModeEx]        | Retrieves the display mode's spatial resolution, color resolution, refresh frequency, and rotation settings.
/// | [get_last_present_count](Self::get_last_present_count)    | [GetLastPresentCount]     | Returns the number of times the swapchain has been processed.
/// | [get_present_statistics](Self::get_present_statistics)    | [GetPresentStatistics]    | Gets presentation statistics so an application can identify frames that do not have a Present method call.
///
/// [GetDisplayModeEx]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9ex-getdisplaymodeex
/// [GetLastPresentCount]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9ex-getlastpresentcount
/// [GetPresentStatistics]: https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb205901(v=vs.85)
///
pub trait IDirect3DSwapChain9ExExt : private::Sealed {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9ex-getdisplaymodeex)\]
    /// IDirect3DSwapChain9Ex::GetDisplayModeEx
    ///
    /// Retrieves the display mode's spatial resolution, color resolution, refresh frequency, and rotation settings.
    fn get_display_mode_ex(&self) -> Result<(DisplayModeEx, DisplayRotation), MethodError> {
        let mut mode = DisplayModeEx::default();
        let mut rot = 0;
        let hr = unsafe { self.as_winapi().GetDisplayModeEx(&mut *mode, &mut rot) };
        MethodError::check("IDirect3DSwapChain9Ex::GetDisplayModeEx", hr)?;
        Ok((mode, DisplayRotation::from_unchecked(rot)))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9ex-getlastpresentcount)\]
    /// IDirect3DSwapChain9Ex::GetLastPresentCount
    ///
    /// Returns the number of times the swapchain has been processed.
    fn get_last_present_count(&self) -> Result<UINT, MethodError> {
        let mut count = 0;
        let hr = unsafe { self.as_winapi().GetLastPresentCount(&mut count) };
        MethodError::check("IDirect3DSwapChain9Ex::GetLastPresentCount", hr)?;
        Ok(count)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb205901(v=vs.85))\]
    /// IDirect3DSwapChain9Ex::GetPresentStatistics
    ///
    /// Gets presentation statistics so an application can identify frames that do not have a Present method call.
    fn get_present_statistics(&self) -> Result<PresentStats, MethodError> {
        let mut stats = PresentStats::default();
        let hr = unsafe { self.as_winapi().GetPresentStats(&mut *stats) };
        MethodError::check("IDirect3DSwapChain9Ex::GetPresentStatistics", hr)?;
        Ok(stats)
    }
}

impl IDirect3DSwapChain9ExExt for mcom::Rc<IDirect3DSwapChain9Ex>   {}
impl IDirect3DSwapChain9ExExt for super::SwapChainEx                {}

mod private {
    use winapi::shared::d3d9::IDirect3DSwapChain9Ex;
    pub unsafe trait Sealed                                 { fn as_winapi(&self) -> &IDirect3DSwapChain9Ex; }
    unsafe impl Sealed for mcom::Rc<IDirect3DSwapChain9Ex>  { fn as_winapi(&self) -> &IDirect3DSwapChain9Ex { &**self } }
    unsafe impl Sealed for super::SwapChainEx               { fn as_winapi(&self) -> &IDirect3DSwapChain9Ex { &*self.0 } }
}
