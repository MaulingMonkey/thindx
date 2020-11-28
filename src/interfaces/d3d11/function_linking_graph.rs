/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11functionlinkinggraph)\]
/// ID3D11FunctionLinkingGraph
///
/// A function-linking-graph interface is used for constructing shaders that consist of a
/// sequence of precompiled function calls that pass values to each other.
#[derive(Clone)] #[repr(transparent)]
pub struct FunctionLinkingGraph(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11FunctionLinkingGraph>);
