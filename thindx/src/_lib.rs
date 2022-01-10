#![doc = include_str!("../Readme.md")]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants

pub extern crate abibool;
pub extern crate abistr;

#[doc(hidden)] pub use abistr::cstr;



#[macro_use] mod macros;

/// C ABI interop types
#[path="ctypes/_ctypes.rs"] pub mod ctypes;

/// Direct3D related types and APIs used across multiple Direct3D versions.
pub mod d3d {
    pub use crate::d3dcommon_h::*;
    pub use crate::d3dcompiler_h::*;
    pub use crate::d3d9types_h::*;
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/dx9-graphics)\]
/// Direct3D 9 related types and APIs
pub mod d3d9 {
    pub use crate::d3d9_h::*;
    pub use crate::d3d9types_h::*;
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d11/atoc-dx-graphics-direct3d-11)\]
/// Direct3D 11 related types and APIs (including shader reflection APIs)
pub mod d3d11 {
    pub use crate::d3d11shader_h::*;
}

mods! {
    #[path="headers/d3d9.h/d3d9.rs"]                mod d3d9_h;         // d3d9 mod
    #[path="headers/d3d9types.h/d3d9types.rs"]      mod d3d9types_h;    // d3d9 mod
    #[path="headers/d3d11shader.h/d3d11shader.rs"]  mod d3d11shader_h;  // d3d11 mod
    #[path="headers/d3dcommon.h/d3dcommon.rs"]      mod d3dcommon_h;    // d3d mod
    #[path="headers/d3dcompiler.h/d3dcompiler.rs"]  mod d3dcompiler_h;  // d3d mod
    #[path="headers/guiddef.h/guiddef.rs"]          inl mod guiddef_h;
    #[path="headers/unknwn.h/unknwn.rs"]            inl mod unknwn_h;
    #[path="headers/xinput.h/xinput.rs"]            pub mod xinput;

    #[path="traits/_traits.rs"] inl mod traits;

    inl mod error_kind;
    pub mod errors;
    inl mod method_error_blob;
    inl mod method_error;
}
#[doc(no_inline)] pub use errors::*;

#[cfg(doc)] #[doc = include_str!("../doc/changelog.md")] pub mod _changelog {}
#[cfg(doc)] pub mod _examples;
#[cfg(doc)] pub mod _headers;
#[cfg(doc)] #[doc(hidden)] pub use _examples as examples;
