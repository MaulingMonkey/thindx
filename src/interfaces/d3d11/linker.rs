use crate::*;
use crate::d3d11::*;

use std::ffi::*;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11linker)\]
/// ID3D11Linker
///
/// A linker interface is used to link a shader module.
///
/// ### See Also
/// *   [D3DCompiler::create_linker]
#[derive(Clone)] #[repr(transparent)]
pub struct Linker(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11Linker>);

impl Linker {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11linker-addclipplanefromcbuffer)\]
    /// ID3D11Linker::AddClipPlaneFromCBuffer
    ///
    /// Adds a [clip plane] with the plane coefficients taken from a [cbuffer] entry for 10Level9 shaders.
    ///
    /// [clip plane]:       https://docs.microsoft.com/en-us/windows/desktop/direct3dhlsl/user-clip-planes-on-10level9
    /// [cbuffer]:          https://docs.microsoft.com/en-us/windows/desktop/direct3dhlsl/dx-graphics-hlsl-constants
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn add_clip_plane_from_cbuffer(&self, cbuffer_slot: u32, cbuffer_entry: u32) -> Result<(), Error> {
        let hr = unsafe { self.0.AddClipPlaneFromCBuffer(cbuffer_slot, cbuffer_entry) };
        Error::check("ID3D11Linker::AddClipPlaneFromCBuffer", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11linker-link)\]
    /// ID3D11Linker::Link
    ///
    /// Links the shader and produces a shader blob that the Direct3D runtime can use.
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn link(&self, entry: &ModuleInstance, entry_name: impl Into<CString>, target_name: impl Into<CString>, flags: ()) -> Result<CompileResult, Error> {
        let entry_name  = entry_name.into();
        let target_name = target_name.into();
        let entry_name  = entry_name.as_ptr();
        let target_name = target_name.as_ptr();

        let _ = flags; let flags = 0;

        let mut blob = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { self.0.Link(entry.as_raw(), entry_name, target_name, flags, &mut blob, &mut errors) };
        unsafe { Error::check_blob("ID3D11Linker::Link", hr, errors) }?;
        Ok(CompileResult { // TODO: rename CompileResult to something more general?  BlobWithWarnings?  BlobWithNonFatalErrors?
            shader: unsafe { ReadOnlyBlob::from_raw(blob) },
            errors: unsafe { ReadOnlyBlob::from_raw_opt(errors) },
        })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11linker-uselibrary)\]
    /// ID3D11Linker::UseLibrary
    ///
    ///
    ///
    /// ### Example
    /// ```rust
    /// # use thin3dcompiler::*;
    /// // TODO
    /// ```
    pub fn use_library(&self, library_mi: &ModuleInstance) -> Result<(), Error> {
        let hr = unsafe { self.0.UseLibrary(library_mi.as_raw()) };
        Error::check("ID3D11Linker::UseLibrary", hr)
    }
}
