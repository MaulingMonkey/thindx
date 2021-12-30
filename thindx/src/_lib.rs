#![doc = include_str!("../Readme.md")]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]

pub extern crate abibool;
pub extern crate abistr;

#[doc(hidden)] pub use abistr::cstr;



#[macro_use] mod macros; #[cfg(test)] use macros::*;
use thindx_zzz_internal_proc_macros::*;

/// C ABI interop types
#[path=r"ctypes\_ctypes.rs"] pub mod ctypes;

/// Direct3D related types and APIs used across multiple Direct3D versions.
pub mod d3d {
    pub use crate::d3dcommon_h::*;
    pub use crate::d3dcompiler_h::*;
}

/// Direct3D 11 related types and APIs (including shader reflection APIs)
pub mod d3d11 {
    pub use crate::d3d11shader_h::*;
}



#[path=r"headers\d3d11shader.h\d3d11shader.rs"] mod d3d11shader_h;  // d3d11 mod
#[path=r"headers\d3dcommon.h\d3dcommon.rs"]     mod d3dcommon_h;    // d3d mod
#[path=r"headers\d3dcompiler.h\d3dcompiler.rs"] mod d3dcompiler_h;  // d3d mod
#[path=r"headers\unknwn.h\unknwn.rs"]           mod unknwn_h;       pub use unknwn_h::*;

#[path=r"traits\_traits.rs"]            mod traits;         pub use traits::*;

mod error_kind;                 pub use error_kind::*;
mod error;                      pub use error::*;
pub mod errors;                 #[doc(no_inline)] pub use errors::*;

#[cfg(doc)] #[path="_examples.rs"] pub mod _examples;
#[cfg(doc)] #[doc(hidden)] pub use _examples as examples;
#[cfg(doc)] pub mod _headers;
