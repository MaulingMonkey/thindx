/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11module)\]
/// ID3D11Module
///
/// A module interface creates an instance of a module that is used for resource rebinding.
#[derive(Clone)] #[repr(transparent)]
pub struct Module(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11Module>);
