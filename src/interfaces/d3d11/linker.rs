/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11linker)\]
/// ID3D11Linker
///
/// A linker interface is used to link a shader module.
#[derive(Clone)] #[repr(transparent)]
pub struct Linker(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11Linker>);
