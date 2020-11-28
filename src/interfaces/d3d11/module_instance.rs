/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11moduleinstance)\]
/// ID3D11ModuleInstance
///
/// A module-instance interface is used for resource rebinding.
#[derive(Clone)] #[repr(transparent)]
pub struct ModuleInstance(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11ModuleInstance>);
