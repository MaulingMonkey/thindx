use crate::*;
use crate::d3d11::*;

use std::ffi::CString;
use std::ptr::*;


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11module)\]
/// ID3D11Module
///
/// A module interface creates an instance of a module that is used for resource rebinding.
#[derive(Clone)] #[repr(transparent)]
pub struct Module(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11Module>);

impl Module {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11module-createinstance)\]
    /// ID3D11Module::CreateInstance
    ///
    /// Initializes an instance of a shader module that is used for resource rebinding.
    ///
    /// ### Arguments
    /// *   `namespace` - The name of a shader module to initialize.  This can be [None] if you don't want to specify a name for the module.
    ///
    /// ### Examples
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn create_instance(&self, namespace: impl Into<Option<CString>>) -> Result<ModuleInstance, Error> {
        let namespace = namespace.into();
        let namespace = namespace.as_ref().map_or(null(), |s| s.as_ptr());

        let mut instance = null_mut();
        let hr = unsafe { self.0.CreateInstance(namespace, &mut instance) };
        Error::check("ID3D11Module::CreateInstance", hr)?;
        Ok(unsafe { ModuleInstance::from_raw(instance) })
    }
}
