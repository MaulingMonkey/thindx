// This file is autogenerated by _xtask.rs

//! # Examples
use crate::*;

/// [d3d::Compiler] construction / storage
///
/// <style>
/// #main { max-width: none; }
/// </style>
///
/// ### Source
/// ```no_run
/// #![allow(unused_variables)]
/// use thindx::d3d;
/// 
/// fn main() {
///     // The simplest option is to simply hardcode a specific version
///     let d3dc = d3d::Compiler::new(47).unwrap();
///     let d3dc = d3d::Compiler::new("d3dcompiler_47.dll").unwrap();
/// 
///     // However, you can potentially allow a range of versions as well
///     let d3dc = (33..47).rev().find_map(|ver| d3d::Compiler::new(ver).ok()).unwrap();
/// 
///     // TLS is also an option
///     thread_local! { static D3DC : d3d::Compiler = d3d::Compiler::new(47).unwrap(); }
/// 
///     // And lazy_static! should be too
///     lazy_static::lazy_static! { static ref D3DC2 : d3d::Compiler = d3d::Compiler::new(47).unwrap(); }
/// 
///     // Init failures are simple std::io::Error s:
///     let err : std::io::Error = d3d::Compiler::new(9001).map(|_d3dc|()).unwrap_err();
///     assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
///     println!("{}", err);
/// }
/// ```
///
/// ### Output
/// ```text
/// Unable to load d3dcompiler_9001.dll: NotFound
/// ```
///
/// ### To run this example yourself
/// ```cmd
/// git clone https://github.com/MaulingMonkey/thindx
/// cd thindx/thindx
/// cargo run --example d3dcompiler-01-construction
/// ```
pub const d3dcompiler_01_construction : () = ();

/// Use [d3d::Compiler::compile] and friends to compile HLSL to bytecode
///
/// <style>
/// #main { max-width: none; }
/// </style>
///
/// ### Source
/// ```no_run
/// #![allow(unused_variables)]
/// use thindx::d3d::{self, *};
/// 
/// fn main() {
///     let compiler = d3d::Compiler::new(47).unwrap();
///     let basic_hlsl   : &[u8] = include_bytes!(r"..\test\data\basic.hlsl");
///     let library_hlsl : &[u8] = include_bytes!(r"..\test\data\library.hlsl");
/// 
///     // Option A:  just preprocess instead of fully compiling
///     let pixel_shader  = compiler.preprocess(basic_hlsl,   (),                      None, None,                  ).unwrap();
///     let vertex_shader = compiler.preprocess(basic_hlsl,   r"test\data\basic.hlsl", None, StandardFileInclude    ).unwrap();
///     let library       = compiler.preprocess(library_hlsl, r"library.hlsl",         None, None,                  ).unwrap();
///     println!("pixel_shader\n============\n{}\n",    pixel_shader .shader);
///     println!("vertex_shader\n=============\n{}\n",  vertex_shader.shader);
///     println!("library\n=======\n{}\n",              library      .shader);
/// 
///     // Option B:  compile_from_file
///     let pixel_shader  = compiler.compile_from_file(r"test\data\basic.hlsl",   None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
///     let vertex_shader = compiler.compile_from_file(r"test\data\basic.hlsl",   None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
///     let library       = compiler.compile_from_file(r"test\data\library.hlsl", None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
///     // resulting blobs are binary data
/// 
///     // Option C:  compile
///     let pixel_shader  = compiler.compile(basic_hlsl,   (),                      None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
///     let vertex_shader = compiler.compile(basic_hlsl,   r"test\data\basic.hlsl", None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
///     let library       = compiler.compile(library_hlsl, r"library.hlsl",         None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
///     // resulting blobs are binary data
/// 
///     // Option D:  compile2
///     let pixel_shader  = compiler.compile2(basic_hlsl,   (), None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
///     let vertex_shader = compiler.compile2(basic_hlsl,   (), None, None, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
///     let library       = compiler.compile2(library_hlsl, (), None, None, (),       "lib_5_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
///     // resulting blobs are binary data
/// 
///     // TODO: show ID3DInclude usage
///     // TODO: show defines usage
///     // TODO: show effects usage?
/// }
/// 
/// // TODO: include hlsl in docs (add an xtask directive?)
/// ```
///
/// ### Output
/// ```text
/// pixel_shader
/// ============
/// #line 1 "C:\\local\\thindx\\thindx\\<memory>"
/// 
/// 
/// struct Vertex {
///     float4 position : POSITION0 ;
///     float4 color : COLOR0 ;
/// } ;
/// 
/// struct VsToPs {
///     float4 color : COLOR0 ;
///     float4 position : SV_POSITION ;
/// } ;
/// 
/// struct Fragment {
///     float4 color : SV_TARGET0 ;
/// } ;
/// 
/// VsToPs vs_main ( Vertex v ) {
///     VsToPs o ;
///     o . color = v . color ;
///     o . position = v . position ;
///     return o ;
/// }
/// 
/// Fragment ps_main ( VsToPs i ) {
///     Fragment o ;
///     o . color = i . color ;
///     return o ;
/// }
/// 
/// 
/// vertex_shader
/// =============
/// #line 1 "C:\\local\\thindx\\thindx\\test\\data\\basic.hlsl"
/// 
/// 
/// struct Vertex {
///     float4 position : POSITION0 ;
///     float4 color : COLOR0 ;
/// } ;
/// 
/// struct VsToPs {
///     float4 color : COLOR0 ;
///     float4 position : SV_POSITION ;
/// } ;
/// 
/// struct Fragment {
///     float4 color : SV_TARGET0 ;
/// } ;
/// 
/// VsToPs vs_main ( Vertex v ) {
///     VsToPs o ;
///     o . color = v . color ;
///     o . position = v . position ;
///     return o ;
/// }
/// 
/// Fragment ps_main ( VsToPs i ) {
///     Fragment o ;
///     o . color = i . color ;
///     return o ;
/// }
/// 
/// 
/// library
/// =======
/// #line 1 "C:\\local\\thindx\\thindx\\library.hlsl"
/// 
/// 
/// cbuffer ExampleCBuffer {
///     float1 scale ;
/// }
/// 
/// export float4 scale4 ( float4 v ) { return v * scale ; }
/// 
/// export float4 xyz1 ( float3 v ) { return float4 ( v , 1.0 ) ; }
/// 
/// 
/// ```
///
/// ### To run this example yourself
/// ```cmd
/// git clone https://github.com/MaulingMonkey/thindx
/// cd thindx/thindx
/// cargo run --example d3dcompiler-02-compile
/// ```
pub const d3dcompiler_02_compile : () = ();

