use crate::*;

use winapi::Interface;

use std::ptr::*;



/// <h1 id="reflection" class="section-header"><a href="#reflection">Bytecode Reflection</a></h1>
impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dreflect)\]
    /// D3DReflect
    ///
    /// Gets a pointer to a reflection interface.
    ///
    /// ### Arguments
    /// *   `src_data`  - A compiled HLSL shader.
    ///
    /// ### Returns
    /// *   Err(`e`) where `e.kind()` == [ErrorKind::MISSING_DLL_EXPORT]    - `d3dcompiler_4?.dll` and earlier
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDARG]               - On invalid `I`
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDCALL]              - On invalid `src_data`
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
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
    /// assert_eq!(Some(D3DERR::INVALIDARG), r.err().map(|e| e.kind()));
    ///
    /// // Invalid `src_data`:
    /// let r = compiler.reflect::<d3d11::ShaderReflection>(&[]);
    /// assert_eq!(Some(D3DERR::INVALIDCALL), r.err().map(|e| e.kind()));
    /// ```
    ///
    /// <div class="note"><b>Note:</b>  The D3dcompiler_40.dll or later version of the file contains the D3DReflect compiler function.</div>
    #[cfg_attr(not(d3dcompiler="40"), deprecated(note = "D3DCompiler::reflect wasn't added until d3dcompiler_40.dll"))]
    pub fn reflect<I: Raw>(&self, src_data: &[u8]) -> Result<I, Error> where I::Raw : Interface {
        let f = self.D3DReflect.ok_or(Error::new("D3DReflect", ErrorKind::MISSING_DLL_EXPORT))?;

        let mut reflector = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &I::Raw::uuidof(), &mut reflector) };
        Error::check("D3DReflect", hr)?;
        Ok(unsafe { I::from_raw(reflector.cast()) })
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
    /// *   Err(`e`) where `e.kind()` == [ErrorKind::MISSING_DLL_EXPORT]    - `d3dcompiler_4?.dll` and earlier
    /// *   Err(`e`) where `e.kind()` == [D3DERR::INVALIDCALL]              - On invalid `src_data`
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust,no_run
    /// # use thin3dcompiler::*; let compiler = D3DCompiler::new(47).unwrap();
    /// // TODO: figure out what reflect_library actually takes and demo that.
    ///
    /// // Invalid `src_data`:
    /// let r = compiler.reflect_library::<d3d11::LibraryReflection>(&[]);
    /// assert_eq!(Some(D3DERR::INVALIDCALL), r.err().map(|e| e.kind()));
    /// ```
    pub fn reflect_library<C: Raw>(&self, src_data: &[u8]) -> Result<C, Error> where C::Raw : Interface {
        let f = self.D3DReflectLibrary.ok_or(Error::new("D3DReflectLibrary", ErrorKind::MISSING_DLL_EXPORT))?;

        let mut reflector = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &C::Raw::uuidof(), &mut reflector) };
        Error::check("D3DReflectLibrary", hr)?;
        Ok(unsafe { C::from_raw(reflector.cast()) })
    }
}
