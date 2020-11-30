pub use enumerations::*;
pub use interfaces::*;
pub use structures::*;

mod enumerations {
    mod shader_version_type;                pub use shader_version_type::*;
}

mod interfaces {
    mod function_linking_graph;             pub use function_linking_graph::*;
    mod function_parameter_reflection;      pub use function_parameter_reflection::*;
    mod function_reflection;                pub use function_reflection::*;
    mod library_reflection;                 pub use library_reflection::*;
    mod linker;                             pub use linker::*;
    mod linking_node;                       pub use linking_node::*;
    mod module_instance;                    pub use module_instance::*;
    mod module;                             pub use module::*;
    mod shader_reflection;                  pub use shader_reflection::*;
    mod shader_reflection_constant_buffer;  pub use shader_reflection_constant_buffer::*;
    mod shader_reflection_type;             pub use shader_reflection_type::*;
    mod shader_reflection_variable;         pub use shader_reflection_variable::*;
}

mod structures {
    #![cfg_attr(not(test), allow(unused_imports))] // TODO: temporary

    mod function_desc;                      pub use function_desc::*;
    mod library_desc;                       pub use library_desc::*;
    mod parameter_desc;                     pub use parameter_desc::*;
    mod shader_buffer_desc;                 pub use shader_buffer_desc::*;
    mod shader_desc;                        pub use shader_desc::*;
    mod shader_input_bind_desc;             pub use shader_input_bind_desc::*;
    mod shader_type_desc;                   pub use shader_type_desc::*;
    mod shader_variable_desc;               pub use shader_variable_desc::*;
    mod signature_parameter_desc;           pub use signature_parameter_desc::*;
}
