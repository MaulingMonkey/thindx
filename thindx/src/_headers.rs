#![cfg_attr(stable, allow(private_intra_doc_links))]

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
//! | interface                                 | d3d11shader.h                                                                                                                                                 | description   |
//! | ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d11::FunctionLinkingGraph]             | [ID3D11FunctionLinkingGraph](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionlinkinggraph)                        | A function-linking-graph interface is used for constructing shaders that consist of a sequence of precompiled function calls that pass values to each other.
//! | [d3d11::FunctionParameterReflection]      | [ID3D11FunctionParameterReflection](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionparameterreflection)          | A function-parameter-reflection interface accesses function-parameter info.
//! | [d3d11::FunctionReflection]               | [ID3D11FunctionReflection](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionreflection)                            | A function-reflection interface accesses function info.
//! | [d3d11::LibraryReflection]                | [ID3D11LibraryReflection](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11libraryreflection)                              | A library-reflection interface accesses library info.
//! | [d3d11::Linker]                           | [ID3D11Linker](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11linker)                                                    | A linker interface is used to link a shader module.
//! | [d3d11::LinkingNode]                      | [ID3D11LinkingNode](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11linkingnode)                                          | A linking-node interface is used for shader linking.
//! | [d3d11::Module]                           | [ID3D11Module](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11module)                                                    | A module interface creates an instance of a module that is used for resource rebinding.
//! | [d3d11::ModuleInstance]                   | [ID3D11ModuleInstance](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11moduleinstance)                                    | A module-instance interface is used for resource rebinding.
//! | [d3d11::ShaderReflection]                 | [ID3D11ShaderReflection](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflection)                                | A shader-reflection interface accesses shader information.
//! | [d3d11::ShaderReflectionConstantBuffer]   | [ID3D11ShaderReflectionConstantBuffer](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectionconstantbuffer)    | This shader-reflection interface provides access to a constant buffer.
//! | [d3d11::ShaderReflectionType]             | [ID3D11ShaderReflectionType](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectiontype)                        | This shader-reflection interface provides access to variable type.
//! | [d3d11::ShaderReflectionVariable]         | [ID3D11ShaderReflectionVariable](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectionvariable)                | This shader-reflection interface provides access to a variable.
//!
//! | structure                         | d3d11shader.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d11::FunctionDesc]             | [D3D11_FUNCTION_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_function_desc)                                      | Describes a function.
//! | [d3d11::LibraryDesc]              | [D3D11_LIBRARY_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_library_desc)                                        | Describes a library.
//! | [d3d11::ParameterDesc]            | [D3D11_PARAMETER_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_parameter_desc)                                    | Describes a function parameter.
//! | [d3d11::ShaderBufferDesc]         | [D3D11_SHADER_BUFFER_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_buffer_desc)                            | Describes a shader constant-buffer.
//! | [d3d11::ShaderDesc]               | [D3D11_SHADER_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_desc)                                          | Describes a shader.
//! | [d3d11::ShaderInputBindDesc]      | [D3D11_SHADER_INPUT_BIND_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_input_bind_desc)                    | Describes how a shader resource is bound to a shader input.
//! | [d3d11::ShaderTypeDesc]           | [D3D11_SHADER_TYPE_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_type_desc)                                | Describes a shader-variable type.
//! | [d3d11::ShaderVariableDesc]       | [D3D11_SHADER_VARIABLE_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_variable_desc)                        | Describes a shader variable.
//! | [d3d11::SignatureParameterDesc]   | [D3D11_SIGNATURE_PARAMETER_DESC](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_signature_parameter_desc)                | Describes a shader signature.
//!
//! | enumeration                       | d3d11shader.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d11::ShaderVersionType]        | [D3D11_SHADER_VERSION_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ne-d3d11shader-d3d11_shader_version_type)                          | Indicates shader type.
//!
//!
//!
//! # d3dcommon.h
//!
//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/)\] Common Direct3D types shared between Direct3D11, Compiler, etc.
//!
//! | enumeration                       | d3dcommon.h                                                                                                                                       | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d::CBufferType]                | [D3D_CBUFFER_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_cbuffer_type)                                    | Values that identify the intended use of constant-buffer data.
//! | [d3d::DriverType]                 | [D3D_DRIVER_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_driver_type)                                      | Driver type options.
//! | [d3d::FeatureLevel]               | [D3D_FEATURE_LEVEL](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_feature_level)                                  | Describes the set of features targeted by a Direct3D device.
//! | [d3d::IncludeType]                | [D3D_INCLUDE_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_include_type)                                    | Values that indicate the location of a shader
//! | [d3d::InterpolationMode]          | [D3D_INTERPOLATION_MODE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_interpolation_mode)                        | Specifies interpolation mode, which affects how values are calculated during rasterization.
//! | [d3d::MinPrecision]               | [D3D_MIN_PRECISION](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_min_precision)                                  | Values that indicate the minimum desired interpolation precision.
//! | [d3d::Name]                       | [D3D_NAME](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_name)                                                    | Values that identify shader parameters that use system-value semantics.
//! | [d3d::Primitive]                  | [D3D_PRIMITIVE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive)                                          | Indicates how the pipeline interprets geometry or hull shader input primitives.
//! | [d3d::PrimitiveTopology]          | [D3D_PRIMITIVE_TOPOLOGY](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive_topology)                        | Values that indicate how the pipeline interprets vertex data that is bound to the input-assembler stage. These primitive topology values determine how the vertex data is rendered on screen.
//! | [d3d::RegisterComponentType]      | [D3D_REGISTER_COMPONENT_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_register_component_type)              | Values that identify the data types that can be stored in a register.
//! | [d3d::ResourceReturnType]         | [D3D_RESOURCE_RETURN_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_resource_return_type)                    | Indicates return value type.
//! | [d3d::ShaderInputType]            | [D3D_SHADER_INPUT_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_type)                          | Values that identify resource types that can be bound to a shader and that are reflected as part of the resource description for the shader.
//! | [d3d::ShaderVariableClass]        | [D3D_SHADER_VARIABLE_CLASS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_class)                  | Values that identify the class of a shader variable.
//! | [d3d::ShaderVariableType]         | [D3D_SHADER_VARIABLE_TYPE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_type)                    | Values that identify various data, texture, and buffer types that can be assigned to a shader variable.
//! | [d3d::SrvDimension]               | [D3D_SRV_DIMENSION](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_srv_dimension)                                  | Values that identify the type of resource to be viewed as a shader resource.
//! | [d3d::TessellatorDomain]          | [D3D_TESSELLATOR_DOMAIN](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_domain)                        | Domain options for tessellator data.
//! | [d3d::TessellatorOutputPrimitive] | [D3D_TESSELLATOR_OUTPUT_PRIMITIVE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_output_primitive)    | Output primitive types.
//! | [d3d::TessellatorPartitioning]    | [D3D_TESSELLATOR_PARTITIONING](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_partitioning)            | Partitioning options.
//!
//! | flag                          | d3dcommon.h                                                                                                                                       | description   |
//! | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d::ParameterFlags]         | [D3D_PARAMETER_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_parameter_flags)                              | Indicates semantic flags for function parameters.
//! | [d3d::ShaderCbufferFlags]     | [D3D_SHADER_CBUFFER_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_cbuffer_flags)                    | Values that identify the indended use of a constant-data buffer.
//! | [d3d::ShaderInputFlags]       | [D3D_SHADER_INPUT_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_flags)                        | Values that identify shader-input options.
//! | [d3d::ShaderVariableFlags]    | [D3D_SHADER_VARIABLE_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_variable_flags)                  | Values that identify information about a shader variable.
//!
//! | interface                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | [d3d::Blob]           | [ID3D10Blob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3d10blob) / [ID3DBlob](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))
//! | trait [d3d::AsID3DInclude]    | [ID3DInclude](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)
//!
//! | structure                     | d3dcommon.h           |
//! | ----------------------------- | --------------------- |
//! | trait [d3d::AsShaderMacros]   | [D3D_SHADER_MACRO](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ns-d3dcommon-d3d_shader_macro)
//!
//!
//!
//! # d3dcompiler.h
//! \[[D3DCompiler Reference](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-d3dcompiler-reference), [d3dcompiler.h header](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/)\]
//! HLSL compilation, linking, disassembly, reflection, archival, and utility APIs
//!
//! | function                                                  | d3dcompiler.h                                                                                                                                                 | description   |
//! | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d::Compiler::compile]                                  | [D3DCompile](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCompile)                                                        | Compile HLSL code or an effect file into bytecode for a given target.
//! | [d3d::Compiler::compile2]                                 | [D3DCompile2](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCompile2)                                                      | Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
//! | [d3d::Compiler::compile_from_file]                        | [D3DCompileFromFile](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCompileFromFile)                                        | Compiles Microsoft High Level Shader Language (HLSL) code into bytecode for a given target.
//! | [d3d::Compiler::compress_shaders]                         | [D3DCompressShaders](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCompressShaders)                                        | Compresses a set of shaders into a more compact form.
//! | ~~d3d::Compiler::create_blob~~ (make a read/write blob?)  | [D3DCreateBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCreateBlob)                                                  | Creates a buffer.
//! | [d3d::Compiler::create_read_only_blob]                    | [D3DCreateBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCreateBlob)                                                  | Creates a buffer.
//! | [d3d::Compiler::create_function_linking_graph]            | [D3DCreateFunctionLinkingGraph](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCreateFunctionLinkingGraph)                  | Creates a function-linking-graph interface.
//! | [d3d::Compiler::create_linker]                            | [D3DCreateLinker](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DCreateLinker)                                              | Creates a linker interface. Note  This function is part of the HLSL shader linking technology that you can use on all Direct3D 11 platforms to create precompiled HLSL functions, package them into libraries, and link them into full shaders at run time.
//! | [d3d::Compiler::decompress_shaders]                       | [D3DDecompressShaders](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DDecompressShaders)                                    | Decompresses one or more shaders from a compressed set.
//! | [d3d::Compiler::disassemble]                              | [D3DDisassemble](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DDisassemble)                                                | Disassembles compiled HLSL code.
//! | ~~d3d::Compiler::disassemble_10_effect~~ (use 11?)        | [D3DDisassemble10Effect](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DDisassemble10Effect)                                | Disassembles compiled HLSL code from a Direct3D10 effect.
//! | [d3d::Compiler::disassemble_region]                       | [D3DDisassembleRegion](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DDisassembleRegion)                                    | Disassembles a specific region of compiled Microsoft High Level Shader Language (HLSL) code.
//! | [d3d::Compiler::get_blob_part]                            | [D3DGetBlobPart](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetBlobPart)                                                | Retrieves a specific part from a compilation result.
//! | [d3d::Compiler::get_debug_info]                           | [D3DGetDebugInfo](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetDebugInfo)                                              | Note  You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store. Gets shader debug information.
//! | [d3d::Compiler::get_input_and_output_signature_blob]      | [D3DGetInputAndOutputSignatureBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetInputAndOutputSignatureBlob)          | Note  D3DGetInputAndOutputSignatureBlob may be altered or unavailable for releases after Windows 8.1. Instead use D3DGetBlobPart with the D3D_BLOB_INPUT_AND_OUTPUT_SIGNATURE_BLOB value.  Gets the input and output signatures from a compilation result.
//! | [d3d::Compiler::get_input_signature_blob]                 | [D3DGetInputSignatureBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetInputSignatureBlob)                            | Note  D3DGetInputSignatureBlob may be altered or unavailable for releases after Windows 8.1. Instead use D3DGetBlobPart with the D3D_BLOB_INPUT_SIGNATURE_BLOB value.  Gets the input signature from a compilation result.
//! | [d3d::Compiler::get_output_signature_blob]                | [D3DGetOutputSignatureBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetOutputSignatureBlob)                          | Note  D3DGetOutputSignatureBlob may be altered or unavailable for releases after Windows 8.1. Instead use D3DGetBlobPart with the D3D_BLOB_OUTPUT_SIGNATURE_BLOB value.  Gets the output signature from a compilation result.
//! | [d3d::Compiler::get_trace_instruction_offsets]            | [D3DGetTraceInstructionOffsets](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DGetTraceInstructionOffsets)                  | Retrieves the byte offsets for instructions within a section of shader code.
//! | [d3d::Compiler::load_module]                              | [D3DLoadModule](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DLoadModule)                                                  | Creates a shader module interface from source data for the shader module.
//! | [d3d::Compiler::preprocess]                               | [D3DPreprocess](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DPreprocess)                                                  | Preprocesses uncompiled HLSL code.
//! | [d3d::Compiler::read_file_to_blob]                        | [D3DReadFileToBlob](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DReadFileToBlob)                                          | Reads a file that is on disk into memory.
//! | [d3d::Compiler::reflect]                                  | [D3DReflect](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DReflect)                                                        | Gets a pointer to a reflection interface.
//! | [d3d::Compiler::reflect_library]                          | [D3DReflectLibrary](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DReflectLibrary)                                          | Creates a library-reflection interface from source data that contains an HLSL library of functions.
//! | [d3d::Compiler::set_blob_part]                            | [D3DSetBlobPart](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DSetBlobPart)                                                | Sets information in a compilation result.
//! | [d3d::Compiler::strip_shader]                             | [D3DStripShader](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DStripShader)                                                | Removes unwanted blobs from a compilation result.
//! | [d3d::Compiler::write_blob_to_file]                       | [D3DWriteBlobToFile](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-D3DWriteBlobToFile)                                        | Writes a memory blob to a file on disk.
//!
//! | structure                         | d3dcompiler.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d::ShaderData]                 | [D3D_SHADER_DATA](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ns-d3dcompiler-d3d_shader_data)                                              | Describes shader data.
//!
//! | enumeration                       | d3dcompiler.h                                                                                                                                                 | description   |
//! | --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d::BlobPart]                   | [D3D_BLOB_PART](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3d_blob_part)                                                  | Values that identify parts of the content of an arbitrary length data buffer.
//!
//! | flag                              | d3dcompiler.h                                                                                                                                                         | description   |
//! | --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------- |
//! | [d3d::CompilerStripFlags]         | [D3DCOMPILER_STRIP_FLAGS](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3dcompiler_strip_flags)                                      | Strip flag options.
//! | **flag constants**                |                                                                                                                                                                       |
//! | [d3d::CompileEffect]              | [D3DCOMPILE_EFFECT_*](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-effect-constants) / UINT                                                 |
//! | [d3d::CompileSecdata]             | [D3DCOMPILE_SECDATA_*](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile2#parameters) / UINT                                   |
//! | [d3d::Compile]                    | [D3DCOMPILE_*](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-constants) / UINT                                                               |
//! | [d3d::CompressShader]             | [D3D_COMPRESS_SHADER_*](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompressshaders#parameters) / UINT                           |
//! | [d3d::Disasm]                     | [D3D_DISASM_*](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddisassembleregion#parameters) / UINT                                  |
//! | [d3d::GetInstOffsets]             | [D3D_GET_INST_OFFSETS_*](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets#parameters) / UINT               |
//! | [d3d::ShaderRequires]             | [D3D_SHADER_REQUIRES_*](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getrequiresflags#return-value) / UINT64  |

use crate::*;
