//! Documentation of d3d headers and what types they map to
//!
//! <style>
//! /*
//! #main { max-width: 1500px; }
//! */
//! td:nth-child(3) { display: none !important; }
//! th:nth-child(3) { display: none !important; }
//! </style>
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
//! | enumeration                   | d3dcommon.h                                                                                                                                       | description   |
//! | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [CBufferType]                 | [D3D_CBUFFER_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_cbuffer_type)                                    | Values that identify the intended use of constant-buffer data.
//! | [DriverType]                  | [D3D_DRIVER_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_driver_type)                                      | Driver type options.
//! | [FeatureLevel]                | [D3D_FEATURE_LEVEL](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_feature_level)                                  | Describes the set of features targeted by a Direct3D device.
//! | [IncludeType]                 | [D3D_INCLUDE_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_include_type)                                    | Values that indicate the location of a shader
//! | [InterpolationMode]           | [D3D_INTERPOLATION_MODE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_interpolation_mode)                        | Specifies interpolation mode, which affects how values are calculated during rasterization.
//! | [MinPrecision]                | [D3D_MIN_PRECISION](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_min_precision)                                  | Values that indicate the minimum desired interpolation precision.
//! | [Name]                        | [D3D_NAME](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_name)                                                    | Values that identify shader parameters that use system-value semantics.
//! | [Primitive]                   | [D3D_PRIMITIVE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive)                                          | Indicates how the pipeline interprets geometry or hull shader input primitives.
//! | [PrimitiveTopology]           | [D3D_PRIMITIVE_TOPOLOGY](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive_topology)                        | Values that indicate how the pipeline interprets vertex data that is bound to the input-assembler stage. These primitive topology values determine how the vertex data is rendered on screen.
//! | [RegisterComponentType]       | [D3D_REGISTER_COMPONENT_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_register_component_type)              | Values that identify the data types that can be stored in a register.
//! | [ResourceReturnType]          | [D3D_RESOURCE_RETURN_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_resource_return_type)                    | Indicates return value type.
//! | [ShaderInputType]             | [D3D_SHADER_INPUT_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_type)                          | Values that identify resource types that can be bound to a shader and that are reflected as part of the resource description for the shader.
//! | [ShaderVariableClass]         | [D3D_SHADER_VARIABLE_CLASS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_class)                  | Values that identify the class of a shader variable.
//! | [ShaderVariableType]          | [D3D_SHADER_VARIABLE_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_type)                    | Values that identify various data, texture, and buffer types that can be assigned to a shader variable.
//! | [SrvDimension]                | [D3D_SRV_DIMENSION](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_srv_dimension)                                  | Values that identify the type of resource to be viewed as a shader resource.
//! | [TessellatorDomain]           | [D3D_TESSELLATOR_DOMAIN](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_domain)                        | Domain options for tessellator data.
//! | [TessellatorOutputPrimitive]  | [D3D_TESSELLATOR_OUTPUT_PRIMITIVE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_output_primitive)    | Output primitive types.
//! | [TessellatorPartitioning]     | [D3D_TESSELLATOR_PARTITIONING](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_partitioning)            | Partitioning options.
//!
//! | flag                          | d3dcommon.h                                                                                                                                       | description   |
//! | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [ParameterFlags]              | [D3D_PARAMETER_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_parameter_flags)                              | Indicates semantic flags for function parameters.
//! | [ShaderCbufferFlags]          | [D3D_SHADER_CBUFFER_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_cbuffer_flags)                    | Values that identify the indended use of a constant-data buffer.
//! | [ShaderInputFlags]            | [D3D_SHADER_INPUT_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_flags)                        | Values that identify shader-input options.
//! | [ShaderVariableFlags]         | [D3D_SHADER_VARIABLE_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_flags)                  | Values that identify information about a shader variable.
//!
//! | interface                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | [ReadOnlyBlob]                | [ID3D10Blob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3d10blob) / [ID3DBlob](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))
//! | trait [AsID3DInclude]         | [ID3DInclude](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)
//!
//! | structure                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | trait [AsShaderMacros]        | [D3D_SHADER_MACRO](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ns-d3dcommon-d3d_shader_macro)
//!
//!
//!
//! # d3dcompiler.h
//!
//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/)\] HLSL compilation, linking, disassembly, reflection, archival, and utility APIs
//!
//! | function                                              | d3dcompiler.h                                                                                                                                                 | description   |
//! | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [D3DCompiler::compile]                                | [D3DCompile](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCompile)                                                        | Compile HLSL code or an effect file into bytecode for a given target.
//! | [D3DCompiler::compile2]                               | [D3DCompile2](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCompile2)                                                      | Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
//! | [D3DCompiler::compile_from_file]                      | [D3DCompileFromFile](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCompileFromFile)                                        | Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
//! | [D3DCompiler::compress_shaders]                       | [D3DCompressShaders](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCompressShaders)                                        | Compresses a set of shaders into a more compact form.
//! | ~~D3DCompiler::create_blob~~ (make a read/write blob?)| [D3DCreateBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCreateBlob)                                                  | Creates a buffer.
//! | [D3DCompiler::create_read_only_blob]                  | [D3DCreateBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCreateBlob)                                                  | Creates a buffer.
//! | [D3DCompiler::create_function_linking_graph]          | [D3DCreateFunctionLinkingGraph](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCreateFunctionLinkingGraph)                  | Creates a function-linking-graph interface.
//! | [D3DCompiler::create_linker]                          | [D3DCreateLinker](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCreateLinker)                                              | Creates a linker interface. Note  This function is part of the HLSL shader linking technology that you can use on all Direct3D 11 platforms to create precompiled HLSL functions, package them into libraries, and link them into full shaders at run time.
//! | [D3DCompiler::decompress_shaders]                     | [D3DDecompressShaders](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DDecompressShaders)                                    | Decompresses one or more shaders from a compressed set.
//! | [D3DCompiler::disassemble]                            | [D3DDisassemble](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DDisassemble)                                                | Disassembles compiled HLSL code.
//! | ~~D3DCompiler::disassemble_10_effect~~ (use 11?)      | [D3DDisassemble10Effect](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DDisassemble10Effect)                                | Disassembles compiled HLSL code from a Direct3D10 effect.
//! | [D3DCompiler::disassemble_region]                     | [D3DDisassembleRegion](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DDisassembleRegion)                                    | Disassembles a specific region of compiled Microsoft High Level Shader Language (HLSL) code.
//! | [D3DCompiler::get_blob_part]                          | [D3DGetBlobPart](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetBlobPart)                                                | Retrieves a specific part from a compilation result.
//! | [D3DCompiler::get_debug_info]                         | [D3DGetDebugInfo](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetDebugInfo)                                              | Note  You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store. Gets shader debug information.
//! | [D3DCompiler::get_input_and_output_signature_blob]    | [D3DGetInputAndOutputSignatureBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetInputAndOutputSignatureBlob)          | Note  D3DGetInputAndOutputSignatureBlob may be altered or unavailable for releases after Windows 8.1. Instead use D3DGetBlobPart with the D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB value.  Gets the input and output signatures from a compilation result.
//! | [D3DCompiler::get_input_signature_blob]               | [D3DGetInputSignatureBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetInputSignatureBlob)                            | Note  D3DGetInputSignatureBlob may be altered or unavailable for releases after Windows 8.1. Instead use D3DGetBlobPart with the D3D_BLOB_INPUT_SIGNATURE_BLOB value.  Gets the input signature from a compilation result.
//! | [D3DCompiler::get_output_signature_blob]              | [D3DGetOutputSignatureBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetOutputSignatureBlob)                          | Note  D3DGetOutputSignatureBlob may be altered or unavailable for releases after Windows 8.1. Instead use D3DGetBlobPart with the D3D_BLOB_OUTPUT_SIGNATURE_BLOB value.  Gets the output signature from a compilation result.
//! | [D3DCompiler::get_trace_instruction_offsets]          | [D3DGetTraceInstructionOffsets](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetTraceInstructionOffsets)                  | Retrieves the byte offsets for instructions within a section of shader code.
//! | [D3DCompiler::load_module]                            | [D3DLoadModule](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DLoadModule)                                                  | Creates a shader module interface from source data for the shader module.
//! | [D3DCompiler::preprocess]                             | [D3DPreprocess](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DPreprocess)                                                  | Preprocesses uncompiled HLSL code.
//! | [D3DCompiler::read_file_to_blob]                      | [D3DReadFileToBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DReadFileToBlob)                                          | Reads a file that is on disk into memory.
//! | [D3DCompiler::reflect]                                | [D3DReflect](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DReflect)                                                        | Gets a pointer to a reflection interface.
//! | [D3DCompiler::reflect_library]                        | [D3DReflectLibrary](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DReflectLibrary)                                          | Creates a library-reflection interface from source data that contains an HLSL library of functions.
//! | [D3DCompiler::set_blob_part]                          | [D3DSetBlobPart](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DSetBlobPart)                                                | Sets information in a compilation result.
//! | [D3DCompiler::strip_shader]                           | [D3DStripShader](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DStripShader)                                                | Removes unwanted blobs from a compilation result.
//! | [D3DCompiler::write_blob_to_file]                     | [D3DWriteBlobToFile](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DWriteBlobToFile)                                        | Writes a memory blob to a file on disk.
//!
//! | structure                         | d3dcompiler.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [ShaderData]                      | [D3D_SHADER_DATA](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ns-d3dcompiler-d3d_shader_data)                                              | Describes shader data.
//!
//! | enumeration                       | d3dcompiler.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [BlobPart]                        | [D3D_BLOB_PART](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3d_blob_part)                                                  | Values that identify parts of the content of an arbitrary length data buffer.
//!
//! | flag                              | d3dcompiler.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [CompilerStripFlags]              | [D3DCOMPILER_STRIP_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3dcompiler_strip_flags)                              | Strip flag options.

use crate::*;
use crate::d3d11::*;
