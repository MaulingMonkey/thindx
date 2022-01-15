use crate::*;
use crate::ctypes::*;
use crate::d3d11::*;

use std::ptr::*;


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11module)\]
/// ID3D11Module
///
/// A module interface creates an instance of a module that is used for resource rebinding.
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// let d3dc = d3d::Compiler::load_system(47).unwrap();
/// let lib = d3dc.compile_from_file(
///     "test/data/library.hlsl", None, None, (), "lib_5_0",
///     d3d::Compile::OptimizationLevel3, d3d::CompileEffect::None
/// ).unwrap();
/// let module : d3d11::Module = d3dc.load_module(&lib).unwrap();
/// ```
///
/// ### See Also
/// *   [d3d::Compiler::load_module] to create [Module]s
/// *   [d3d11::FunctionLinkingGraph] for examples
/// *   [examples::d3dcompiler_03_link]
#[derive(Clone)] #[repr(transparent)]
pub struct Module(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11Module>);

convert!(unsafe Module => Unknown, winapi::um::d3d11shader::ID3D11Module);

impl Module {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11module-createinstance)\]
    /// ID3D11Module::CreateInstance
    ///
    /// Initializes an instance of a shader module that is used for resource rebinding.
    ///
    /// ### Arguments
    /// *   `namespace` - The name of a shader module to initialize.  This can be [None] if you don't want to specify a name for the module.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let lib = d3dc.compile_from_file("test/data/library.hlsl", None, None, (), "lib_5_0", d3d::Compile::OptimizationLevel3, d3d::CompileEffect::None).unwrap();
    /// # let module : d3d11::Module = d3dc.load_module(&lib).unwrap();
    /// #
    /// // no namespace
    /// let ns : Option<&str> = None;
    /// let _ : d3d11::ModuleInstance = module.create_instance(()).unwrap();
    /// let _ : d3d11::ModuleInstance = module.create_instance(ns).unwrap();
    ///
    /// // global namespace
    /// let _ : d3d11::ModuleInstance = module.create_instance("").unwrap();
    /// let _ : d3d11::ModuleInstance = module.create_instance(cstr!("")).unwrap();
    ///
    /// // specific namespace
    /// let _ : d3d11::ModuleInstance = module.create_instance("namespace").unwrap();
    /// let _ : d3d11::ModuleInstance = module.create_instance(cstr!("namespace")).unwrap();
    /// ```
    pub fn create_instance(&self, namespace: impl TryIntoAsOptCStr) -> Result<ModuleInstance, MethodError> {
        let namespace = namespace.try_into().map_err(|e| MethodError::new("ID3D11Module::CreateInstance", e))?;
        let namespace = namespace.as_opt_cstr();

        let mut instance = null_mut();
        let hr = unsafe { self.0.CreateInstance(namespace, &mut instance) };
        MethodError::check("ID3D11Module::CreateInstance", hr)?;
        Ok(unsafe { ModuleInstance::from_raw(instance) })
    }
}
