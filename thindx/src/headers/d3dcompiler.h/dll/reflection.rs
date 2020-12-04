use crate::*;
use crate::d3d::*;

use winapi::Interface;

use std::ptr::*;



/// <h1 id="reflection" class="section-header"><a href="#reflection">Bytecode Reflection</a></h1>
impl Compiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreflect)\]
    /// D3DReflect
    ///
    /// Gets a pointer to a reflection interface.
    ///
    /// ### Arguments
    /// *   `src_data`  - A compiled HLSL shader.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [THINERR::MISSING_DLL_EXPORT]    - `d3dcompiler_4?.dll` and earlier
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDARG]               - On invalid `I`
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDCALL]              - On invalid `src_data`
    /// *   Ok(`()`)
    ///
    /// ### See Also
    /// *   [d3d11::ShaderReflection] for a more complete example
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let compiler = Compiler::new(47).unwrap();
    /// let shader = compiler.compile_from_file(
    ///     r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap().shader;
    ///
    /// // Should succeed:
    /// let r = compiler.reflect::<d3d11::ShaderReflection>(shader.get_buffer()).unwrap();
    ///
    /// // Invalid interface:
    /// let r = compiler.reflect::<Unknown>(shader.get_buffer());
    /// assert_eq!(Some(E::INVALIDARG), r.err().map(|e| e.kind()));
    ///
    /// // Invalid `src_data`:
    /// let r = compiler.reflect::<d3d11::ShaderReflection>(&[]);
    /// assert_eq!(Some(D3DERR::INVALIDCALL), r.err().map(|e| e.kind()));
    /// ```
    #[requires(d3dcompiler=40)]
    pub fn reflect<I: Raw>(&self, src_data: &[u8]) -> Result<I, Error> where I::Raw : Interface {
        let f = self.D3DReflect.ok_or(Error::new("D3DReflect", THINERR::MISSING_DLL_EXPORT))?;

        let mut reflector = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &I::Raw::uuidof(), &mut reflector) };
        Error::check("D3DReflect", hr)?;
        Ok(unsafe { I::from_raw(reflector.cast()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreflect)\]
    /// D3DReflect
    ///
    /// Gets a pointer to a reflection interface.
    ///
    /// ### Arguments
    /// *   `src_data`  - A compiled HLSL shader.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [THINERR::MISSING_DLL_EXPORT]    - `d3dcompiler_4?.dll` and earlier
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDARG]               - On invalid `I`
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDCALL]              - On invalid `src_data`
    /// *   Ok(`()`)
    ///
    /// ### See Also
    /// *   [d3d11::ShaderReflection] for a more complete example
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let compiler = Compiler::new(47).unwrap();
    /// let shader = compiler.compile_from_file(
    ///     r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap().shader;
    ///
    /// // Should succeed:
    /// let r = compiler.reflect11(&shader).unwrap();
    ///
    /// // Invalid `src_data`:
    /// let r = compiler.reflect11(&[]);
    /// assert_eq!(Some(D3DERR::INVALIDCALL), r.err().map(|e| e.kind()));
    /// ```
    #[requires(d3dcompiler=40)]
    pub fn reflect11(&self, src_data: impl AsRef<[u8]>) -> Result<d3d11::ShaderReflection, Error> {
        self.reflect(src_data.as_ref())
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreflectlibrary)\]
    /// D3DReflectLibrary
    ///
    /// Creates a library-reflection interface from source data that contains an HLSL library of functions.
    ///
    /// > **Note:** This function is part of the HLSL shader linking technology that you can use on all Direct3D 11
    /// > platforms to create precompiled HLSL functions, package them into libraries, and link them into full shaders
    /// > at run time.
    ///
    /// ### Arguments
    /// *   `src_data`  - An HLSL library of functions.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [THINERR::MISSING_DLL_EXPORT]    - `d3dcompiler_4?.dll` and earlier
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDCALL]              - On invalid `src_data`
    /// *   Ok(`()`)
    ///
    /// ### See Also
    /// *   [d3d11::LibraryReflection] for a more complete example
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let compiler = Compiler::new(47).unwrap();
    /// let shader = compiler.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap().shader;
    ///
    /// // Should succeed:
    /// let r = compiler.reflect_library::<d3d11::LibraryReflection>(shader.get_buffer()).unwrap();
    ///
    /// // Invalid interface:
    /// let r = compiler.reflect_library::<Unknown>(shader.get_buffer());
    /// assert_eq!(Some(E::INVALIDARG), r.err().map(|e| e.kind()));
    ///
    /// // Invalid `src_data`:
    /// let r = compiler.reflect_library::<d3d11::LibraryReflection>(&[]);
    /// assert_eq!(Some(D3DERR::INVALIDCALL), r.err().map(|e| e.kind()));
    /// ```
    //#[requires(d3dcompiler=47)] // ?
    pub fn reflect_library<C: Raw>(&self, src_data: &[u8]) -> Result<C, Error> where C::Raw : Interface {
        let f = self.D3DReflectLibrary.ok_or(Error::new("D3DReflectLibrary", THINERR::MISSING_DLL_EXPORT))?;

        let mut reflector = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &C::Raw::uuidof(), &mut reflector) };
        Error::check("D3DReflectLibrary", hr)?;
        Ok(unsafe { C::from_raw(reflector.cast()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreflectlibrary)\]
    /// D3DReflectLibrary
    ///
    /// Creates a library-reflection interface from source data that contains an HLSL library of functions.
    ///
    /// > **Note:** This function is part of the HLSL shader linking technology that you can use on all Direct3D 11
    /// > platforms to create precompiled HLSL functions, package them into libraries, and link them into full shaders
    /// > at run time.
    ///
    /// ### Arguments
    /// *   `src_data`  - An HLSL library of functions.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [THINERR::MISSING_DLL_EXPORT]    - `d3dcompiler_4?.dll` and earlier
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDCALL]              - On invalid `src_data`
    /// *   Ok(`()`)
    ///
    /// ### See Also
    /// *   [d3d11::LibraryReflection] for a more complete example
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::{*, d3d::*}; let compiler = Compiler::new(47).unwrap();
    /// let shader = compiler.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     Compile::Debug, CompileEffect::None
    /// ).unwrap().shader;
    ///
    /// // Should succeed:
    /// let r = compiler.reflect_library_11(&shader).unwrap();
    ///
    /// // Invalid `src_data`:
    /// let r = compiler.reflect_library_11(&[]);
    /// assert_eq!(Some(D3DERR::INVALIDCALL), r.err().map(|e| e.kind()));
    /// ```
    //#[requires(d3dcompiler=47)] // ?
    pub fn reflect_library_11(&self, src_data: impl AsRef<[u8]>) -> Result<d3d11::LibraryReflection, Error> {
        self.reflect_library(src_data.as_ref())
    }
}

