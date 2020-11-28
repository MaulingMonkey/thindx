/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflection)\]
/// ID3D11ShaderReflection
///
/// A shader-reflection interface accesses shader information.
#[derive(Clone)] #[repr(transparent)]
pub struct ShaderReflection(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11ShaderReflection>);
