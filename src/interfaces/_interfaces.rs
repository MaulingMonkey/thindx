use crate::*;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/)\]
/// d3d11shader.h interfaces, structures, and enumerations
pub mod d3d11 {
    mod function_linking_graph;             pub use function_linking_graph::*;
    mod function_parameter_reflection;      pub use function_parameter_reflection::*;
    mod function_reflection;                pub use function_reflection::*;
    mod library_reflection;                 pub use library_reflection::*;
    mod linker;                             pub use linker::*;
    mod linking_node;                       pub use linking_node::*;
    mod module_instance;                    pub use module_instance::*;
    mod module;                             pub use module::*;
    mod shader_reflection;                  pub use shader_reflection::*;
    mod shader_reflection_constant_buffer;  pub use shader_reflection_constant_buffer::*;
    mod shader_reflection_type;             pub use shader_reflection_type::*;
    mod shader_reflection_variable;         pub use shader_reflection_variable::*;

    // structure
    mod parameter_desc;                     pub use parameter_desc::*;
}

mod blob;               pub use blob::*;
mod unknown;            pub use unknown::*;



/// ### Safety
///
/// * Assumes `$outer` is `#[repr(transparent)]`
/// * Typechecked via some `From` impls, but sufficiently malicious `Deref` impls might be able to defeat that.
macro_rules! convert {
    ( unsafe $outer:ty => $deref:ty, $winapi:ty ) => {
        convert!(unsafe $outer, $winapi);

        impl std::ops::Deref for $outer {
            type Target = $deref;
            fn deref(&self) -> &Self::Target { self.0.up_ref().into() }
        }
    };
    ( unsafe $outer:ty, $winapi:ty ) => {
        impl From<mcom::Rc<$winapi>> for $outer { fn from(value: mcom::Rc<$winapi>) -> Self { Self(value) } }
        impl From<$outer> for mcom::Rc<$winapi> { fn from(value: $outer) -> Self { value.0 } }

        impl From<&mcom::Rc<$winapi>> for &$outer { fn from(value: &mcom::Rc<$winapi>) -> Self { unsafe { std::mem::transmute(value) } } }
        impl From<&$outer> for &mcom::Rc<$winapi> { fn from(value: &$outer) -> Self { unsafe { std::mem::transmute(value) } } }

        unsafe impl Raw for $outer {
            type Raw = $winapi;

            unsafe fn from_raw(raw: *mut Self::Raw) -> Self { Self(mcom::Rc::from_raw(raw)) }
            unsafe fn from_raw_opt(raw: *mut Self::Raw) -> Option<Self> { Some(Self(mcom::Rc::from_raw_opt(raw)?)) }
            fn into_raw(self) -> *mut Self::Raw { self.0.into_raw() }
            fn as_raw(&self) -> *mut Self::Raw { self.0.as_ptr() }
        }
    };
}

// Misc
convert!(unsafe Unknown,                                            winapi::um::unknwnbase::IUnknown);
convert!(unsafe ReadOnlyBlob => Unknown,                            winapi::um::d3dcommon::ID3DBlob);

convert!(unsafe d3d11::FunctionLinkingGraph => Unknown,             winapi::um::d3d11shader::ID3D11FunctionLinkingGraph);
convert!(unsafe d3d11::LibraryReflection => Unknown,                winapi::um::d3d11shader::ID3D11LibraryReflection);
convert!(unsafe d3d11::Linker => Unknown,                           winapi::um::d3d11shader::ID3D11Linker);
convert!(unsafe d3d11::LinkingNode => Unknown,                      winapi::um::d3d11shader::ID3D11LinkingNode);
convert!(unsafe d3d11::Module => Unknown,                           winapi::um::d3d11shader::ID3D11Module);
convert!(unsafe d3d11::ModuleInstance => Unknown,                   winapi::um::d3d11shader::ID3D11ModuleInstance);
convert!(unsafe d3d11::ShaderReflection => Unknown,                 winapi::um::d3d11shader::ID3D11ShaderReflection);
