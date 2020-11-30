use crate::*;
use crate::d3d11::*;

use std::convert::TryInto;
use std::ptr::*;



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
/// * [D3DCompiler::create_function_linking_graph]
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
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; use d3d11::*;
    /// # let compiler = D3DCompiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// // TODO
    /// // flg.call_function().unwrap();
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
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; use d3d11::*;
    /// # let compiler = D3DCompiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// // Nothing created yet
    /// let error : Error = flg.create_module_instance().err().unwrap();
    /// assert_eq!(E::FAIL, error.kind());
    /// println!("{}", error);
    ///
    /// // TODO: successful example
    /// ```
    ///
    /// ### Outputs
    /// ```text
    /// ID3D11FunctionLinkingGraph::CreateModuleInstance failed (ErrorKind::FAIL)
    /// error X9021: ID3D11FunctionLinkingGraph::CreateModuleInstance: FLG has no nodes
    /// ```
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
    /// # use thin3dcompiler::*; use d3d11::*;
    /// # let compiler = D3DCompiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// let hlsl = flg.generate_hlsl(()).unwrap();
    /// ```
    pub fn generate_hlsl(&self, flags: ()) -> Result<ReadOnlyBlob, Error> {
        let _ = flags; let flags = 0;

        let mut blob = null_mut();
        let hr = unsafe { self.0.GenerateHlsl(flags, &mut blob) };
        Error::check("ID3D11FunctionLinkingGraph::GenerateHlsl", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(blob) })
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
    /// # use thin3dcompiler::*; use d3d11::*;
    /// # let compiler = D3DCompiler::new(47).unwrap();
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
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; use d3d11::*;
    /// # let compiler = D3DCompiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// // TODO
    /// // flg.pass_value().unwrap();
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
    /// # use thin3dcompiler::*; use d3d11::*;
    /// # let compiler = D3DCompiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// // TODO
    /// // flg.pass_value_with_swizzle().unwrap();
    /// ```
    pub fn pass_value_with_swizzle(&self, src_node: &LinkingNode, src_parameter_index: i32, src_swizzle: &str, dst_node: &LinkingNode, dst_parameter_index: i32, dst_swizzle: &str) -> Result<(), Error> {
        let src_swizzle = src_swizzle.bytes().chain(Some(0)).collect::<Vec<_>>();
        let dst_swizzle = dst_swizzle.bytes().chain(Some(0)).collect::<Vec<_>>();
        let hr = unsafe { self.0.PassValueWithSwizzle(src_node.as_raw(), src_parameter_index, src_swizzle.as_ptr().cast(), dst_node.as_raw(), dst_parameter_index, dst_swizzle.as_ptr().cast()) };
        Error::check("ID3D11FunctionLinkingGraph::PassValueWithSwizzle", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11functionlinkinggraph-setinputsignature)\]
    /// ID3D11FunctionLinkingGraph::SetInputSignature
    ///
    /// Sets the input signature of the function-linking-graph.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; use d3d11::*;
    /// # let compiler = D3DCompiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// // TODO
    /// // flg.set_input_signature().unwrap();
    /// ```
    pub fn set_input_signature(&self, input_parameters: &[ParameterDesc]) -> Result<LinkingNode, Error> {
        let n = input_parameters.len().try_into().map_err(|_| Error::new("ID3D11FunctionLinkingGraph::SetInputSignature", ErrorKind::SLICE_TOO_LARGE))?;

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
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; use d3d11::*;
    /// # let compiler = D3DCompiler::new(47).unwrap();
    /// # let flg : FunctionLinkingGraph = compiler.create_function_linking_graph(None).unwrap();
    /// // TODO
    /// // flg.set_output_signature().unwrap();
    /// ```
    pub fn set_output_signature(&self, output_parameters: &[ParameterDesc]) -> Result<LinkingNode, Error> {
        let n = output_parameters.len().try_into().map_err(|_| Error::new("ID3D11FunctionLinkingGraph::SetOutputSignature", ErrorKind::SLICE_TOO_LARGE))?;

        let mut node = null_mut();
        let hr = unsafe { self.0.SetOutputSignature(output_parameters.as_ptr().cast(), n, &mut node) };
        Error::check("ID3D11FunctionLinkingGraph::SetOutputSignature", hr)?;
        Ok(unsafe { LinkingNode::from_raw(node) })
    }

    // TODO: safe alternatives to set_{input,output}_signature
}
