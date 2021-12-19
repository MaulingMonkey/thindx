//! Use [d3d::Compiler] to create a [d3d11::FunctionLinkingGraph] and create shaders
use thindx::*;
use thindx::d3d::*;
use thindx::d3d11::*;

fn main() {
    let compiler = d3d::Compiler::new(47).unwrap();
    let lib_source = b"export float4 xyz1(float3 v) { return float4(v, 1.0); }";
    let lib_bytecode = compiler.compile(lib_source, "example.hlsl", None, None, (), "lib_5_0", Compile::OptimizationLevel3, CompileEffect::None).unwrap();
    let lib = compiler.load_module(lib_bytecode.shader.as_bytes()).unwrap();



    // Use FunctionLinkingGraph to create a shader.  Note that the fn call order
    // here is brittle, reordering many of the calls here will cause E::FAIL errors.

    let graph : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();

    let input = graph.set_input_signature(&[
        ParameterDesc::new(cstr!("inputPos"),  cstr!("POSITION0"), SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
        ParameterDesc::new(cstr!("inputTex"),  cstr!("TEXCOORD0"), SVT::Float, SVC::Vector, 1, 2, Interpolation::Linear, PF::In, 0, 0, 0, 0),
        ParameterDesc::new(cstr!("inputNorm"), cstr!("NORMAL0"),   SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ]).unwrap();

    let xyz1 = graph.call_function("", &lib, "xyz1").unwrap();

    let output = graph.set_output_signature(&[
        ParameterDesc::new(cstr!("outputTex"),  cstr!("TEXCOORD0"),   SVT::Float, SVC::Vector, 1, 2, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
        ParameterDesc::new(cstr!("outputNorm"), cstr!("NORMAL0"),     SVT::Float, SVC::Vector, 1, 3, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
        ParameterDesc::new(None,                cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
    ]).unwrap();

    // pass input[0] ("inputPos")   to "xyz1"s args[0] ("v")
    // pass xyz1[-1] (return)       to output[2] ("outputPos")
    // pass input[1] ("inputTex")   to output[0] ("outputTex")
    // pass input[2].yx ("inputNorm.yx")  to output[1].xy ("outputNorm.xy")
    graph.pass_value(&input, 0, &xyz1, 0).unwrap();
    graph.pass_value(&xyz1, -1, &output, 2).unwrap();
    graph.pass_value(&input, 1, &output, 0).unwrap();
    graph.pass_value_with_swizzle(&input, 2, "yx", &output, 1, "xy").unwrap();


    // Option A:  Generate HLSL to process further manually
    println!("{}", graph.generate_hlsl(()).unwrap().to_utf8_lossy());


    // Option B:  Link HLSL
    let (graph_inst, _warnings) = graph.create_module_instance().unwrap();
    let lib_inst = lib.create_instance("").unwrap();
    let linker = compiler.create_linker().unwrap();
    linker.use_library(&lib_inst).unwrap();
    linker.link(&graph_inst, "main", "vs_5_0", None).unwrap();
}
