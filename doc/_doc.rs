pub use thin3d9::*;

// TODO: replace most of these with thin3d9 types
pub use winapi::shared::d3d9::D3DADAPTER_DEFAULT;
pub use winapi::shared::d3d9::D3DCREATE_FPU_PRESERVE;
pub use winapi::shared::d3d9types::D3DSWAPEFFECT_DISCARD;
pub use winapi::shared::windef::HMONITOR;

pub use std::ptr::{null, null_mut};



mod device_ext;     pub use device_ext::*;
mod direct3d_ext;   pub use direct3d_ext::*;
