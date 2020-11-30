//! Documentation of d3d headers and what types they map to
//!
//! # d3d11shader.h
//!
//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/)\] Direct3D11 APIs for shader reflection, linking, etc.
//!
//! | interface                         | d3d11shader.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [FunctionLinkingGraph]            | [ID3D11FunctionLinkingGraph](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionlinkinggraph)                        | A function-linking-graph interface is used for constructing shaders that consist of a sequence of precompiled function calls that pass values to each other.
//! | [FunctionParameterReflection]     | [ID3D11FunctionParameterReflection](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionparameterreflection)          | A function-parameter-reflection interface accesses function-parameter info.
//! | [FunctionReflection]              | [ID3D11FunctionReflection](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionreflection)                            | A function-reflection interface accesses function info.
//! | [LibraryReflection]               | [ID3D11LibraryReflection](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11libraryreflection)                              | A library-reflection interface accesses library info.
//! | [Linker]                          | [ID3D11Linker](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11linker)                                                    | A linker interface is used to link a shader module.
//! | [LinkingNode]                     | [ID3D11LinkingNode](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11linkingnode)                                          | A linking-node interface is used for shader linking.
//! | [Module]                          | [ID3D11Module](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11module)                                                    | A module interface creates an instance of a module that is used for resource rebinding.
//! | [ModuleInstance]                  | [ID3D11ModuleInstance](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11moduleinstance)                                    | A module-instance interface is used for resource rebinding.
//! | [ShaderReflection]                | [ID3D11ShaderReflection](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflection)                                | A shader-reflection interface accesses shader information.
//! | [ShaderReflectionConstantBuffer]  | [ID3D11ShaderReflectionConstantBuffer](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectionconstantbuffer)    | This shader-reflection interface provides access to a constant buffer.
//! | [ShaderReflectionType]            | [ID3D11ShaderReflectionType](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectiontype)                        | This shader-reflection interface provides access to variable type.
//! | [ShaderReflectionVariable]        | [ID3D11ShaderReflectionVariable](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectionvariable)                | This shader-reflection interface provides access to a variable.
//!
//! | structure                         | d3d11shader.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [FunctionDesc]                    | [D3D11_FUNCTION_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_function_desc)                                      | Describes a function.
//! | [LibraryDesc]                     | [D3D11_LIBRARY_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_library_desc)                                        | Describes a library.
//! | [ParameterDesc]                   | [D3D11_PARAMETER_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_parameter_desc)                                    | Describes a function parameter.
//! | [ShaderBufferDesc]                | [D3D11_SHADER_BUFFER_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_buffer_desc)                            | Describes a shader constant-buffer.
//! | [ShaderDesc]                      | [D3D11_SHADER_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_desc)                                          | Describes a shader.
//! | [ShaderInputBindDesc]             | [D3D11_SHADER_INPUT_BIND_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_input_bind_desc)                    | Describes how a shader resource is bound to a shader input.
//! | [ShaderTypeDesc]                  | [D3D11_SHADER_TYPE_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_type_desc)                                | Describes a shader-variable type.
//! | [ShaderVariableDesc]              | [D3D11_SHADER_VARIABLE_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_variable_desc)                        | Describes a shader variable.
//! | [SignatureParameterDesc]          | [D3D11_SIGNATURE_PARAMETER_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_signature_parameter_desc)                | Describes a shader signature.
//!
//! | enumeration                       | d3d11shader.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [ShaderVersionType]               | [D3D11_SHADER_VERSION_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ne-d3d11shader-d3d11_shader_version_type)                          | Indicates shader type.
//!
//!
//!
//! # d3dcommon.h
//!
//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/)\] Common Direct3D types shared between Direct3D11, D3DCompiler, etc.
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
//! | [ReadOnlyBlob]                | [ID3D10Blob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3d10blob) / [ID3DBlob](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))
//! | trait [AsID3DInclude]         | [ID3DInclude](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)
//!
//! | structure                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | trait [AsShaderMacros]        | [D3D_SHADER_MACRO](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ns-d3dcommon-d3d_shader_macro)

use crate::*;
use crate::d3d11::*;
