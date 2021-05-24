use super::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9)\]
/// Manages swapping buffers for a view.
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChain(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DSwapChain9>);
