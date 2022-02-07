#![doc = include_str!("../Readme.md")]
#![forbid(unsafe_op_in_unsafe_fn)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants

#![allow(clippy::identity_op)]                  // I like to `<< 0`, `+ 0`, etc. for consistency
#![allow(clippy::missing_safety_doc)]           // I prefer ⚠️ Safety ⚠️ docs
#![allow(clippy::derivable_impls)]              // I do this a lot for explicitness with d3d enums
#![allow(clippy::too_many_arguments)]           // 1:1 mapping to D3D... I don't have much of a choice!

// #![warn(clippy::undocumented_unsafe_blocks)]    // too noisy to implement yet

pub extern crate abibool;
pub extern crate abistr;

#[doc(hidden)] pub use abistr::cstr;



#[macro_use] mod macros;

/// C ABI interop types
#[path="ctypes/_ctypes.rs"] pub mod ctypes;

mods! {
    inl mod namespaces {
        inl mod _root;
        pub mod d3d;
        pub mod d3d9;
        pub mod d3d11;
        pub mod wkpdid;
        pub mod xinput;
    }

    #[path="headers/d3d9.h/d3d9.rs"]                mod d3d9_h;         // d3d9 mod
    #[path="headers/d3d9caps.h/d3d9caps.rs"]        mod d3d9caps_h;     // d3d9 mod
    #[path="headers/d3d9types.h/d3d9types.rs"]      mod d3d9types_h;    // d3d9 mod
    #[path="headers/d3d11shader.h/d3d11shader.rs"]  mod d3d11shader_h;  // d3d11 mod
    #[path="headers/d3dcommon.h/d3dcommon.rs"]      mod d3dcommon_h;    // d3d mod
    #[path="headers/d3dcompiler.h/d3dcompiler.rs"]  mod d3dcompiler_h;  // d3d mod
    #[path="headers/guiddef.h/guiddef.rs"]          mod guiddef_h;
    #[path="headers/unknwn.h/unknwn.rs"]            mod unknwn_h;
    #[path="headers/xinput.h/xinput.rs"]            mod xinput_h;

    #[path="traits/_traits.rs"]                     mod traits;

    pub(crate) mod dll;
    mod error_kind;
    pub mod errors;
    mod method_error_blob;
    mod method_error;
}

#[cfg(doc)] #[doc = include_str!("../doc/changelog.md")] pub mod _changelog {}
#[cfg(doc)] pub mod _examples;
#[cfg(doc)] pub mod _headers;
#[cfg(doc)] #[doc(hidden)] pub use _examples as examples;
