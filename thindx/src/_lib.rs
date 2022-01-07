#![doc = include_str!("../Readme.md")]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants

pub extern crate abibool;
pub extern crate abistr;

#[doc(hidden)] pub use abistr::cstr;



#[macro_use] mod macros;

/// C ABI interop types
#[path=r"ctypes\_ctypes.rs"] pub mod ctypes;

/// Direct3D related types and APIs used across multiple Direct3D versions.
pub mod d3d {
    pub use crate::d3dcommon_h::*;
    pub use crate::d3dcompiler_h::*;
    pub use crate::d3d9types_h::*;
}

/// Direct3D 9 related types and APIs
pub mod d3d9 {
    pub use crate::d3d9_h::*;
    pub use crate::d3d9types_h::*;
}

/// Direct3D 11 related types and APIs (including shader reflection APIs)
pub mod d3d11 {
    pub use crate::d3d11shader_h::*;
}

mods! {
    #[path=r"headers\d3d9.h\d3d9.rs"]               mod d3d9_h;         // d3d9 mod
    #[path=r"headers\d3d9types.h\d3d9types.rs"]     mod d3d9types_h;    // d3d9 mod
    #[path=r"headers\d3d11shader.h\d3d11shader.rs"] mod d3d11shader_h;  // d3d11 mod
    #[path=r"headers\d3dcommon.h\d3dcommon.rs"]     mod d3dcommon_h;    // d3d mod
    #[path=r"headers\d3dcompiler.h\d3dcompiler.rs"] mod d3dcompiler_h;  // d3d mod
    #[path=r"headers\unknwn.h\unknwn.rs"]           inl mod unknwn_h;

    #[path=r"traits\_traits.rs"] inl mod traits;

    inl mod error_kind;
    inl mod error;
    pub mod errors;
    inl mod method_error;
}
#[doc(no_inline)] pub use errors::*;

#[cfg(doc)] pub mod _examples;
#[cfg(doc)] pub mod _headers;
#[cfg(doc)] #[doc(hidden)] pub use _examples as examples;
