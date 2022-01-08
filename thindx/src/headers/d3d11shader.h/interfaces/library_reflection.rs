use crate::*;
use crate::d3d11::*;

use std::iter::Iterator;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11libraryreflection)\]
/// ID3D11LibraryReflection
///
/// A library-reflection interface accesses library info.
///
/// ### See Also
/// *   [d3d::Compiler::reflect_library]
/// *   [d3d::Compiler::reflect_library_11]
#[derive(Clone)] #[repr(transparent)]
pub struct LibraryReflection(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11LibraryReflection>);

convert!(unsafe LibraryReflection => Unknown, winapi::um::d3d11shader::ID3D11LibraryReflection);

/// Direct API calls
impl LibraryReflection {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11libraryreflection-getdesc)\]
    /// ID3D11LibraryReflection::GetDesc
    ///
    /// Get a library descriptor structure for the library reflection.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::new(47).unwrap();
    /// let shader = d3dc.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap();
    ///
    /// let r : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    /// let desc : d3d11::LibraryDesc = r.get_desc().unwrap();
    /// println!("{:#?}", desc);
    /// assert!(desc.function_count > 0);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// LibraryDesc {
    ///     creator: Some(
    ///         "Microsoft (R) HLSL Shader Compiler 10.1",
    ///     ),
    ///     flags: Compile::None,
    ///     function_count: 1,
    /// }
    /// ```
    pub fn get_desc(&self) -> Result<LibraryDesc, MethodErrorBlob> {
        let mut desc = LibraryDesc::default();
        let hr = unsafe { self.0.GetDesc(desc.as_mut_ptr()) };
        MethodErrorBlob::check("ID3D11LibraryReflection::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11libraryreflection-getfunctionbyindex)\]
    /// ID3D11LibraryReflection::GetFunctionByIndex
    ///
    /// Gets the function reflector.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::new(47).unwrap();
    /// let shader = d3dc.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap();
    ///
    /// let r : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    ///
    /// for i in 0..r.get_desc().unwrap().function_count {
    ///     let f : d3d11::FunctionReflection = r.get_function_by_index(i);
    ///     let desc = f.get_desc().unwrap();
    ///     println!("{:#?}", desc);
    /// }
    /// ```
    ///
    /// ### Output
    /// ```text
    /// FunctionDesc {
    ///     version: 4293918800,
    ///     creator: Some(
    ///         "Microsoft (R) HLSL Shader Compiler 10.1",
    ///     ),
    ///     flags: Compile::{Debug|NoPreshader},
    ///     constant_buffers: 0,
    ///     bound_resources: 0,
    ///     instruction_count: 3,
    ///     ...,
    ///     name: Some(
    ///         "xyz1",
    ///     ),
    ///     function_parameter_count: 1,
    ///     has_return: true,
    ///     has_10_level_9_vertex_shader: false,
    ///     has_10_level_9_pixel_shader: false,
    /// }
    /// ```
    //#allow_missing_argument_docs
    pub fn get_function_by_index(&self, function_index: u32) -> FunctionReflection {
        let ptr = unsafe { self.0.GetFunctionByIndex(function_index as i32) };
        unsafe { FunctionReflection::from_raw(self, ptr) }
    }
}

/// Convenience utility methods
impl LibraryReflection {
    /// Enumerate library functions
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let d3dc = Compiler::new(47).unwrap();
    /// let shader = d3dc.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap();
    ///
    /// let r : d3d11::LibraryReflection = d3dc.reflect_library(&shader).unwrap();
    ///
    /// for f in r.functions().unwrap() {
    ///     let desc = f.get_desc().unwrap();
    ///     println!("{:#?}", desc);
    /// }
    /// ```
    ///
    /// ### Output
    /// ```text
    /// FunctionDesc {
    ///     version: 4293918800,
    ///     creator: Some(
    ///         "Microsoft (R) HLSL Shader Compiler 10.1",
    ///     ),
    ///     flags: Compile::{Debug|NoPreshader},
    ///     constant_buffers: 0,
    ///     bound_resources: 0,
    ///     instruction_count: 3,
    ///     ...,
    ///     name: Some(
    ///         "xyz1",
    ///     ),
    ///     function_parameter_count: 1,
    ///     has_return: true,
    ///     has_10_level_9_vertex_shader: false,
    ///     has_10_level_9_pixel_shader: false,
    /// }
    /// ```
    pub fn functions<'lr>(&'lr self) -> Result<impl 'lr + Iterator<Item = FunctionReflection<'lr>>, MethodErrorBlob> {
        Ok(LibraryReflectionFunctionsIter {
            desc:               self.get_desc()?,
            library_reflection: self,
            index:              0,
        })
    }
}

struct LibraryReflectionFunctionsIter<'lr> {
    library_reflection: &'lr LibraryReflection,
    desc:               LibraryDesc<'lr>,
    index:              u32,
}

impl<'lr> Iterator for LibraryReflectionFunctionsIter<'lr> {
    type Item = FunctionReflection<'lr>;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        if i < self.desc.function_count {
            self.index = i + 1;
            Some(self.library_reflection.get_function_by_index(i))
        } else {
            None
        }
    }
}
