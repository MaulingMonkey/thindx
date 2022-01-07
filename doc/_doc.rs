#[doc(hidden)]
pub mod d3d9 {
    pub use thindx::*;
    pub use thindx::d3d::*;
    pub use thindx::d3d9::*;

    // XXX: temporary?

    pub use winapi::shared::d3d9caps::{
        D3DCAPS9,
    };
    pub use winapi::shared::d3d9types::{
        D3DDISPLAYMODE,
        D3DDISPLAYMODEEX,
        D3DPRESENT_PARAMETERS,
    };
    pub use winapi::shared::windef::{
        HWND,
        HMONITOR,
    };
    pub type AdapterIndex = u32;
    pub const D3DADAPTER_DEFAULT : AdapterIndex = 0;
}
