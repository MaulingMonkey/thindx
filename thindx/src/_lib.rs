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

mods! {
    inl mod namespaces {
        pub mod d3d;
        pub mod d3d9;
        pub mod d3d11;
        pub mod xinput;
    }

    #[path="headers/d3d9.h/d3d9.rs"]                mod d3d9_h;         // d3d9 mod
    #[path="headers/d3d9types.h/d3d9types.rs"]      mod d3d9types_h;    // d3d9 mod
    #[path="headers/d3d11shader.h/d3d11shader.rs"]  mod d3d11shader_h;  // d3d11 mod
    #[path="headers/d3dcommon.h/d3dcommon.rs"]      mod d3dcommon_h;    // d3d mod
    #[path="headers/d3dcompiler.h/d3dcompiler.rs"]  mod d3dcompiler_h;  // d3d mod
    #[path="headers/guiddef.h/guiddef.rs"]          inl mod guiddef_h;
    #[path="headers/unknwn.h/unknwn.rs"]            inl mod unknwn_h;
    #[path="headers/xinput.h/xinput.rs"]            mod xinput_h;

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
