//! Documentation of d3d headers and what types they map to
//!
//! # [d3dcommon.h](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/)
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
//! | [ShaderCbufferFlags]          | [D3D_SHADER_CBUFFER_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_cbuffer_flags)
//! | [ShaderInputFlags]            | [D3D_SHADER_INPUT_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_flags)
//! | [ShaderVariableFlags]         | [D3D_SHADER_VARIABLE_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_flags)
//!
//! | interface                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | [ReadOnlyBlob]                | [ID3DBlob](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))
//! | trait [AsID3DInclude]         | [ID3DInclude](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)
//!
//! | structure                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | trait [AsShaderMacros]        | [D3D_SHADER_MACRO](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ns-d3dcommon-d3d_shader_macro)

use crate::*;
use crate::d3d11::*;
