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
/// *   [examples::d3dcompiler_03_link]
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
    /// *   `cbuffer_slot`      - The cbuffer slot (e.g. which `cb#` register to read)
    /// *   `cbuffer_entry`     - The cbuffer entry (e.g. which `float4` element of the cbuffer to read the plane coefficients from)
    ///
    /// ### Errors
    /// *   [E::INVALIDARG]     - If `cbuffer_slot` >= the maximum number of cbuffer slots (`15` on my machine)
    /// *   [E::INVALIDARG]     - If `cbuffer_entry` >= the maximum number of cbuffer entries (`4096` on my machine)
    /// *   [E::INVALIDARG]     - If the slot + entry was already added
    /// *   [E::FAIL]           - If trying to add too many clip planes (e.g. 6 were already added, and you're attempting to add a 7th)
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// let linker : d3d11::Linker = d3dc.create_linker().unwrap();
    ///
    /// let add_clip_plane = |slot, entry| linker
    ///     .add_clip_plane_from_cbuffer(slot, entry)
    ///     .map_err(|e| e.kind());
    ///
    /// assert_eq!(add_clip_plane(0, 0), Ok(())); // 1
    /// assert_eq!(add_clip_plane(0, 0), Err(E::INVALIDARG)); // already added
    ///
    /// // "Each shader stage allows up to 15 shader-constant buffers; [...]"
    /// assert_eq!(add_clip_plane(1,  0), Ok(())); // 2
    /// assert_eq!(add_clip_plane(14, 0), Ok(())); // 3
    /// assert_eq!(add_clip_plane(15, 0), Err(E::INVALIDARG));
    ///
    /// // "[...] each buffer can hold up to 4096 constants."
    /// assert_eq!(add_clip_plane(0, 1   ), Ok(())); // 4
    /// assert_eq!(add_clip_plane(0, 4095), Ok(())); // 5
    /// assert_eq!(add_clip_plane(0, 4096), Err(E::INVALIDARG));
    /// #
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(!0, 0));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(0, !0));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(0, !0-1));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(0, !0-4));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(0, !0-10));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(0, !0-100));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(!0-1, 0));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(!0-4, 0));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(!0-10, 0));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(!0-100, 0));
    /// # assert_eq!(Err(E::INVALIDARG), add_clip_plane(!0, !0));
    ///
    /// // "... you provide a list of up to 6 float4 constants that define the plane coefficients
    /// // for each active clip plane."
    /// assert_eq!(add_clip_plane(3, 1), Ok(())); // 6th successful clip plane added to linker
    /// assert_eq!(add_clip_plane(3, 2), Err(E::FAIL)); // 7th clip plane exceeds limits
    /// ```
    ///
    /// ### See Also
    /// *   [User clip planes on feature level 9 hardware](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/user-clip-planes-on-10level9) (clip plane limit)
    /// *   [Introduction to Buffers in Direct3D 11:  Constant Buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d11/overviews-direct3d-11-resources-buffers-intro#constant-buffer) (cbuffer limits quoted in example)
    pub fn add_clip_plane_from_cbuffer(&self, cbuffer_slot: u32, cbuffer_entry: u32) -> Result<(), MethodError> {
        fn_context!(d3d11::Linker::add_clip_plane_from_cbuffer => ID3D11Linker::AddClipPlaneFromCBuffer);
        fn_check_hr!(unsafe { self.0.AddClipPlaneFromCBuffer(cbuffer_slot, cbuffer_entry) })
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
    /// let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// # let lib = d3dc.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", d3d::Compile::OptimizationLevel3, d3d::CompileEffect::None).unwrap();
    /// # let lib = d3dc.load_module(&lib).unwrap();
    /// # let lib = lib.create_instance(()).unwrap();
    /// #
    /// # let graph : d3d11::FunctionLinkingGraph = d3dc.create_function_linking_graph(None).unwrap();
    /// # let input  = graph.set_input_signature( &[d3d11::ParameterDesc::new(None, cstr!("POSITION0"),   d3d::SVT::Float, d3d::SVC::Vector, 1, 4, d3d::Interpolation::Linear,    d3d::PF::In,  0, 0, 0, 0)]).unwrap();
    /// # let output = graph.set_output_signature(&[d3d11::ParameterDesc::new(None, cstr!("SV_POSITION"), d3d::SVT::Float, d3d::SVC::Vector, 1, 4, d3d::Interpolation::Undefined, d3d::PF::Out, 0, 0, 0, 0)]).unwrap();
    /// # graph.pass_value(&input, 0, &output, 0).unwrap();
    /// # let (graph, _warnings) = graph.create_module_instance().unwrap();
    /// #
    /// let linker = d3dc.create_linker().unwrap();
    /// linker.use_library(&lib).unwrap();
    /// let shader = linker.link(&graph, "main", "vs_5_0", None).unwrap();
    /// ```
    ///
    /// ### See Also
    /// *   [examples::d3dcompiler_03_link]
    pub fn link(&self, entry: &ModuleInstance, entry_name: impl TryIntoAsCStr, target_name: impl TryIntoAsCStr, flags: Option<std::convert::Infallible>) -> Result<LinkResult, MethodErrorBlob> {
        fn_context!(d3d11::Linker::link => ID3D11Linker::Link);
        let entry_name  = entry_name .try_into().map_err(|e| MethodErrorBlob::new("ID3D11Linker::Link", e))?;
        let target_name = target_name.try_into().map_err(|e| MethodErrorBlob::new("ID3D11Linker::Link", e))?;
        let entry_name  = entry_name .as_cstr();
        let target_name = target_name.as_cstr();

        let _ = flags; let flags = 0;

        let mut blob = null_mut();
        let mut errors = null_mut();
        let hr = unsafe { self.0.Link(entry.as_raw(), entry_name, target_name, flags, &mut blob, &mut errors) };
        unsafe { MethodErrorBlob::check_blob("ID3D11Linker::Link", hr, errors) }?;
        Ok(LinkResult {
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
    /// let d3dc = d3d::Compiler::load_system(47).unwrap();
    ///
    /// let lib = d3dc.compile_from_file(
    ///     r"test\data\library.hlsl", None, None, (), "lib_5_0",
    ///     d3d::Compile::OptimizationLevel3, d3d::CompileEffect::None
    /// ).unwrap();
    /// let lib = d3dc.load_module(&lib).unwrap().create_instance(()).unwrap();
    ///
    /// let linker = d3dc.create_linker().unwrap();
    /// linker.use_library(&lib).unwrap();
    /// // linker.link(...), etc.
    /// ```
    ///
    /// ### See Also
    /// *   [examples::d3dcompiler_03_link]
    //#allow_missing_argument_docs
    pub fn use_library(&self, library_mi: &ModuleInstance) -> Result<(), MethodError> {
        fn_context!(d3d11::Linker::use_library => ID3D11Linker::UseLibrary);
        fn_check_hr!(unsafe { self.0.UseLibrary(library_mi.as_raw()) })
    }
}

//#cpp2rust ID3D11Linker                            = d3d11::Linker
