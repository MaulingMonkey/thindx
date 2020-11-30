#![cfg(windows)]

/* always */                        pub extern crate thindx_core        as core;
#[cfg(feature = "d3d11shader"   )]  pub extern crate thindx_d3d11shader as d3d11shader;
#[cfg(feature = "d3dcommon"     )]  pub extern crate thindx_d3dcommon   as d3dcommon;
#[cfg(feature = "d3dcompiler"   )]  pub extern crate thindx_d3dcompiler as d3dcompiler;