#[test] fn library_reflection() {
    let d3dc = Compiler::new(47).unwrap();

    let shader = d3dc.compile_from_file(
        r"test\data\library.hlsl", None, None, (), "lib_5_0",
        Compile::Debug, CompileEffect::None
    ).unwrap().shader;

    let r : d3d11::LibraryReflection = d3dc.reflect_library(shader.get_buffer()).unwrap();
    let desc = r.get_desc().unwrap();
    assert!(desc.function_count > 0);

    let mut found_xyz1 = false;
    for i in 0..desc.function_count {
        let f : d3d11::FunctionReflection = r.get_function_by_index(i);
        let desc = f.get_desc().unwrap();
        if desc.name.to_bytes() == b"xyz1" {
            found_xyz1 = true;
            assert_eq!(desc.function_parameter_count,   1   );
            assert_eq!(desc.has_return,                 true);
        }
    }
    assert!(found_xyz1);

    // Reflection likes to use the null object pattern for functions returning interface pointers.
    // Most out-of-bounds accesses result in a reused "null" object that E::FAIL on most calls.

    let invalid1 = r.get_function_by_index(desc.function_count);
    let invalid2 = r.get_function_by_index(!0);
    assert_eq!(invalid1.as_raw(), invalid2.as_raw()); // same object

    let valid = r.get_function_by_index(0);
    assert_ne!(valid.as_raw(), invalid1.as_raw()); // valid is a different object
    assert_ne!(valid.as_raw(), invalid2.as_raw()); // valid is a different object

    for invalid_index in [
        1000, 10000,
        !0/2-100, !0/2-10, !0/2-4, !0/2, !0/2+1, !0/2+4, !0/2+100,
        !0-100, !0-10, !0-4, !0-1, !0,
    ].iter().copied() {
        println!("testing get_function_by_index({})", invalid_index);
        let f = r.get_function_by_index(invalid_index);
        assert_eq!(Some(E::FAIL), f.get_desc().err().map(|e| e.kind()));

        let cb = f.get_constant_buffer_by_name("SomeNonexistantCBuffer");
        assert_eq!(Some(E::FAIL), cb.get_desc().err().map(|e| e.kind()));

        for invalid_index in [
            0, 1, 4, 10, 100, 1000, 100000,  100000,
            !0/2-100, !0/2-10, !0/2-4, !0/2, !0/2+1, !0/2+4, !0/2+100,
            !0-100, !0-10, !0-4, !0-1, !0,
        ].iter().copied() {
            let cb = f.get_constant_buffer_by_index(invalid_index);
            assert_eq!(Some(E::FAIL), cb.get_desc().err().map(|e| e.kind()));

            let param = f.get_function_parameter(invalid_index as _);
            assert_eq!(Some(E::FAIL), param.get_desc().err().map(|e| e.kind()));
        }
    }

    // TODO: full test coverage of all methods
}
