use crate::d3d::*;

use std::path::*;



/// <h1 id="constructors" class="section-header"><a href="#constructors">Constructors</a></h1>
impl Compiler {
    /// Attempt to load d3dcompiler_NN.dll
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// **Prefer [Compiler::load_system].**
    ///
    /// The possibility of DLL preloading attacks makes this function insecure.
    /// To be fair, the security of using d3dcompiler at all is questionable.
    /// If your data is untrusted, should you really be using this API at all?
    /// This is a developer focused, unfuzzed, native C++ parser and deserializer, with all the edge cases that entails.
    ///
    /// Recommended reading, to secure more than just this call:
    ///
    /// *   Dynamic-Link Library Security<br>
    ///     <https://docs.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-security>
    /// *   Secure loading of libraries to prevent DLL preloading attacks<br>
    ///     <https://support.microsoft.com/en-us/topic/secure-loading-of-libraries-to-prevent-dll-preloading-attacks-d41303ec-0748-9211-f317-2edc819682e1>
    ///
    /// Additionally, one can argue about if it's sound to consider this function memory safe (e.g. missing the `unsafe` keyword.)
    /// A user-provided custom `d3dcompiler_NN.dll` could relax soundness guarantees vs a Microsoft version.
    /// However, taking that argument to the extreme, one could argue a user-provided custom kernel32.dll breaks rust's entire stdlib.
    /// While Windows attempts to guard against that somewhat, the interpid *could* be running this executable on redox.  Or on linux with wine.  Or...
    ///
    /// So, instead, I assert the following:
    /// *   Functions named `D3DCompile`, `D3DCreateBlob`, etc. have an implicit - if incredibly ill defined - contract.
    /// *   It should be "sound" to rely on that contract.  Violations of the contract are bugs in the DLLs, not the consuming Rust code.
    /// *   That contract is narrowed by bugs in known/common d3dcompiler versions (including 3rd party ones), so please [file issues](https://github.com/MaulingMonkey/thindx/issues/new) if you can crash it!
    /// *   Working around *intentional* undefined behavior - by e.g. defining D3DCompiler with an incorrect function signature, or to immediately dereference null with any/all args just to prove a point, is not.
    /// *   If in doubt, file an issue!  I can simply close it if I disagree.
    ///
    /// ### Arguments
    /// *   `version` - the d3dcompiler.dll version to load
    ///     * [i32], [u32], [usize], [u64] - load `d3dcompiler_{version}.dll`
    ///     * &[str], &[String], &[Path], or &[PathBuf] - load `{version}`
    ///
    /// ### Returns
    /// *   Err([std::io::Error])   - if `d3dcompiler_{version}.dll` could not be loaded
    /// *   Ok([Compiler])          - `d3dcompiler_{version}.dll` was found
    ///
    /// ### Example
    /// ```rust
    /// use thindx::d3d;
    /// let d3dc = d3d::Compiler::load_insecure(47).unwrap();
    /// let d3dc = d3d::Compiler::load_insecure("d3dcompiler_47.dll").unwrap();
    /// ```
    pub fn load_insecure(version: impl CompilerLoadInsecure) -> Result<Self, std::io::Error> {
        Self::load_impl(version.try_load()?)
    }

    /// Attempt to load d3dcompiler_NN.dll via LOAD_LIBRARY_SEARCH_SYSTEM32 (e.g. `%WINDOWS%\System32`)
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// The use of LOAD_LIBRARY_SEARCH_SYSTEM32 should guard somewhat against DLL preloading attacks.
    ///
    /// However, using d3dcompiler at all on untrusted data is questionable.
    /// If your data is untrusted, should you really be using this API at all?
    /// This is a developer focused, unfuzzed, native C++ parser and deserializer, with all the edge cases that entails.
    ///
    /// ### Arguments
    /// *   `version` - the d3dcompiler.dll version to load
    ///     * [i32], [u32], [usize], [u64] - load `d3dcompiler_{version}.dll`
    ///
    /// ### Returns
    /// *   Err([std::io::Error])   - if `d3dcompiler_{version}.dll` could not be loaded
    /// *   Ok([Compiler])          - `d3dcompiler_{version}.dll` was found
    ///
    /// ### Example
    /// ```rust
    /// use thindx::d3d;
    /// let d3dc = d3d::Compiler::load_system(47).unwrap();
    /// ```
    ///
    /// ### Remarks
    /// | Platform                  | Support   |
    /// | ------------------------- | --------- |
    /// | Windows 8+                | ✔️ Supported
    /// | Windows 7                 | ⚠️ Requires [KB2533623](https://support.microsoft.com/kb/2533623)
    /// | Windows Vista             | ⚠️ Requires [KB2533623](https://support.microsoft.com/kb/2533623)
    /// | Windows XP                | ❌ Not supported
    /// | Windows Server 2012+      | ✔️ Supported
    /// | Windows Server 2008 R2    | ⚠️ Requires [KB2533623](https://support.microsoft.com/kb/2533623)
    /// | Windows Server 2008       | ⚠️ Requires [KB2533623](https://support.microsoft.com/kb/2533623)
    /// | Windows Server 2003       | ❌ Not supported
    pub fn load_system(verison: impl CompilerLoadSysemVersion) -> Result<Self, std::io::Error> {
        Self::load_impl(verison.try_load()?)
    }

