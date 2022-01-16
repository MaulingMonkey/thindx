pub use minidl::Library;

use winapi::shared::minwindef::{WORD, HMODULE, FARPROC};
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryExA, LOAD_LIBRARY_SEARCH_SYSTEM32};

use std::io;
use std::mem::size_of;
use std::ptr::null_mut;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexa)\]
/// Load a library using with `LOAD_LIBRARY_SEARCH_SYSTEM32`
///
/// ### Arguments
///
/// - `name0`   should be a `\0` terminated DLL name, like `"d3dcompiler_47.dll\0"`
///
/// ### Panics
///
/// - if `name0` does not end with `\0`
/// - if `name0` contains interior `\0`s
///
/// ### See Also
///
/// *   Dynamic-Link Library Security<br>
///     <https://docs.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-security>
/// *   Secure loading of libraries to prevent DLL preloading attacks<br>
///     <https://support.microsoft.com/en-us/topic/secure-loading-of-libraries-to-prevent-dll-preloading-attacks-d41303ec-0748-9211-f317-2edc819682e1>
pub(crate) fn load_system_0(name0: &str) -> minidl::Result<Library> {
    let path = name0.strip_suffix('\0').expect("name0 must end in `\\0`");
    assert!(!path.contains('\0'));

    // SAFETY: ⚠️ could use testing on older/unsupported windows
    //  * `name0`   ✔️ is `\0` terminated as verified in expect above
    //  * `name0`   ✔️ contains no interior `\0`s as verified in assert above
    //  * null      ✔️ is the expected reserved value for the second param
    //  * flags     ✔️ are hardcoded, well documented
    //  * flags     ⚠️ are untested on older/unsupported windows
    let hmodule = unsafe { LoadLibraryExA(name0.as_ptr().cast(), null_mut(), LOAD_LIBRARY_SEARCH_SYSTEM32) };

    if !hmodule.is_null() {
        // SAFETY: ⚠️ `minidl::Library` is a `#[repr(transparent)]` wrapper around a basic pointer type as of minidl.rev = "e1e86cb7a6e48a3ed1aff4a1e927311d90039e82"
        return Ok(unsafe { std::mem::transmute(hmodule) });
    }

    let err = io::Error::last_os_error();
    match err.raw_os_error() {
        Some(ERROR_BAD_EXE_FORMAT) => {
            Err(io::Error::new(io::ErrorKind::Other, format!(
                "Unable to load {path}: ERROR_BAD_EXE_FORMAT (likely tried to load a {that}-bit DLL into this {this}-bit process)",
                this = if cfg!(target_arch = "x86_64") { "64" } else { "32" },
                that = if cfg!(target_arch = "x86_64") { "32" } else { "64" },
            )))
        },
        Some(ERROR_MOD_NOT_FOUND) => {
            Err(io::Error::new(io::ErrorKind::NotFound, format!("Unable to load {path}: NotFound")))
        },
        _ => Err(err)
    }
}



impl LibraryExt for Library {}
pub(crate) trait LibraryExt : Sized + From<Library> + Into<Library> {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)\]
    /// Load a symbol from the library by ordinal.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   This function implicitly transmutes `FARPROC` to `FnPtr` without type checking!
    ///     Use extreme caution, and consider following [d3d::Compiler::load_impl]'s example in using a proc macro to verify function type signatures against winapi if possible.
    /// *   New DLLs often change the ordinals of their functions.  This is super sketchy!
    unsafe fn sym_opt_by_ordinal<FnPtr>(self, ordinal: WORD) -> Option<FnPtr> {
        assert_eq!(size_of::<FnPtr>(), size_of::<FARPROC>(), "symbol result is not pointer sized!");

        // SAFETY: ✔️
        //  * `hmodule`     ✔️ is a valid, non-dangling, permanently loaded hmodule
        //  * `ordinal`     ✔️ is a WORD/u16, meeting GetProcAddress's documented requirement:
        //                  "If this parameter is an ordinal value, it must be in the low-order word; the high-order word must be zero."
        let func = GetProcAddress(self.as_hmodule(), ordinal as usize as *const _);
        if func.is_null() {
            None
        } else {
            // SAFETY: ✔️
            //  * `FnPtr`       ✔️ is asserted to be the same size as `FARPROC` via assert at start of function (can't enforce this at compile time)
            //  * `FnPtr`       ✔️ is assumed compatible with `FARPROC` per the documented safety contract of this unsafe function
            Some(std::mem::transmute_copy::<FARPROC, FnPtr>(&func))
        }
    }

    fn has_sym(self, name: impl AsRef<str>) -> bool {
        // SAFETY: ✔️ FARPROC -> FARPROC should always be a safe transmute
        let s : Option<FARPROC> = unsafe { self.into().sym_opt(name) };
        s.is_some()
    }

    #[allow(clippy::wrong_self_convention)]
    fn as_hmodule(self) -> HMODULE {
        // SAFETY: ⚠️ `minidl::Library` is a `#[repr(transparent)]` wrapper around a basic pointer type as of minidl.rev = "e1e86cb7a6e48a3ed1aff4a1e927311d90039e82"
        unsafe { std::mem::transmute::<Library, HMODULE>(self.into()) }
    }

    /// ### ⚠️ Safety ⚠️
    ///
    /// * `hmodule` should be a permanently loaded, valid module
    unsafe fn from_hmodule(hmodule: HMODULE) -> Option<Self> {
        if hmodule.is_null() {
            None
        } else {
            // SAFETY: ⚠️ `minidl::Library` is a `#[repr(transparent)]` wrapper around a basic pointer type as of minidl.rev = "e1e86cb7a6e48a3ed1aff4a1e927311d90039e82"
            Some(Self::from(std::mem::transmute::<HMODULE, Library>(hmodule)))
        }
    }
}

const ERROR_BAD_EXE_FORMAT : i32 = 0x00C1;
const ERROR_MOD_NOT_FOUND  : i32 = 0x007E;
