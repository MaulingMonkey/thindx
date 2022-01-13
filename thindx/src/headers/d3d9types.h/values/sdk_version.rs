#[allow(unused_imports)] use crate::d3d9::*;

use winapi::shared::d3d9::{D3D_SDK_VERSION, D3D9b_SDK_VERSION};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-direct3dcreate9)\]
/// A D3D(9b)_SDK_VERSION for use with {[IDirect3D9Ext](IDirect3D9Ext::create), [IDirect3D9ExExt](IDirect3D9ExExt::create)}::[create](IDirect3D9ExExt::create)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SdkVersion(u32);

impl SdkVersion {
    /// ### ⚠️ Safety ⚠️
    /// The SDK version here corresponds to breaking changes in `d3d9.h` resulting in changes to built applications.
    /// By using `SdkVersion::default()`, you use the `D3D_SDK_VERSION` winapi - and by extension this crate - was built with.
    /// By using `SdkVersion::from(...)`, you can use `D3D_SDK_VERSION` from another version of the header, that may not be compatible.
    //#allow_missing_argument_docs
    pub unsafe fn from(dword: u32) -> Self { Self(dword) }

    // See #ifdef D3D_DEBUG_INFO nonsense in `d3d9.h`
    #[must_use] pub const fn with_debug_enabled(self)  -> Self { Self(self.0 |  0x80000000) }
    #[must_use] pub const fn with_debug_disabled(self) -> Self { Self(self.0 & !0x80000000) }
    #[must_use] pub const fn with_debug(self, enabled: bool) -> Self {
        if enabled {
            self.with_debug_enabled()
        } else {
            self.with_debug_disabled()
        }
    }

    /// D3D_SDK_VERSION
    pub const DEFAULT   : SdkVersion = SdkVersion(D3D_SDK_VERSION  ).with_debug(cfg!(debug_assertions));

    /// D3D9b_SDK_VERSION
    pub const DEFAULT9B : SdkVersion = SdkVersion(D3D9b_SDK_VERSION).with_debug(cfg!(debug_assertions));
}

impl Default for SdkVersion { fn default() -> Self { Self::DEFAULT } }
impl From<SdkVersion> for u32 { fn from(sdk: SdkVersion) -> Self { sdk.0 } }
