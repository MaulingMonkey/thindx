#![cfg_attr(not(test), allow(unused_imports))] // TODO: temporary?

mod function_desc;                      pub use function_desc::*;
mod library_desc;                       pub use library_desc::*;
mod parameter_desc;                     pub use parameter_desc::*;
mod shader_buffer_desc;                 pub use shader_buffer_desc::*;
mod shader_desc;                        pub use shader_desc::*;
mod shader_input_bind_desc;             pub use shader_input_bind_desc::*;
mod shader_type_desc;                   pub use shader_type_desc::*;
mod shader_variable_desc;               pub use shader_variable_desc::*;

// D3D11_SIGNATURE_PARAMETER_DESC