    fn load_impl(lib: minidl::Library) -> Result<Self, std::io::Error> {
        macro_rules! compiler { ( $($ident:ident),* $(,)? ) => {{
            #[allow(dead_code, unused_assignments, non_snake_case)]
            #[cfg(not(unsafe_unsound_unstable_remove_static_asserts_for_coverage))]
            fn static_assert_correct_types(compiler: &Compiler) {
                $(
                    let mut $ident = compiler.$ident.unwrap();
                    $ident = winapi::um::d3dcompiler::$ident;
                    let _ = $ident;
                )*
            }

            // SAFETY: ✔️
            //  * `sym_opt` ✔️ asserts that `$ident` is nul terminated and has no interior nuls
            //  * `sym_opt` ✔️ is a fairly vanilla ASCII identifier thanks to the `:ident` bound
            //  * `$ident`  ✔️ fn symbols are type-checked against winapi above in static_assert_correct_types
            //  * `lib`     ✔️ is permanently loaded library (per fundamental design of `minidl`)
            //  * `lib`     ⚠️ could be malicious, but it's too late to do anything about it by the time we have a `Library`.
            //              See more detailed safety section ramblings in `load_*` for details / mitigation.
            let compiler = unsafe { Self { $(
                $ident : lib.sym_opt(concat!(stringify!($ident), "\0")),
            )* } };

            compiler
        }}}

        #[allow(clippy::undocumented_unsafe_blocks)] // there is a safety doc inside the macro... clippy is blind!
        Ok(compiler! {
            D3DCompile,
            D3DCompile2,
            D3DCompileFromFile,
            D3DCompressShaders,
            D3DCreateBlob,
            D3DCreateFunctionLinkingGraph,
            D3DCreateLinker,
            D3DDecompressShaders,
            D3DDisassemble,
            D3DDisassembleRegion,
            D3DGetBlobPart,
            D3DGetDebugInfo,
            D3DGetInputAndOutputSignatureBlob,
            D3DGetInputSignatureBlob,
            D3DGetOutputSignatureBlob,
            D3DGetTraceInstructionOffsets,
            D3DLoadModule,
            D3DPreprocess,
            D3DReadFileToBlob,
            D3DReflect,
            D3DReflectLibrary,
            D3DSetBlobPart,
            D3DStripShader,
            D3DWriteBlobToFile,
            //UncommentToVerifyStaticAssertCatchesEverything,
        })
    }
}

#[doc(hidden)] pub trait CompilerLoadInsecure       : sealed::CompilerLoadInsecure      {}
#[doc(hidden)] pub trait CompilerLoadSysemVersion   : sealed::CompilerLoadSysemVersion  {}

impl<T: sealed::CompilerLoadInsecure    > CompilerLoadInsecure      for T {}
impl<T: sealed::CompilerLoadSysemVersion> CompilerLoadSysemVersion  for T {}

mod sealed {
    use crate::dll;
    use super::*;

    pub trait CompilerLoadInsecure              { fn try_load(self) -> minidl::Result<minidl::Library>; }
    impl CompilerLoadInsecure for i32           { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(format!("d3dcompiler_{}.dll", self)) } }
    impl CompilerLoadInsecure for u32           { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(format!("d3dcompiler_{}.dll", self)) } }
    impl CompilerLoadInsecure for usize         { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(format!("d3dcompiler_{}.dll", self)) } }
    impl CompilerLoadInsecure for u64           { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(format!("d3dcompiler_{}.dll", self)) } }
    impl CompilerLoadInsecure for &'_ Path      { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(self) } }
    impl CompilerLoadInsecure for &'_ PathBuf   { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(self) } }
    impl CompilerLoadInsecure for &'_ str       { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(self) } }
    impl CompilerLoadInsecure for &'_ String    { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(self) } }

    pub trait CompilerLoadSysemVersion          { fn try_load(self) -> minidl::Result<minidl::Library>; }
    impl CompilerLoadSysemVersion for i32       { fn try_load(self) -> minidl::Result<minidl::Library> { dll::load_system_0(&format!("d3dcompiler_{}.dll\0", self)) } }
    impl CompilerLoadSysemVersion for u32       { fn try_load(self) -> minidl::Result<minidl::Library> { dll::load_system_0(&format!("d3dcompiler_{}.dll\0", self)) } }
    impl CompilerLoadSysemVersion for usize     { fn try_load(self) -> minidl::Result<minidl::Library> { dll::load_system_0(&format!("d3dcompiler_{}.dll\0", self)) } }
    impl CompilerLoadSysemVersion for u64       { fn try_load(self) -> minidl::Result<minidl::Library> { dll::load_system_0(&format!("d3dcompiler_{}.dll\0", self)) } }
}