/// Use [d3d::Compiler] to create a [d3d11::FunctionLinkingGraph] and create shaders
///
/// <style>
/// #main { max-width: none; }
/// </style>
///
/// ### Source
/// ```no_run
/// use thindx::*;
/// use thindx::d3d::*;
/// use thindx::d3d11::*;
/// 
/// fn main() {
///     let compiler = d3d::Compiler::new(47).unwrap();
///     let lib_source = b"export float4 xyz1(float3 v) { return float4(v, 1.0); }";
///     let lib_bytecode = compiler.compile(lib_source, "example.hlsl", None, None, (), "lib_5_0", Compile::OptimizationLevel3, CompileEffect::None).unwrap();
///     let lib = compiler.load_module(&lib_bytecode.shader).unwrap();
/// 
/// 
/// 
///     // Use FunctionLinkingGraph to create a shader.  Note that the fn call order
///     // here is brittle, reordering many of the calls here will cause E::FAIL errors.
/// 
///     let graph : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
/// 
///     let input = graph.set_input_signature(&[
///         ParameterDesc::new(cstr!("inputPos"),  cstr!("POSITION0"), SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
///         ParameterDesc::new(cstr!("inputTex"),  cstr!("TEXCOORD0"), SVT::Float, SVC::Vector, 1, 2, Interpolation::Linear, PF::In, 0, 0, 0, 0),
///         ParameterDesc::new(cstr!("inputNorm"), cstr!("NORMAL0"),   SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
///     ]).unwrap();
/// 
///     let xyz1 = graph.call_function("", &lib, "xyz1").unwrap();
/// 
///     let output = graph.set_output_signature(&[
///         ParameterDesc::new(cstr!("outputTex"),  cstr!("TEXCOORD0"),   SVT::Float, SVC::Vector, 1, 2, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
///         ParameterDesc::new(cstr!("outputNorm"), cstr!("NORMAL0"),     SVT::Float, SVC::Vector, 1, 3, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
///         ParameterDesc::new(None,                cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
///     ]).unwrap();
/// 
///     // pass input[0] ("inputPos")   to "xyz1"s args[0] ("v")
///     // pass xyz1[-1] (return)       to output[2] ("outputPos")
///     // pass input[1] ("inputTex")   to output[0] ("outputTex")
///     // pass input[2].yx ("inputNorm.yx")  to output[1].xy ("outputNorm.xy")
///     graph.pass_value(&input, 0, &xyz1, 0).unwrap();
///     graph.pass_value(&xyz1, -1, &output, 2).unwrap();
///     graph.pass_value(&input, 1, &output, 0).unwrap();
///     graph.pass_value_with_swizzle(&input, 2, "yx", &output, 1, "xy").unwrap();
/// 
/// 
///     // Option A:  Generate HLSL to process further manually
///     println!("{}", graph.generate_hlsl(()).unwrap().to_utf8_lossy());
/// 
/// 
///     // Option B:  Link HLSL
///     let (graph_inst, _warnings) = graph.create_module_instance().unwrap();
///     let lib_inst = lib.create_instance("").unwrap();
///     let linker = compiler.create_linker().unwrap();
///     linker.use_library(&lib_inst).unwrap();
///     linker.link(&graph_inst, "main", "vs_5_0", None).unwrap();
/// }
/// ```
///
/// ### Output
/// ```text
/// float4 xyz1(in float3 v);
/// 
/// void main(in float3 inputPos : POSITION0, in float2 inputTex : TEXCOORD0, in float3 inputNorm : NORMAL0, out float2 outputTex : TEXCOORD0, out float3 outputNorm : NORMAL0, out float4 __Output_n2_2 : SV_POSITION)
/// {
///     float4 xyz1_n1_0;
///     float3 xyz1_n1_1;
///     xyz1_n1_1 = inputPos;
///     xyz1_n1_0 = ::xyz1(xyz1_n1_1);
///     outputTex = inputTex;
///     outputNorm.xy = inputNorm.yx;
///     __Output_n2_2 = xyz1_n1_0;
/// }
/// 
/// ```
///
/// ### To run this example yourself
/// ```cmd
/// git clone https://github.com/MaulingMonkey/thindx
/// cd thindx/thindx
/// cargo run --example d3dcompiler-03-link
/// ```
pub const d3dcompiler_03_link : () = ();

