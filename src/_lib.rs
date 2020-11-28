#![allow(broken_intra_doc_links)] // temporary
#![deny(unreachable_patterns)]

#[macro_use] mod macros;
#[allow(unused_imports)] use macros::*;

//use mcom::Rc;



#[path="dll/_dll.rs"]               mod dll;          pub use dll::*;
#[path="flags/_flags.rs"]           mod flags;        pub use flags::*;
#[path="interfaces/_interfaces.rs"] mod interfaces;   pub use interfaces::*;
#[path="structures/_structures.rs"] mod structures;   pub use structures::*;

pub use enumerations::*;
mod enumerations {
    mod blob_part;              pub use blob_part::*;
}

pub use traits::*;
mod traits {
    mod as_id3dinclude;         pub use as_id3dinclude::*;
    mod as_shader_macros;       pub use as_shader_macros::*;
    mod raw;                    pub use raw::*;
}

mod error;                      pub use error::*;
