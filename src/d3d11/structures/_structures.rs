#![cfg_attr(not(test), allow(unused_imports))] // TODO: temporary?

mod function_desc;                      pub use function_desc::*;
mod library_desc;                       pub use library_desc::*;
mod parameter_desc;                     pub use parameter_desc::*;
mod shader_buffer_desc;                 pub use shader_buffer_desc::*;

// D3D11_SHADER_DESC
// D3D11_SHADER_INPUT_BIND_DESC
// D3D11_SHADER_TYPE_DESC
// D3D11_SHADER_VARIABLE_DESC
// D3D11_SIGNATURE_PARAMETER_DESC