/// Use [d3d::Compiler] to inspect shader bytecode
///
/// <style>
/// #main { max-width: none; }
/// </style>
///
/// ### Source
/// ```no_run
/// use thindx::*;
/// use thindx::d3d::*;
/// //use thindx::d3d11::*;
/// 
/// fn dump_library() {
///     let compiler = d3d::Compiler::new(47).unwrap();
///     let library = compiler.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
///     let _library = compiler.reflect_library::<d3d11::LibraryReflection>(&library).unwrap();
///     let library  = compiler.reflect_library_11(&library).unwrap(); // equivalent shorthand
/// 
///     println!("library");
///     println!("=======");
///     println!("{:?}\n", library.get_desc().unwrap());
///     for function in library.functions().unwrap() {
///         let desc = function.get_desc().unwrap();
///         println!("{:#?}\n", desc);
///         // TODO: plenty
///     }
/// }
/// 
/// fn dump_shader() {
///     let compiler = d3d::Compiler::new(47).unwrap();
///     let shader  = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
///     let _shader = compiler.reflect::<d3d11::ShaderReflection>(&shader).unwrap();
///     let shader  = compiler.reflect11(&shader).unwrap(); // equivalent shorthand
/// 
///     let desc = shader.get_desc().unwrap();
///     println!("shader");
///     println!("======");
///     println!(".get_bitwise_instruction_count()      == {:?}", shader.get_bitwise_instruction_count());
///     println!(".get_conversion_instruction_count()   == {:?}", shader.get_conversion_instruction_count());
///     println!(".get_movc_instruction_count()         == {:?}", shader.get_movc_instruction_count());
///     println!(".get_mov_instruction_count()          == {:?}", shader.get_mov_instruction_count());
///     println!(".get_gs_input_primitive()             == {:?}", shader.get_gs_input_primitive());
///     println!(".get_min_feature_level()              == {:?}", shader.get_min_feature_level());
///     println!(".get_num_interface_slots()            == {:?}", shader.get_num_interface_slots());
///     println!(".get_requires_flags()                 == {:?}", shader.get_requires_flags());
///     println!(".get_thread_group_size()              == {:?}", shader.get_thread_group_size());
///     println!(".is_sample_frequency_shader()         == {:?}", shader.is_sample_frequency_shader());
///     println!(".get_input_parameter_desc(..) = {:#?}",  (0..desc.input_parameters            ).map(|i| shader.get_input_parameter_desc(i)            ).collect::<Vec<_>>());
///     println!(".get_output_parameter_desc(..) = {:#?}", (0..desc.output_parameters           ).map(|i| shader.get_output_parameter_desc(i)           ).collect::<Vec<_>>());
///     println!(".get_patch_parameter_desc(..) = {:#?}",  (0..desc.patch_constant_parameters   ).map(|i| shader.get_patch_constant_parameter_desc(i)   ).collect::<Vec<_>>());
///     println!(".get_desc() == {:#?}", desc);
///     // TODO: get_constant_buffer_by_index
///     // TODO: get_constant_buffer_by_name
///     // TODO: get_resource_binding_desc
///     // TODO: get_resource_binding_desc_by_name
///     // TODO: get_variable_by_name
/// }
/// 
/// fn main() {
///     dump_library();
///     print!("\n\n\n");
///     dump_shader();
/// }
/// ```
///
/// ### Output
/// ```text
/// library
/// =======
/// LibraryDesc { creator: "Microsoft (R) HLSL Shader Compiler 10.1", flags: Compile::None, function_count: 2 }
/// 
/// FunctionDesc {
///     version: 4293918800,
///     creator: "Microsoft (R) HLSL Shader Compiler 10.1",
///     flags: Compile::{Debug|NoPreshader},
///     constant_buffers: 1,
///     bound_resources: 1,
///     instruction_count: 2,
///     temp_register_count: 0,
///     temp_array_count: 0,
///     def_count: 0,
///     dcl_count: 2,
///     texture_normal_instructions: 0,
///     texture_load_instructions: 0,
///     texture_comp_instructions: 0,
///     texture_bias_instructions: 0,
///     texture_gradient_instructions: 0,
///     float_instruction_count: 1,
///     int_instruction_count: 0,
///     uint_instruction_count: 0,
///     static_flow_control_count: 1,
///     dynamic_flow_control_count: 0,
///     macro_instruction_count: 0,
///     array_instruction_count: 0,
///     mov_instruction_count: 0,
///     movc_instruction_count: 0,
///     conversion_instruction_count: 0,
///     bitwise_instruction_count: 0,
///     min_feature_level: FeatureLevel::_11_0,
///     required_feature_flags: ShaderRequires::None,
///     name: "scale4",
///     function_parameter_count: 1,
///     has_return: true,
///     has_10_level_9_vertex_shader: false,
///     has_10_level_9_pixel_shader: false,
/// }
/// 
/// FunctionDesc {
///     version: 4293918800,
///     creator: "Microsoft (R) HLSL Shader Compiler 10.1",
///     flags: Compile::{Debug|NoPreshader},
///     constant_buffers: 0,
///     bound_resources: 0,
///     instruction_count: 3,
///     temp_register_count: 0,
///     temp_array_count: 0,
///     def_count: 0,
///     dcl_count: 2,
///     texture_normal_instructions: 0,
///     texture_load_instructions: 0,
///     texture_comp_instructions: 0,
///     texture_bias_instructions: 0,
///     texture_gradient_instructions: 0,
///     float_instruction_count: 0,
///     int_instruction_count: 0,
///     uint_instruction_count: 0,
///     static_flow_control_count: 1,
///     dynamic_flow_control_count: 0,
///     macro_instruction_count: 0,
///     array_instruction_count: 0,
///     mov_instruction_count: 2,
///     movc_instruction_count: 0,
///     conversion_instruction_count: 0,
///     bitwise_instruction_count: 0,
///     min_feature_level: FeatureLevel::_11_0,
///     required_feature_flags: ShaderRequires::None,
///     name: "xyz1",
///     function_parameter_count: 1,
///     has_return: true,
///     has_10_level_9_vertex_shader: false,
///     has_10_level_9_pixel_shader: false,
/// }
/// 
/// 
/// 
/// 
/// shader
/// ======
/// .get_bitwise_instruction_count()      == 0
/// .get_conversion_instruction_count()   == 0
/// .get_movc_instruction_count()         == 0
/// .get_mov_instruction_count()          == 2
/// .get_gs_input_primitive()             == Primitive::Undefined
/// .get_min_feature_level()              == Ok(FeatureLevel::_10_0)
/// .get_num_interface_slots()            == 0
/// .get_requires_flags()                 == 0
/// .get_thread_group_size()              == (0, 0, 0)
/// .is_sample_frequency_shader()         == false
/// .get_input_parameter_desc(..) = [
///     Ok(
///         SignatureParameterDesc {
///             semantic_name: "POSITION",
///             semantic_index: 0,
///             register: 0,
///             system_value_type: Name::Undefined,
///             component_type: RegisterComponent::Float32,
///             mask: 15,
///             read_write_mask: 15,
///             stream: 0,
///             min_precision: MinPrecision::Default,
///         },
///     ),
///     Ok(
///         SignatureParameterDesc {
///             semantic_name: "COLOR",
///             semantic_index: 0,
///             register: 1,
///             system_value_type: Name::Undefined,
///             component_type: RegisterComponent::Float32,
///             mask: 15,
///             read_write_mask: 15,
///             stream: 0,
///             min_precision: MinPrecision::Default,
///         },
///     ),
/// ]
/// .get_output_parameter_desc(..) = [
///     Ok(
///         SignatureParameterDesc {
///             semantic_name: "COLOR",
///             semantic_index: 0,
///             register: 0,
///             system_value_type: Name::Undefined,
///             component_type: RegisterComponent::Float32,
///             mask: 15,
///             read_write_mask: 0,
///             stream: 0,
///             min_precision: MinPrecision::Default,
///         },
///     ),
///     Ok(
///         SignatureParameterDesc {
///             semantic_name: "SV_POSITION",
///             semantic_index: 0,
///             register: 1,
///             system_value_type: Name::Position,
///             component_type: RegisterComponent::Float32,
///             mask: 15,
///             read_write_mask: 0,
///             stream: 0,
///             min_precision: MinPrecision::Default,
///         },
///     ),
/// ]
/// .get_patch_parameter_desc(..) = []
/// .get_desc() == ShaderDesc {
///     version: 65600,
///     creator: "Microsoft (R) HLSL Shader Compiler 10.1",
///     flags: Compile::{Debug|NoPreshader},
///     constant_buffers: 0,
///     bound_resources: 0,
///     input_parameters: 2,
///     output_parameters: 2,
///     instruction_count: 3,
///     temp_register_count: 0,
///     temp_array_count: 0,
///     def_count: 0,
///     dcl_count: 4,
///     texture_normal_instructions: 0,
///     texture_load_instructions: 0,
///     texture_comp_instructions: 0,
///     texture_bias_instructions: 0,
///     texture_gradient_instructions: 0,
///     float_instruction_count: 0,
///     int_instruction_count: 0,
///     uint_instruction_count: 0,
///     static_flow_control_count: 1,
///     dynamic_flow_control_count: 0,
///     macro_instruction_count: 0,
///     array_instruction_count: 0,
///     cut_instruction_count: 0,
///     emit_instruction_count: 0,
///     gs_output_topology: PrimitiveTopology::Undefined,
///     gs_max_output_vertex_count: 0,
///     input_primitive: Primitive::Undefined,
///     patch_constant_parameters: 0,
///     gs_instance_count: 0,
///     control_points: 0,
///     hs_output_primitive: TessellatorOutput::Undefined,
///     hs_partitioning: TessellatorPartitioning::Undefined,
///     tessellator_domain: TessellatorDomain::Undefined,
///     barrier_instructions: 0,
///     interlocked_instructions: 0,
///     texture_store_instructions: 0,
/// }
/// ```
///
/// ### To run this example yourself
/// ```cmd
/// git clone https://github.com/MaulingMonkey/thindx
/// cd thindx/thindx
/// cargo run --example d3dcompiler-04-reflection
/// ```
pub const d3dcompiler_04_reflection : () = ();
