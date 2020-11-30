//! [d3d11shader.h](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/)

extern crate thindx_core        as core;        use core::*;
extern crate thindx_d3dcommon   as d3dcommon;   use d3dcommon::*;
extern crate thindx_d3dcompiler as d3dcompiler; use d3dcompiler::*;

pub use flags::*;
pub use interfaces::*;
pub use structures::*;

mod flags {
    mod shader_requires;                pub use shader_requires::*;
}

mod interfaces {
}

mod structures {
    mod function_desc;                  pub use function_desc::*;
    mod library_desc;                   pub use library_desc::*;
    mod parameter_desc;                 pub use parameter_desc::*;
    mod shader_buffer_desc;             pub use shader_buffer_desc::*;
    mod shader_desc;                    pub use shader_desc::*;
    mod shader_input_bind_desc;         pub use shader_input_bind_desc::*;
    mod shader_type_desc;               pub use shader_type_desc::*;
    mod shader_variable_desc;           pub use shader_variable_desc::*;
    mod signature_parameter_desc;       pub use signature_parameter_desc::*;
}
