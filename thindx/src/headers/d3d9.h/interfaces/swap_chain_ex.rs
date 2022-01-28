#![cfg_attr(not(feature = "9ex"), allow(unused_imports))]

use crate::*;
use crate::d3d9::*;

use winapi::shared::d3d9::*;
use winapi::shared::minwindef::UINT;
use winapi::um::unknwnbase::IUnknown;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9ex)\]
/// (extends [SwapChain])
/// Adds more querying options.
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChainEx(pub(crate) mcom::Rc<IDirect3DSwapChain9Ex>);

#[cfg(feature = "9ex")] unsafe impl AsSafe<IUnknown             > for SwapChainEx { fn as_safe(&self) -> &IUnknown               { &***self.0 } }
#[cfg(feature = "9ex")] unsafe impl AsSafe<IDirect3DSwapChain9  > for SwapChainEx { fn as_safe(&self) -> &IDirect3DSwapChain9    { &**self.0 } }
#[cfg(feature = "9ex")] unsafe impl AsSafe<IDirect3DSwapChain9Ex> for SwapChainEx { fn as_safe(&self) -> &IDirect3DSwapChain9Ex  { &*self.0 } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9ex)\]
/// IDirect3DSwapChain9Ex extension methods
///
/// ### Methods
///
/// | thindx                                                    | docs.microsoft.com        | description   |
/// | --------------------------------------------------------- | ------------------------- | ------------- |
/// | [get_display_mode_ex](Self::get_display_mode_ex)          | [GetDisplayModeEx]        | Retrieves the display mode's spatial resolution, color resolution, refresh frequency, and rotation settings.
/// | [get_last_present_count](Self::get_last_present_count)    | [GetLastPresentCount]     | Returns the number of times the swapchain has been processed.
/// | [get_present_statistics](Self::get_present_statistics)    | [GetPresentStatistics]    | Gets presentation statistics so an application can identify frames that do not have a Present method call.
///
/// [GetDisplayModeEx]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9ex-getdisplaymodeex
/// [GetLastPresentCount]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3dswapchain9ex-getlastpresentcount
/// [GetPresentStatistics]: https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb205901(v=vs.85)
///
#[cfg(feature = "9ex")]
pub trait IDirect3DSwapChain9ExExt : AsSafe<IDirect3DSwapChain9Ex> {
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

#[cfg(feature = "9ex")] impl IDirect3DSwapChain9ExExt for mcom::Rc<IDirect3DSwapChain9Ex>   {}
#[cfg(feature = "9ex")] impl IDirect3DSwapChain9ExExt for super::SwapChainEx                {}



//#cpp2rust IDirect3DSwapChain9Ex                   = d3d9::SwapChainEx
//#cpp2rust IDirect3DSwapChain9Ex                   = d3d9::IDirect3DSwapChain9ExExt

//#cpp2rust IDirect3DSwapChain9Ex::GetDisplayModeEx             = d3d9::IDirect3DSwapChain9ExExt::get_display_mode_ex
//#cpp2rust IDirect3DSwapChain9Ex::GetLastPresentCount          = d3d9::IDirect3DSwapChain9ExExt::get_last_present_count
//TODO:     XXX: documentation uses "GetPresentStatistics", but some C++ headers/vtables use "GetPresentStats"
//#cpp2rust IDirect3DSwapChain9Ex::GetPresentStatistics         = d3d9::IDirect3DSwapChain9ExExt::get_present_statistics
//#cpp2rust IDirect3DSwapChain9Ex::GetPresentStats              = d3d9::IDirect3DSwapChain9ExExt::get_present_statistics
