#![cfg(feature = "9ex")]

use super::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9ex)\]
/// (extends [SwapChain])
/// Adds more querying options.
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChainEx(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DSwapChain9Ex>);
