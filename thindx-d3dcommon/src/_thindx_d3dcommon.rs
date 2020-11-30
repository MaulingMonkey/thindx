//! [d3dcommon.h](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/)
//!
//! | enumeration                   | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | [CBufferType]                 | [D3D_CBUFFER_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_cbuffer_type)
//! | [DriverType]                  | [D3D_DRIVER_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_driver_type)
//! | [FeatureLevel]                | [D3D_FEATURE_LEVEL](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_feature_level)
//! | [IncludeType]                 | [D3D_INCLUDE_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_include_type)
//! | [InterpolationMode]           | [D3D_INTERPOLATION_MODE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_interpolation_mode)
//! | [MinPrecision]                | [D3D_MIN_PRECISION](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_min_precision)
//! | [Name]                        | [D3D_NAME](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_name)
//! | [Primitive]                   | [D3D_PRIMITIVE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive)
//! | [PrimitiveTopology]           | [D3D_PRIMITIVE_TOPOLOGY](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive_topology)
//! | [RegisterComponentType]       | [D3D_REGISTER_COMPONENT_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_register_component_type)
//! | [ResourceReturnType]          | [D3D_RESOURCE_RETURN_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_resource_return_type)
//! | [ShaderInputType]             | [D3D_SHADER_INPUT_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_type)
//! | [ShaderVariableClass]         | [D3D_SHADER_VARIABLE_CLASS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_class)
//! | [ShaderVariableType]          | [D3D_SHADER_VARIABLE_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_type)
//! | [SrvDimension]                | [D3D_SRV_DIMENSION](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_srv_dimension)
//! | [TessellatorDomain]           | [D3D_TESSELLATOR_DOMAIN](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_domain)
//! | [TessellatorOutputPrimitive]  | [D3D_TESSELLATOR_OUTPUT_PRIMITIVE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_output_primitive)
//! | [TessellatorPartitioning]     | [D3D_TESSELLATOR_PARTITIONING](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_partitioning)
//!
//! | flag                          | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | [ParameterFlags]              | [D3D_PARAMETER_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_parameter_flags)
//! | [ShaderCBufferFlags]          | [D3D_SHADER_CBUFFER_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_cbuffer_flags)
//! | [ShaderInputFlags]            | [D3D_SHADER_INPUT_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_flags)
//! | [ShaderVariableFlags]         | [D3D_SHADER_VARIABLE_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_flags)
//!
//! | interface                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | [ReadOnlyBlob]                | [ID3DBlob](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))
//! | [Include], [AsID3DInclude]    | [ID3DInclude](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)
//!
//! | structure                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | [ShaderMacro]                 | [D3D_SHADER_MACRO](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_macro)
//!
//! | trait                         | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | [AsID3DInclude]               | [ID3DInclude](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)
//! | [AsShaderMacros]              | [D3D_SHADER_MACRO](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_macro)\[\]
//!
//! | misc                          | description           |
//! | ----------------------------- | --------------------- |
//! | [Error]                       | { kind: [ErrorKind], method: [Option]\<\&'static [str]\>, errors: Option\<[ReadOnlyBlob]\> }



extern crate thindx_core as core; use core::*;

pub use enumerations::*;
pub use flags::*;
pub use interfaces::*;
pub use traits::*;

pub use error::*;



mod enumerations {
    mod cbuffer_type;                       pub use cbuffer_type::*;
    mod feature_level;                      pub use feature_level::*;
    mod interpolation_mode;                 pub use interpolation_mode::*;
    mod min_precision;                      pub use min_precision::*;
    mod name;                               pub use name::*;
    mod primitive_topology;                 pub use primitive_topology::*;
    mod primitive;                          pub use primitive::*;
    mod register_component_type;            pub use register_component_type::*;
    mod resource_return_type;               pub use resource_return_type::*;
    mod shader_input_type;                  pub use shader_input_type::*;
    mod shader_variable_class;              pub use shader_variable_class::*;
    mod shader_variable_type;               pub use shader_variable_type::*;
    mod srv_dimension;                      pub use srv_dimension::*;
    mod tessellator_domain;                 pub use tessellator_domain::*;
    mod tessellator_output_primitive;       pub use tessellator_output_primitive::*;
    mod tessellator_partitioning;           pub use tessellator_partitioning::*;
}

mod flags {
    mod parameter_flags;                    pub use parameter_flags::*;
    mod shader_cbuffer_flags;               pub use shader_cbuffer_flags::*;
    mod shader_input_flags;                 pub use shader_input_flags::*;
    mod shader_variable_flags;              pub use shader_variable_flags::*;
}

mod interfaces {
    mod blob;                               pub use blob::*;
}

mod traits {
    mod as_id3dinclude;                     pub use as_id3dinclude::*;
    mod as_shader_macros;                   pub use as_shader_macros::*;
}

mod error;
