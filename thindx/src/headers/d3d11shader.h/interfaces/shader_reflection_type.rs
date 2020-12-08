use crate::*;
use crate::d3d11::*;

use winapi::shared::winerror::*;
use winapi::um::d3d11shader::*;

use std::ffi::*;
use std::marker::PhantomData;
use std::ptr::NonNull;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11shaderreflectiontype)\]
/// ID3D11ShaderReflectionType
///
/// This shader-reflection interface provides access to variable type.
///
/// ### See Also
/// *   [d3d11::ShaderReflectionVariable::get_type]
#[derive(Clone)] #[repr(transparent)]
pub struct ShaderReflectionType<'r> {
    ptr:        NonNull<ID3D11ShaderReflectionType>,
    phantom:    PhantomData<&'r ShaderReflection>,
}

impl<'r> ShaderReflectionType<'r> {
    pub(crate) unsafe fn from_raw(_: impl ParentOrPhantom<'r>, ptr: *mut ID3D11ShaderReflectionType) -> Self {
        Self {
            ptr:        NonNull::new(ptr).expect("ShaderReflectionType should never be null"),
            phantom:    PhantomData,
        }
    }

    pub(crate) fn as_raw(&self) -> *mut ID3D11ShaderReflectionType { self.ptr.as_ptr() }
}

impl<'r> ShaderReflectionType<'r> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-getbaseclass)\]
    /// ID3D11ShaderReflectionType::GetBaseClass
    ///
    /// Gets an ID3D11ShaderReflectionType Interface interface containing the variable base class type.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_base_class(&self) -> ShaderReflectionType<'r> {
        let ptr = unsafe { self.ptr.as_ref().GetBaseClass() };
        unsafe { ShaderReflectionType::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-getdesc)\]
    /// ID3D11ShaderReflectionType::GetDesc
    ///
    /// Get the description of a shader-reflection-variable type.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_desc(&self) -> Result<ShaderTypeDesc<'r>, Error> {
        let mut desc = ShaderTypeDesc::default();
        let hr = unsafe { self.ptr.as_ref().GetDesc(desc.as_mut_ptr()) };
        Error::check("ID3D11ShaderReflectionType::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-getinterfacebyindex)\]
    /// ID3D11ShaderReflectionType::GetInterfaceByIndex
    ///
    /// Get an interface by index.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_interface_by_index(&self, index: u32) -> ShaderReflectionType<'r> {
        let ptr = unsafe { self.ptr.as_ref().GetInterfaceByIndex(index) };
        unsafe { ShaderReflectionType::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-getmembertypebyindex)\]
    /// ID3D11ShaderReflectionType::GetMemberTypeByIndex
    ///
    /// Get a shader-reflection-variable type by index.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_member_type_by_index(&self, index: u32) -> ShaderReflectionType<'r> {
        let ptr = unsafe { self.ptr.as_ref().GetMemberTypeByIndex(index) };
        unsafe { ShaderReflectionType::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-getmembertypebyname)\]
    /// ID3D11ShaderReflectionType::GetMemberTypeByName
    ///
    /// Get a shader-reflection-variable type by name.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_member_type_by_name(&self, name: impl TryIntoAsCStr) -> ShaderReflectionType<'r> {
        let name = name.try_into().ok();
        let name = name.map_or(cstr!("").as_cstr(), |n| n.as_cstr());
        let ptr = unsafe { self.ptr.as_ref().GetMemberTypeByName(name) };
        unsafe { ShaderReflectionType::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-getmembertypename)\]
    /// ID3D11ShaderReflectionType::GetMemberTypeName
    ///
    /// Get a shader-reflection-variable type.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_member_type_name(&self, index: u32) -> Option<&'r CStr> {
        let cstr = unsafe { self.ptr.as_ref().GetMemberTypeName(index) };
        if cstr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(cstr) })
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-getnuminterfaces)\]
    /// ID3D11ShaderReflectionType::GetNumInterfaces
    ///
    /// Gets the number of interfaces.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_num_interfaces(&self) -> u32 {
        unsafe { self.ptr.as_ref().GetNumInterfaces() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-getsubtype)\]
    /// ID3D11ShaderReflectionType::GetSubType
    ///
    /// Gets the base class of a class.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn get_sub_type(&self) -> ShaderReflectionType<'r> {
        let ptr = unsafe { self.ptr.as_ref().GetSubType() };
        unsafe { ShaderReflectionType::from_raw(self.phantom, ptr) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-implementsinterface)\]
    /// ID3D11ShaderReflectionType::ImplementsInterface
    ///
    /// Indicates whether a class type implements an interface.
    ///
    /// ### Errors
    /// *   Never?
    ///
    /// ### Returns
    /// *   `true`          - The interface is implemented
    /// *   `false`         - The interface isn't implemented
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn implements_interface(&self, base: &ShaderReflectionType) -> Result<bool, Error> {
        let hr = unsafe { self.ptr.as_ref().ImplementsInterface(base.as_raw()) };
        if hr == S_FALSE {
            Ok(false)
        } else {
            Error::check("ID3D11ShaderReflectionType::ImplementsInterface", hr)?;
            Ok(true)
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-isequal)\]
    /// ID3D11ShaderReflectionType::IsEqual
    ///
    /// Indicates whether two [ShaderReflectionType] pointers have the same underlying type.
    ///
    /// ### Errors
    /// *   Never?
    ///
    /// ### Returns
    /// *   `true`          - The interface is implemented
    /// *   `false`         - The interface isn't implemented
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn is_equal(&self, type_: &ShaderReflectionType) -> Result<bool, Error> {
        let hr = unsafe { self.ptr.as_ref().IsEqual(type_.as_raw()) };
        if hr == S_FALSE {
            Ok(false)
        } else {
            Error::check("ID3D11ShaderReflectionType::IsEqual", hr)?;
            Ok(true)
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflectiontype-isoftype)\]
    /// ID3D11ShaderReflectionType::IsOfType
    ///
    /// Indicates whether a variable is of the specified type.
    ///
    /// ### Errors
    /// *   Never?
    ///
    /// ### Returns
    /// *   `true`          - The interface is implemented
    /// *   `false`         - The interface isn't implemented
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn is_of_type(&self, type_: &ShaderReflectionType) -> Result<bool, Error> {
        let hr = unsafe { self.ptr.as_ref().IsOfType(type_.as_raw()) };
        if hr == S_FALSE {
            Ok(false)
        } else {
            Error::check("ID3D11ShaderReflectionType::IsOfType", hr)?;
            Ok(true)
        }
    }
}
