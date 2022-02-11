use crate::*;
use crate::ctypes::*;
use crate::d3d::*;
use crate::d3d11::*;

use std::ptr::*;

// TODO REFACTOR:
//  - [ ] Structify Ok result of create_module_instance ?
//  - [ ] Unstructify Ok result of other fns?
//  - [ ] Explicit index types, with a less magic value for return than -1 ?

// TODO TESTING:
//  - [ ] Test bad indicies
//  - [ ] Test bad swizzles
//  - [ ] Test bad fn names containing interior nuls and the like
//  - [ ] See if 'static requirements can be relaxed



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionlinkinggraph)\]
/// ID3D11FunctionLinkingGraph
///
/// A function-linking-graph interface is used for constructing shaders that consist of a
/// sequence of precompiled function calls that pass values to each other.
///
/// > **Note:** This interface is part of the HLSL shader linking technology that you can use on all Direct3D 11 platforms
/// > to create precompiled HLSL functions, package them into libraries, and link them into full shaders at run time.
///
/// ### Example
/// ```rust
/// use thindx::{*, d3d::*, d3d11::*};
/// let d3dc = Compiler::load_system(47).unwrap();
///
///
/// // 1. Create a library of shader functions
///
/// let lib_source = br##"
///     export float4 xyz1(float3 v) { return float4(v, 1.0); }
/// "##;
///
/// let lib_bytecode = d3dc.compile(
///     lib_source, "example.hlsl", None, None, (), "lib_5_0",
///     Compile::OptimizationLevel3, CompileEffect::None
/// ).unwrap();
///
/// let lib = d3dc.load_module(&lib_bytecode).unwrap();
///
///
/// // 2. Use FunctionLinkingGraph to create a shader.  Note that the fn call order
/// // here is brittle, reordering many of the calls here will cause E::FAIL errors.
///
/// let graph : FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
///
/// let input = graph.set_input_signature(&[
///     ParameterDesc::new(cstr!("inputPos"),  cstr!("POSITION0"), SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
///     ParameterDesc::new(cstr!("inputTex"),  cstr!("TEXCOORD0"), SVT::Float, SVC::Vector, 1, 2, Interpolation::Linear, PF::In, 0, 0, 0, 0),
///     ParameterDesc::new(cstr!("inputNorm"), cstr!("NORMAL0"),   SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
/// ]).unwrap();
///
/// let xyz1 = graph.call_function("", &lib, "xyz1").unwrap();
///
/// let output = graph.set_output_signature(&[
///     ParameterDesc::new(cstr!("outputTex"),  cstr!("TEXCOORD0"),   SVT::Float, SVC::Vector, 1, 2, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
///     ParameterDesc::new(cstr!("outputNorm"), cstr!("NORMAL0"),     SVT::Float, SVC::Vector, 1, 3, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
///     ParameterDesc::new(None,                cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
/// ]).unwrap();
///
/// // pass input[0] ("inputPos")   to "xyz1"s args[0] ("v")
/// // pass xyz1[-1] (return)       to output[2] ("outputPos")
/// // pass input[1] ("inputTex")   to output[0] ("outputTex")
/// // pass input[2].yx ("inputNorm.yx")  to output[1].xy ("outputNorm.xy")
/// graph.pass_value(&input, 0, &xyz1, 0).unwrap();
/// graph.pass_value(&xyz1, -1, &output, 2).unwrap();
/// graph.pass_value(&input, 1, &output, 0).unwrap();
/// graph.pass_value_with_swizzle(&input, 2, "yx", &output, 1, "xy").unwrap();
///
///
///
/// // 3.  Option A:  Generate HLSL
/// println!("{}", graph.generate_hlsl(()).unwrap().to_utf8_lossy());
///
///
/// // 3.  Option B:  Link HLSL
/// let (graph_inst, _warnings) = graph.create_module_instance().unwrap();
///
/// let lib_inst = lib.create_instance("").unwrap();
/// let linker = d3dc.create_linker().unwrap();
/// linker.use_library(&lib_inst).unwrap();
/// linker.link(&graph_inst, "main", "vs_5_0", None).unwrap();
/// ```
///
/// ### Output
/// ```hlsl
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
/// ```
///
/// ### See Also
/// *   [d3d::Compiler::create_function_linking_graph]
#[derive(Clone)] #[repr(transparent)]
pub struct FunctionLinkingGraph(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11FunctionLinkingGraph>);

