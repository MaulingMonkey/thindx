//! Thin DirectX wrappers for Rust.
//! 
//! ### Why not `winapi` directly?
//! 
//! *   This crate aims to make fns safe/sound/slightly rusty when possible
//! *   Attempts to verify API soundness through unit tests, even if this will mostly test DirectX's behavior.
//! *   Doc comments for one-stop intellisense, safety documentation, etc.
//!
//! ### Why not `<other graphics crate>`?
//! 
//! *   Most other graphics crates focus on **hiding** the underlying APIs as much as possible, improving portability.
//! *   This crate **surfaces** the underlying API `1:1`ish as much as possible, improving the potential for
//!     interoperability with other graphics code, or for accomplishing incremental rewrites.
//!
//! ### Is this crate sound?
//! 
//! Probably not.  I'm exposing a huge legacy C++ API to Rust.  Mistakes will happen.
//! 
//! That said, soundness *is* a **very high priority** goal.  `thindx` will add things like extra bounds checks, parameter
//! validation, extra init, etc. if need be in order to ensure soundness in safe fns whenever possible.  When it's not
//! possible to validate unsoundness away, the fns in question should be marked `unsafe`.  This crate strives to be sounder
//! than whatever manual FFI you'd write yourself would be, and that's a **high** bar.
//! 
//! But there are some practical limits to this.  If a background driver thread invokes UB if it fails to allocate memory,
//! without any direct correlation to specific API misuse like a large integer overflowing, that's a bug I can't sanely
//! mitigate via safe fns.  I mean, theoretically I could write a pure-software clone of the entire DirectX runtime... but no.
//! 
//! Additionally, while I'm seeking to validate my APIs via testing, older implementations of the APIs in question may have
//! more bugs / unchecked parameters / ??? that I'll fail to mitigate due to being unable to trigger them myself.  While I'm
//! happy to investigate, accept pull requests, expand test coverage, etc. it's worth assuming this crate is unsound on
//! older versions unless you've tested yourself.

#![deny(broken_intra_doc_links)]
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
