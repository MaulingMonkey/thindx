use crate::*;
use crate::d3d::*;
use crate::d3d11::*;

use std::convert::TryInto;
use std::os::raw::c_char;
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
/// ### See Also
/// * [d3d::Compiler::create_function_linking_graph]
///
/// ### Example
/// ```rust
/// use thindx::{*, d3d::*, d3d11::*};
/// let compiler = Compiler::new(47).unwrap();
///
///
/// // 1. Create a library of shader functions
///
/// let lib_source = br##"
///     export float4 xyz1(float3 v) { return float4(v, 1.0); }
/// "##;
///
/// let lib_bytecode = compiler.compile(
///     lib_source, "example.hlsl", None, None, (), "lib_5_0",
///     Compile::OptimizationLevel3, CompileEffect::None
/// ).unwrap();
///
/// let lib = compiler.load_module(&lib_bytecode).unwrap();
///
///
/// // 2. Use FunctionLinkingGraph to create a shader.  Note that the fn call order
/// // here is brittle, reordering many of the calls here will cause E::FAIL errors.
///
/// let graph : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
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
/// let linker = compiler.create_linker().unwrap();
/// linker.use_library(&lib_inst).unwrap();
/// linker.link(&graph_inst, "main", "vs_5_0", None).unwrap();
/// ```
///
/// ### Outputs
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
    /// # let compiler = Compiler::new(47).unwrap();
    /// # let lib_bytecode = compiler.compile(b"export float4 xyz1(float3 v) { return float4(v, 1.0); }", "example.hlsl", None, None, (), "lib_5_0", Compile::OptimizationLevel3, CompileEffect::None).unwrap();
    /// # let lib = compiler.load_module(&lib_bytecode).unwrap();
    /// # let graph : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// # let input  = graph.set_input_signature(&[]).unwrap();
    /// let xyz1 = graph.call_function("example_namespace", &lib, "xyz1").unwrap();
    /// let xyz1 = graph.call_function("",   &lib, "xyz1").unwrap();
    /// let xyz1 = graph.call_function(None, &lib, "xyz1").unwrap();
    ///
    /// assert_eq!(E::FAIL, graph.call_function("", &lib, "nonexistant").err().unwrap().kind());
    /// ```
    pub fn call_function<'s>(
        &self,
        module_instance_namespace:      impl Into<Option<&'s str>>,
        module_with_function_prototype: &Module,
        function_name:                  &'s str,
    ) -> Result<LinkingNode, Error> {
        let ns      = module_instance_namespace.into().map(|s| s.bytes().chain(Some(0)).collect::<Vec<_>>());
        let name    = function_name.bytes().chain(Some(0)).collect::<Vec<_>>();
        let module  = module_with_function_prototype.as_raw();

        let ns = ns.as_ref().map_or(null(), |s| s.as_ptr().cast());
        let name = name.as_ptr().cast();

        let mut node = null_mut();
        let hr = unsafe { self.0.CallFunction(ns, module, name, &mut node) };
        Error::check("ID3D11FunctionLinkingGraph::CallFunction", hr)?;
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
    pub fn create_module_instance(&self) -> Result<(ModuleInstance, Option<ReadOnlyBlob>), Error> {
        // TODO: named tuple return?  better error type that can carry the blob?
        let mut module = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { self.0.CreateModuleInstance(&mut module, &mut errors) };
        unsafe { Error::check_blob("ID3D11FunctionLinkingGraph::CreateModuleInstance", hr, errors)? };
        let module = unsafe { ModuleInstance::from_raw(module) };
        let errors = unsafe { ReadOnlyBlob::from_raw_opt(errors) };
        Ok((module, errors))
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
    /// # let compiler = Compiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// let hlsl = flg.generate_hlsl(()).unwrap();
    /// println!("{}", hlsl.to_utf8_lossy());
    /// ```
    ///
    /// ### Outputs
    /// ```hlsl
    /// void main()
    /// {
    /// }
    /// ```
    pub fn generate_hlsl(&self, flags: ()) -> Result<TextBlob, Error> {
        let _ = flags; let flags = 0;

        let mut blob = null_mut();
        let hr = unsafe { self.0.GenerateHlsl(flags, &mut blob) };
        Error::check("ID3D11FunctionLinkingGraph::GenerateHlsl", hr)?;
        Ok(TextBlob::new(unsafe { ReadOnlyBlob::from_raw(blob) }))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-getlasterror)\]
    /// ID3D11FunctionLinkingGraph::GetLastError
    ///
    /// Gets the error from the last function call of the function-linking-graph.
    ///
    /// ### Returns
    /// *   Ok(Some([ReadOnlyBlob]))    - The errors in question
    /// *   Ok(None)                    - There were no errors
    /// *   Err([ErrorKind])            - Ironically, there was an error in attempting to acquire the errors.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let compiler = Compiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// let errors : Option<ReadOnlyBlob> = flg.get_last_error().unwrap();
    /// assert!(errors.is_none(), "No errors were reported by flg");
    /// ```
    pub fn get_last_error(&self) -> Result<Option<ReadOnlyBlob>, Error> {
        let mut errors = null_mut();
        let hr = unsafe { self.0.GetLastError(&mut errors) };
        Error::check("ID3D11FunctionLinkingGraph::GetLastError", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw_opt(errors) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-passvalue)\]
    /// ID3D11FunctionLinkingGraph::PassValue
    ///
    /// Passes a value from a source linking node to a destination linking node.
    ///
    /// ### Errors
    /// -   [E::FAIL]           - if multiple values are passed to the same destination
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let compiler = Compiler::new(47).unwrap();
    /// # let lib_bytecode = compiler.compile(b"export float4 xyz1(float3 v) { return float4(v, 1.0); }", "example.hlsl", None, None, (), "lib_5_0", Compile::OptimizationLevel3, CompileEffect::None).unwrap();
    /// # let lib = compiler.load_module(&lib_bytecode).unwrap();
    /// # let graph : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// # let input  = graph.set_input_signature(&[ParameterDesc::new(cstr!("inPos"),  cstr!("POSITION0"),   SVT::Float, SVC::Vector, 1, 4, Interpolation::Linear,    PF::In,  0, 0, 0, 0)]).unwrap();
    /// # let output = graph.set_output_signature(&[ParameterDesc::new(cstr!("outPos"), cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0)]).unwrap();
    /// graph.pass_value(&input, 0, &output, 0).unwrap();
    /// ```
    pub fn pass_value(&self, src_node: &LinkingNode, src_parameter_index: i32, dst_node: &LinkingNode, dst_parameter_index: i32) -> Result<(), Error> {
        let hr = unsafe { self.0.PassValue(src_node.as_raw(), src_parameter_index, dst_node.as_raw(), dst_parameter_index) };
        Error::check("ID3D11FunctionLinkingGraph::PassValue", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-passvaluewithswizzle)\]
    /// ID3D11FunctionLinkingGraph::PassValueWithSwizzle
    ///
    /// Passes a value with swizzle from a source linking node to a destination linking node.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let compiler = Compiler::new(47).unwrap();
    /// # let lib_bytecode = compiler.compile(b"export float4 xyz1(float3 v) { return float4(v, 1.0); }", "example.hlsl", None, None, (), "lib_5_0", Compile::OptimizationLevel3, CompileEffect::None).unwrap();
    /// # let lib = compiler.load_module(&lib_bytecode).unwrap();
    /// # let graph : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// # let input  = graph.set_input_signature(&[ParameterDesc::new(cstr!("inPos"),  cstr!("POSITION0"),   SVT::Float, SVC::Vector, 1, 4, Interpolation::Linear,    PF::In,  0, 0, 0, 0)]).unwrap();
    /// # let output = graph.set_output_signature(&[ParameterDesc::new(cstr!("outPos"), cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0)]).unwrap();
    /// graph.pass_value_with_swizzle(&input, 0, "xyzw", &output, 0, "zyxw").unwrap();
    /// ```
    pub fn pass_value_with_swizzle(&self, src_node: &LinkingNode, src_parameter_index: i32, src_swizzle: &str, dst_node: &LinkingNode, dst_parameter_index: i32, dst_swizzle: &str) -> Result<(), Error> {
        let src_swizzle = src_swizzle.bytes().map(|b| b as c_char).chain(Some(0)).collect::<Vec<_>>();
        let dst_swizzle = dst_swizzle.bytes().map(|b| b as c_char).chain(Some(0)).collect::<Vec<_>>();
        let hr = unsafe { self.0.PassValueWithSwizzle(src_node.as_raw(), src_parameter_index, src_swizzle.as_ptr(), dst_node.as_raw(), dst_parameter_index, dst_swizzle.as_ptr()) };
        Error::check("ID3D11FunctionLinkingGraph::PassValueWithSwizzle", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-setinputsignature)\]
    /// ID3D11FunctionLinkingGraph::SetInputSignature
    ///
    /// Sets the input signature of the function-linking-graph.
    ///
    /// ### Errors
    /// *   [E::FAIL]   - if called after `call_function`
    /// *   [E::FAIL]   - if called after `set_output_signature` ?
    /// *   [E::FAIL]   - if called a second time on the same FLG
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let compiler = Compiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// flg.set_input_signature(&[
    ///     ParameterDesc::new(cstr!("inputPos"),  cstr!("POSITION0"), SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("inputTex"),  cstr!("TEXCOORD0"), SVT::Float, SVC::Vector, 1, 2, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("inputNorm"), cstr!("NORMAL0"),   SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    /// ]).unwrap();
    /// # assert_eq!(E::FAIL, flg.set_input_signature(&[]).err().unwrap().kind());
    /// ```
    pub fn set_input_signature(&self, input_parameters: &[ParameterDesc<'static>]) -> Result<LinkingNode, Error> {
        let n = input_parameters.len().try_into().map_err(|_| Error::new("ID3D11FunctionLinkingGraph::SetInputSignature", THINERR::SLICE_TOO_LARGE))?;

        let mut node = null_mut();
        let hr = unsafe { self.0.SetInputSignature(input_parameters.as_ptr().cast(), n, &mut node) };
        Error::check("ID3D11FunctionLinkingGraph::SetInputSignature", hr)?;
        Ok(unsafe { LinkingNode::from_raw(node) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-setoutputsignature)\]
    /// ID3D11FunctionLinkingGraph::SetOutputSignature
    ///
    /// Sets the output signature of the function-linking-graph.
    ///
    /// ### Errors
    /// *   [E::FAIL]           if called before set_input_signature
    /// *   [E::FAIL]           if called a second time on the same FLG
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*, d3d11::*};
    /// # let compiler = Compiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// # flg.set_input_signature(&[]).unwrap();
    /// flg.set_output_signature(&[
    ///     ParameterDesc::new(cstr!("outputPos"),  cstr!("POSITION0"),   SVT::Float, SVC::Vector, 1, 2, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("outputNorm"), cstr!("NORMAL0"),     SVT::Float, SVC::Vector, 1, 3, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("outputTex"),  cstr!("SV_POSITION"), SVT::Float, SVC::Vector, 1, 4, Interpolation::Undefined, PF::Out, 0, 0, 0, 0),
    /// ]).unwrap();
    /// # assert_eq!(E::FAIL, flg.set_output_signature(&[]).err().unwrap().kind());
    /// ```
    pub fn set_output_signature(&self, output_parameters: &[ParameterDesc<'static>]) -> Result<LinkingNode, Error> {
        let n = output_parameters.len().try_into().map_err(|_| Error::new("ID3D11FunctionLinkingGraph::SetOutputSignature", THINERR::SLICE_TOO_LARGE))?;

        let mut node = null_mut();
        let hr = unsafe { self.0.SetOutputSignature(output_parameters.as_ptr().cast(), n, &mut node) };
        Error::check("ID3D11FunctionLinkingGraph::SetOutputSignature", hr)?;
        Ok(unsafe { LinkingNode::from_raw(node) })
    }
}
