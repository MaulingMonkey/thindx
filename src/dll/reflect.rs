use crate::*;

use winapi::Interface;

use std::ptr::*;



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
    /// // TODO: figure out what reflect actually takes and demo that.
    /// let ps = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    /// let r = compiler.reflect::<d3d11::ShaderReflection>(ps.shader.get_buffer()).unwrap();
    ///
    /// // Invalid interface:
    /// let r = compiler.reflect::<Unknown>(ps.shader.get_buffer());
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
        let f = self.D3DReflect.ok_or(ErrorKind::MISSING_DLL_EXPORT)?;

        let mut reflector = null_mut();
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), &I::Raw::uuidof(), &mut reflector) };
        Error::check("D3DReflect", hr)?;
        Ok(unsafe { I::from_raw(reflector.cast()) })
    }
}
