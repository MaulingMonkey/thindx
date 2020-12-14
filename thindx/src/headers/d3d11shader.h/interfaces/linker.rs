use crate::*;
use crate::ctypes::*;
use crate::d3d::*;
use crate::d3d11::*;

use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11linker)\]
/// ID3D11Linker
///
/// A linker interface is used to link a shader module.
///
/// ### See Also
/// *   [d3d::Compiler::create_linker] to create [Linker]s
/// *   [d3d11::FunctionLinkingGraph] for examples
#[derive(Clone)] #[repr(transparent)]
pub struct Linker(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11Linker>);

convert!(unsafe Linker => Unknown, winapi::um::d3d11shader::ID3D11Linker);

impl Linker {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11linker-addclipplanefromcbuffer)\]
    /// ID3D11Linker::AddClipPlaneFromCBuffer
    ///
    /// Adds a [clip plane] with the plane coefficients taken from a [cbuffer] entry for 10Level9 shaders.
    ///
    /// [clip plane]:       https://docs.microsoft.com/en-us/windows/desktop/direct3dhlsl/user-clip-planes-on-10level9
    /// [cbuffer]:          https://docs.microsoft.com/en-us/windows/desktop/direct3dhlsl/dx-graphics-hlsl-constants
    ///
    /// ### Arguments
    /// *   `cbuffer_slot`      - The cbuffer slot (is this the same as the `cb#` register?)
    /// *   `cbuffer_entry`     - The cbuffer entry (is this the offset within the cbuffer in `float4` registers?)
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    ///
    /// ### See Also
    /// *   [User clip planes on feature level 9 hardware](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/user-clip-planes-on-10level9)
    pub fn add_clip_plane_from_cbuffer(&self, cbuffer_slot: u32, cbuffer_entry: u32) -> Result<(), Error> {
        let hr = unsafe { self.0.AddClipPlaneFromCBuffer(cbuffer_slot, cbuffer_entry) };
        Error::check("ID3D11Linker::AddClipPlaneFromCBuffer", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11linker-link)\]
    /// ID3D11Linker::Link
    ///
    /// Links the shader and produces a shader blob that the Direct3D runtime can use.
    ///
    /// ### Arguments
    /// *   `entry`         - A [`ModuleInstance`] created by e.g. [FunctionLinkingGraph::create_module_instance]
    /// *   `entry_name`    - The name to give the generated entry point when linking the shader (mostly for debug purpouses.)
    /// *   `target_name`   - What kind of shader to generate (e.g. `"vs_5_0"` etc.)
    /// *   `flags`         - Reserved.  Initialize with [`None`].
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    ///
    /// ### See Also
    /// *   [examples::d3dcompiler_03_link]
    pub fn link(&self, entry: &ModuleInstance, entry_name: impl TryIntoAsCStr, target_name: impl TryIntoAsCStr, flags: Option<void::Void>) -> Result<CompileResult, Error> {
        let entry_name  = entry_name .try_into().map_err(|e| Error::new("ID3D11Linker::Link", e))?;
        let target_name = target_name.try_into().map_err(|e| Error::new("ID3D11Linker::Link", e))?;
        let entry_name  = entry_name .as_cstr();
        let target_name = target_name.as_cstr();

        let _ = flags; let flags = 0;

        let mut blob = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { self.0.Link(entry.as_raw(), entry_name, target_name, flags, &mut blob, &mut errors) };
        unsafe { Error::check_blob("ID3D11Linker::Link", hr, errors) }?;
        Ok(CompileResult { // TODO: rename CompileResult to something more general?  BlobWithWarnings?  BlobWithNonFatalErrors?
            shader: unsafe { CodeBlob::from_unchecked(ReadOnlyBlob::from_raw(blob)) },
            errors: TextBlob::new(unsafe { ReadOnlyBlob::from_raw_opt(errors) }),
        })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11linker-uselibrary)\]
    /// ID3D11Linker::UseLibrary
    ///
    /// Adds an instance of a library module to be used for linking.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    #[xallow(missing_argument_docs)]
    pub fn use_library(&self, library_mi: &ModuleInstance) -> Result<(), Error> {
        let hr = unsafe { self.0.UseLibrary(library_mi.as_raw()) };
        Error::check("ID3D11Linker::UseLibrary", hr)
    }
}
