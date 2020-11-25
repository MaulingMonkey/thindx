#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dzbuffertype)\]
/// D3DZBUFFERTYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ZBufferType(D3DZBUFFERTYPE);
pub type ZB = ZBufferType;

enumish! { ZB => D3DZBUFFERTYPE; False, True, UseW }

#[allow(non_upper_case_globals)] impl ZBufferType { // These are enum-like
    pub const False : ZBufferType = ZBufferType(D3DZB_FALSE);
    pub const True  : ZBufferType = ZBufferType(D3DZB_TRUE);
    pub const UseW  : ZBufferType = ZBufferType(D3DZB_USEW);
}

impl Default for ZBufferType {
    fn default() -> Self { ZBufferType::False } // 0
}