convert!(unsafe FunctionLinkingGraph => Unknown, winapi::um::d3d11shader::ID3D11FunctionLinkingGraph);

impl FunctionLinkingGraph {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-callfunction)\]
    /// ID3D11FunctionLinkingGraph::CallFunction
    ///
    /// Creates a call-function linking node to use in the function-linking-graph.
    ///
    /// ### Arguments
    /// *   `module_instance_namespace`         - The optional namespace for the function, or `None` if no namespace is needed.
    /// *   `module_with_function_prototype`    - A [Module] interface for the library module that contains the function prototype.
    /// *   `function_name`                     - The name of the function.
    ///
    /// ### Errors
    /// *   [E::FAIL]               - if called after [set_output_signature](#fn.set_output_signature)
    /// *   [E::FAIL]               - if `function_name` doesn't exist within `module_with_function_prototype`
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// # let lib_bytecode = d3dc.compile(b"export float4 xyz1(float3 v) { return float4(v, 1.0); }", "example.hlsl", None, None, (), "lib_5_0", Compile::OptimizationLevel3, CompileEffect::None).unwrap();
    /// # let lib = d3dc.load_module(&lib_bytecode).unwrap();
    /// # let graph : FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// # let input  = graph.set_input_signature(&[]).unwrap();
    /// let xyz1 = graph.call_function("example_namespace", &lib, "xyz1").unwrap();
    /// let xyz1 = graph.call_function("", &lib, "xyz1").unwrap();
    /// let xyz1 = graph.call_function((), &lib, "xyz1").unwrap();
    ///
    /// assert_eq!(E::FAIL, graph.call_function("", &lib, "nonexistant").err().unwrap().kind());
    /// ```
    pub fn call_function(
        &self,
        module_instance_namespace:      impl TryIntoAsOptCStr,
        module_with_function_prototype: &Module,
        function_name:                  impl TryIntoAsCStr,
    ) -> Result<LinkingNode, Error> {
        fn_context!(d3d11::FunctionLinkingGraph::call_function => ID3D11FunctionLinkingGraph::CallFunction);
        let ns      = module_instance_namespace.try_into()  .map_err(|e| fn_param_error!(module_instance_namespace, e))?;
        let name    = function_name.try_into()              .map_err(|e| fn_param_error!(function_name, e))?;
        let module  = module_with_function_prototype.as_raw();

        let mut node = null_mut();
        fn_check_hr!(unsafe { self.0.CallFunction(ns.as_opt_cstr(), module, name.as_cstr(), &mut node) })?;
        Ok(unsafe { LinkingNode::from_raw(node) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-createmoduleinstance)\]
    /// ID3D11FunctionLinkingGraph::CreateModuleInstance
    ///
    /// Initializes a shader module from the function-linking-graph object.
    ///
    /// ### Errors
    /// *   [E::FAIL]   - if called before set_output_signature
    /// *   [E::FAIL]   - if the FLG has no nodes
    pub fn create_module_instance(&self) -> Result<(ModuleInstance, TextBlob), ErrorWithBlob> {
        fn_context!(d3d11::FunctionLinkingGraph::create_module_instance => ID3D11FunctionLinkingGraph::CreateModuleInstance);
        // TODO: named tuple return?
        let mut module = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { self.0.CreateModuleInstance(&mut module, &mut errors) };
        let errors = TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors) });
        match fn_check_hr!(hr) {
            Err(error) => Err(ErrorWithBlob { error, errors }),
            Ok(()) => {
                let module = unsafe { ModuleInstance::from_raw(module) };
                Ok((module, errors))
            }
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-generatehlsl)\]
    /// ID3D11FunctionLinkingGraph::GenerateHlsl
    ///
    /// Generates Microsoft High Level Shader Language (HLSL) shader code that represents the function-linking-graph.
    ///
    /// ### Arguments
    /// *   `flags`             - Reserved, initialize with `()`.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// # let flg : FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// let hlsl = flg.generate_hlsl(()).unwrap();
    /// println!("{}", hlsl.to_utf8_lossy());
    /// ```
    ///
    /// ### Output
    /// ```hlsl
    /// void main()
    /// {
    /// }
    /// ```
    pub fn generate_hlsl(&self, flags: ()) -> Result<TextBlob, Error> {
        fn_context!(d3d11::FunctionLinkingGraph::generate_hlsl => ID3D11FunctionLinkingGraph::GenerateHlsl);
        let _ = flags; let flags = 0;

        let mut blob = null_mut();
        fn_check_hr!(unsafe { self.0.GenerateHlsl(flags, &mut blob) })?;
        Ok(TextBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-getlasterror)\]
    /// ID3D11FunctionLinkingGraph::GetLastError
    ///
    /// Gets the error from the last function call of the function-linking-graph.
    ///
    /// ### Returns
    /// *   Ok([TextBlob])              - The errors in question.  If there were no previous errors, the text blob will be empty.
    /// *   Err([ErrorKind])            - Ironically, there was an error in attempting to acquire the errors.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// # let flg : FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// let errors : TextBlob = flg.get_last_error().unwrap();
    /// assert!(errors.is_empty(), "No errors were reported by flg");
    /// ```
    pub fn get_last_error(&self) -> Result<TextBlob, Error> {
        fn_context!(d3d11::FunctionLinkingGraph::get_last_error => ID3D11FunctionLinkingGraph::GetLastError);
        let mut errors = null_mut();
        fn_check_hr!(unsafe { self.0.GetLastError(&mut errors) })?;
        Ok(TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-passvalue)\]
    /// ID3D11FunctionLinkingGraph::PassValue
    ///
    /// Passes a value from a source linking node to a destination linking node.
    ///
    /// ### Arguments
    /// *   `src_node`              - The [`LinkingNode`] that the value will be read from
    /// *   `src_parameter_index`   - The `src_node` index that the value will be read from
    /// *   `dst_node`              - The [`LinkingNode`] that the value will be written to
    /// *   `dst_parameter_index`   - The `src_node` index that the value will be written to
    ///
    /// ### Errors
    /// -   [E::FAIL]           - if multiple values are passed to the same destination
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// # let lib_bytecode = d3dc.compile(b"export float4 xyz1(float3 v) { return float4(v, 1.0); }", "example.hlsl", None, None, (), "lib_5_0", Compile::OptimizationLevel3, CompileEffect::None).unwrap();
    /// # let lib = d3dc.load_module(&lib_bytecode).unwrap();
    /// # let graph : FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// # let input  = graph.set_input_signature(&[ParameterDesc::new(cstr!("inPos"),  cstr!("POSITION0"),   SVT::Float, SVC::Vector, 1, 4, Interpolation::Linear,    PF::In,  0, 0, 0, 0)]).unwrap();
    /// # let output = graph.set_output_signature(&[ParameterDesc::new(cstr!("outPos"), cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0)]).unwrap();
    /// graph.pass_value(&input, 0, &output, 0).unwrap();
    /// ```
    pub fn pass_value(&self, src_node: &LinkingNode, src_parameter_index: i32, dst_node: &LinkingNode, dst_parameter_index: i32) -> Result<(), Error> {
        fn_context!(d3d11::FunctionLinkingGraph::pass_value => ID3D11FunctionLinkingGraph::PassValue);
        fn_check_hr!(unsafe { self.0.PassValue(src_node.as_raw(), src_parameter_index, dst_node.as_raw(), dst_parameter_index) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-passvaluewithswizzle)\]
    /// ID3D11FunctionLinkingGraph::PassValueWithSwizzle
    ///
    /// Passes a value with swizzle from a source linking node to a destination linking node.
    ///
    /// ### Arguments
    /// *   `src_node`              - The [`LinkingNode`] that the value will be read from
    /// *   `src_parameter_index`   - The `src_node` index that the value will be read from
    /// *   `src_swizzle`           - How the read value will be swizzled (e.g. "xyxy")
    /// *   `dst_node`              - The [`LinkingNode`] that the value will be written to
    /// *   `dst_parameter_index`   - The `src_node` index that the value will be written to
    /// *   `dst_swizzle`           - How the written value will be swizzled (e.g. "xyxy")
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// # let lib_bytecode = d3dc.compile(b"export float4 xyz1(float3 v) { return float4(v, 1.0); }", "example.hlsl", None, None, (), "lib_5_0", Compile::OptimizationLevel3, CompileEffect::None).unwrap();
    /// # let lib = d3dc.load_module(&lib_bytecode).unwrap();
    /// # let graph : FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// # let input  = graph.set_input_signature(&[ParameterDesc::new(cstr!("inPos"),  cstr!("POSITION0"),   SVT::Float, SVC::Vector, 1, 4, Interpolation::Linear,    PF::In,  0, 0, 0, 0)]).unwrap();
    /// # let output = graph.set_output_signature(&[ParameterDesc::new(cstr!("outPos"), cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0)]).unwrap();
    /// graph.pass_value_with_swizzle(&input, 0, "xyzw", &output, 0, "zyxw").unwrap();
    /// ```
    pub fn pass_value_with_swizzle(&self, src_node: &LinkingNode, src_parameter_index: i32, src_swizzle: impl TryIntoAsCStr, dst_node: &LinkingNode, dst_parameter_index: i32, dst_swizzle: impl TryIntoAsCStr) -> Result<(), Error> {
        fn_context!(d3d11::FunctionLinkingGraph::pass_value_with_swizzle => ID3D11FunctionLinkingGraph::PassValueWithSwizzle);
        let src_swizzle = fn_param_try_into!(src_swizzle)?;
        let dst_swizzle = fn_param_try_into!(dst_swizzle)?;
        fn_check_hr!(unsafe { self.0.PassValueWithSwizzle(src_node.as_raw(), src_parameter_index, src_swizzle.as_cstr(), dst_node.as_raw(), dst_parameter_index, dst_swizzle.as_cstr()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-setinputsignature)\]
    /// ID3D11FunctionLinkingGraph::SetInputSignature
    ///
    /// Sets the input signature of the function-linking-graph.
    ///
    /// ### Arguments
    /// *   `input_parameters`      - An array or slice of [`ParameterDesc`]s describing the input argument(s) of the entire [`FunctionLinkingGraph`]
    ///
    /// ### Errors
    /// *   [E::FAIL]   - if called after `call_function`
    /// *   [E::FAIL]   - if called after `set_output_signature` ?
    /// *   [E::FAIL]   - if called a second time on the same FLG
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// # let flg : FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// flg.set_input_signature(&[
    ///     ParameterDesc::new(cstr!("inputPos"),  cstr!("POSITION0"), SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("inputTex"),  cstr!("TEXCOORD0"), SVT::Float, SVC::Vector, 1, 2, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("inputNorm"), cstr!("NORMAL0"),   SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    /// ]).unwrap();
    /// # assert_eq!(E::FAIL, flg.set_input_signature(&[]).err().unwrap().kind());
    /// ```
    pub fn set_input_signature(&self, input_parameters: &[ParameterDesc<'static>]) -> Result<LinkingNode, Error> {
        fn_context!(d3d11::FunctionLinkingGraph::set_input_signature => ID3D11FunctionLinkingGraph::SetInputSignature);
        let n = fn_param_try_len32!(input_parameters)?;

        let mut node = null_mut();
        fn_check_hr!(unsafe { self.0.SetInputSignature(input_parameters.as_ptr().cast(), n, &mut node) })?;
        Ok(unsafe { LinkingNode::from_raw(node) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-setoutputsignature)\]
    /// ID3D11FunctionLinkingGraph::SetOutputSignature
    ///
    /// Sets the output signature of the function-linking-graph.
    ///
    /// ### Arguments
    /// *   `output_parameters`     - An array or slice of [`ParameterDesc`]s describing the output argument(s) of the entire [`FunctionLinkingGraph`]
    ///
    /// ### Errors
    /// *   [E::FAIL]           if called before set_input_signature
    /// *   [E::FAIL]           if called a second time on the same FLG
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let d3dc = Compiler::load_system(47).unwrap();
    /// # let flg : FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// # flg.set_input_signature(&[]).unwrap();
    /// flg.set_output_signature(&[
    ///     ParameterDesc::new(cstr!("outputPos"),  cstr!("POSITION0"),   SVT::Float, SVC::Vector, 1, 2, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("outputNorm"), cstr!("NORMAL0"),     SVT::Float, SVC::Vector, 1, 3, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("outputTex"),  cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
    /// ]).unwrap();
    /// # assert_eq!(E::FAIL, flg.set_output_signature(&[]).err().unwrap().kind());
    /// ```
    pub fn set_output_signature(&self, output_parameters: &[ParameterDesc<'static>]) -> Result<LinkingNode, Error> {
        fn_context!(d3d11::FunctionLinkingGraph::set_output_signature => ID3D11FunctionLinkingGraph::SetOutputSignature);
        let n = fn_param_try_len32!(output_parameters)?;

        let mut node = null_mut();
        fn_check_hr!(unsafe { self.0.SetOutputSignature(output_parameters.as_ptr().cast(), n, &mut node) })?;
        Ok(unsafe { LinkingNode::from_raw(node) })
    }
}

//#cpp2rust ID3D11FunctionLinkingGraph                          = d3d11::FunctionLinkingGraph
