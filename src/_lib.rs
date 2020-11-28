#![allow(broken_intra_doc_links)] // temporary
#![deny(unreachable_patterns)]

#[macro_use] mod macros;



#[path="dll/_dll.rs"]                   mod dll;            pub use dll::*;
#[path="enumerations/_enumerations.rs"] mod enumerations;   pub use enumerations::*;
#[path="flags/_flags.rs"]               mod flags;          pub use flags::*;
#[path="interfaces/_interfaces.rs"]     mod interfaces;     pub use interfaces::*;
#[path="structures/_structures.rs"]     mod structures;     pub use structures::*;
#[path="traits/_traits.rs"]             mod traits;         pub use traits::*;

mod error_kind;                 pub use error_kind::*;
mod error;                      pub use error::*;
