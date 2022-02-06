use crate::*;

use winapi::shared::minwindef::{LPCVOID, UINT};
use winapi::shared::ntdef::{LPCSTR, HRESULT};
use winapi::um::d3dcommon::{ID3DInclude, ID3DIncludeVtbl, D3D_INCLUDE_TYPE};
use winapi::um::d3dcompiler::D3D_COMPILE_STANDARD_FILE_INCLUDE;

use std::alloc::Layout;
use std::convert::TryInto;
use std::path::*;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)\]
/// ID3DInclude compatible types
///
/// ### ⚠️ Safety ⚠️
/// By implementing this trait, you promise to return one of:
/// *   [null_mut]\(\)
/// *   [D3D_COMPILE_STANDARD_FILE_INCLUDE](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile#parameters)
/// *   A valid, well behaved [ID3D11Include](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)
///     instance, that lives for at least as long as `self` remains untouched / undropped.
pub unsafe trait AsInclude {
    /// Treat this as a raw winapi [ID3DInclude].
    fn as_id3dinclude(&self) -> *mut ID3DInclude;
}

unsafe impl AsInclude for () {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { null_mut() }
}

unsafe impl AsInclude for Option<std::convert::Infallible> {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { null_mut() }
}

unsafe impl<AI: AsInclude> AsInclude for &AI {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { (**self).as_id3dinclude() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile#parameters)\]
/// D3D_COMPILE_STANDARD_FILE_INCLUDE
///
/// This default include handler includes files that are relative to the current directory and files that are relative to
/// the directory of the initial source file. When you use [StandardFileInclude], you must specify the source file name
/// in the `source_name` parameter; the compiler will derive the initial relative directory from `source_name`.
pub struct StandardFileInclude;
unsafe impl AsInclude for StandardFileInclude {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { D3D_COMPILE_STANDARD_FILE_INCLUDE }
}

//#cpp2rust ID3DInclude                         = trait d3d::AsInclude
//#cpp2rust D3D_COMPILE_STANDARD_FILE_INCLUDE   = d3d::StandardFileInclude




/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude)\]
/// ID3DInclude
#[repr(C)] pub struct Include<I> {
    vtable: *const ID3DIncludeVtbl,
    imp:    I,
}

unsafe impl<I> AsInclude for Include<I> {
    fn as_id3dinclude(&self) -> *mut ID3DInclude { self as *const Self as *mut Self as *mut ID3DInclude }
}

impl Include<()> {
    /// Wrap [Fn]\([d3d::IncludeType], file_name: [abistr::CStrNonNull], parent: Option<(&H, &\[u8\])>\).
    ///
    /// ### Examples
    /// ```rust
    /// # use thindx::*;
    /// # use std::path::*;
    /// # return; // doc tests have wrong dir
    /// let d3dc = d3d::Compiler::load_system(47).unwrap();
    ///
    /// let include = d3d::Include::from_fn_with_header(|include_type, file_name, parent|{
    ///     let (quote, unquote) = match include_type {
    ///         d3d::Include::Local     => ('"', '"'),
    ///         d3d::Include::System    => ('<', '>'),
    ///         _                       => ('?', '?'),
    ///     };
    ///
    ///     let file_name   = file_name.to_str().map_err(|_| E::FAIL)?;
    ///     let path        = Path::new(r"thindx\test\data").join(file_name);
    ///
    ///     println!("resolving `#include {quote}{file_name}{unquote}` to {path:?}");
    ///
    ///     let data        = std::fs::read(&path).map_err(|_| E::FAIL)?;
    ///     let header      = path; // PathBuf
    ///
    ///     println!("  read {} bytes", data.len());
    ///
    ///     if let Some((parent_header, _parent_data)) = parent {
    ///         println!("  into {:?}", parent_header);
    ///     } else {
    ///         println!("  into root file");
    ///     }
    ///
    ///     Ok((header, data))
    /// });
    ///
    /// let compiled = d3dc.compile_from_file(
    ///     r"thindx\test\data\include-chain-1.hlsl", None, &include,
    ///     "ps_main", "ps_4_0", d3d::Compile::Debug, d3d::CompileEffect::None
    /// );
    /// ```
    /// *   [_examples::d3dcompiler_02_compile]
    pub fn from_fn_with_header<H, F: Fn(d3d::IncludeType, abistr::CStrNonNull, Option<(&H, &[u8])>) -> Result<(H, Vec<u8>), ErrorKind>>(f: F) -> Include<F> {
        let vtable : &'static ID3DIncludeVtbl = &ID3DIncludeVtbl {
            Open:   open::<H, F>,
            Close:  close::<H, F>,
        };

        return Include {
            vtable: vtable,
            imp:    f,
        };

        /// ### ⚠️ Safety ⚠️
        /// *   All pointers should be valid
        /// *   No pointers should alias each other
        /// *   `this` must point at a `Self`
        unsafe extern "system" fn open<H, F: Fn(d3d::IncludeType, abistr::CStrNonNull, Option<(&H, &[u8])>) -> Result<(H, Vec<u8>), ErrorKind>>(this: *mut ID3DInclude, include_type: D3D_INCLUDE_TYPE, file_name: LPCSTR, parent_data: LPCVOID, data: *mut LPCVOID, bytes: *mut UINT) -> HRESULT {
            if this         .is_null() { return E::INVALIDARG.into() }
            if file_name    .is_null() { return E::INVALIDARG.into() }
            if data         .is_null() { return E::INVALIDARG.into() }
            if bytes        .is_null() { return E::INVALIDARG.into() }

            let this            = unsafe { &*(this as *const Include<F>) };
            let include_type    = d3d::IncludeType::from_unchecked(include_type);
            let file_name       = unsafe { abistr::CStrNonNull::from_ptr_unchecked_unbounded(file_name) };

            let parent = if parent_data.is_null() {
                None
            } else {
                let parent_data = parent_data as *mut u8;
                let parent_thb = unsafe { ThinHeaderBlob::<H>::from_data(parent_data) };
                let parent_thb = unsafe { &*parent_thb };
                let parent_data = unsafe { std::slice::from_raw_parts(parent_data, (*parent_thb).data_bytes) };
                Some((&parent_thb.header, parent_data))
            };

            let o_data  = unsafe { &mut *data };
            let o_bytes = unsafe { &mut *bytes };
            *o_data     = null();
            *o_bytes    = 0;

            match (this.imp)(include_type, file_name, parent) {
                Ok((header, data)) => {
                    let len32 : UINT = match data.len().try_into() {
                        Ok(n)   => n,
                        Err(_)  => return THINERR::SLICE_TOO_LARGE.into(),
                    };

                    let thb = match ThinHeaderBlob::<H>::alloc(header, &data[..]) {
                        Ok(thb) => thb.as_ptr(),
                        Err(_)  => return THINERR::SLICE_TOO_LARGE.into(),
                    };

                    *o_data     = unsafe { ThinHeaderBlob::<H>::to_data(thb) }.cast();
                    *o_bytes    = len32;
                    S::OK
                },
                Err(kind) => kind,
            }.into()
        }

        /// ### ⚠️ Safety ⚠️
        /// *   All pointers should be valid
        /// *   `this` must point at a `Self`
        /// *   `data` must match the output of a previous call to `open` that has not yet been `close`d
        unsafe extern "system" fn close<H, F: Fn(d3d::IncludeType, abistr::CStrNonNull, Option<(&H, &[u8])>) -> Result<(H, Vec<u8>), ErrorKind>>(this: *mut ID3DInclude, data: LPCVOID) -> HRESULT {
            if this         .is_null() { return E::INVALIDARG.into() }
            if data         .is_null() { return E::INVALIDARG.into() }

            // this is unused
            let data        = data as *mut u8;
            let thb         = unsafe { ThinHeaderBlob::<H>::from_data(data) };

            unsafe { ThinHeaderBlob::<H>::free(thb) };

            S::OK.into()
        }
    }

    /// Wrap [Fn]\([d3d::IncludeType], file_name: [abistr::CStrNonNull]\).
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # use std::path::*;
    /// # return; // doc tests have wrong dir
    /// let d3dc = d3d::Compiler::load_system(47).unwrap();
    ///
    /// let include = d3d::Include::from_fn(|_ty, file|{
    ///     let p = Path::new(r"thindx\test\data");
    ///     let p = p.join(file.to_str().map_err(|_| D3D11_ERROR::FILE_NOT_FOUND)?);
    ///     std::fs::read(p).map_err(|_| D3D11_ERROR::FILE_NOT_FOUND)
    /// });
    ///
    /// let compiled = d3dc.compile_from_file(
    ///     r"thindx\test\data\include-chain-1.hlsl", None, &include,
    ///     "ps_main", "ps_4_0", d3d::Compile::Debug, d3d::CompileEffect::None
    /// );
    /// ```
    pub fn from_fn(f: impl Fn(d3d::IncludeType, abistr::CStrNonNull) -> Result<Vec<u8>, ErrorKind>) -> impl AsInclude {
        Self::from_fn_with_header(move |include_type, file_name, _parent| {
            Ok(((), f(include_type, file_name)?))
        })
    }

    /// Wrap [Fn]\(dir: &[Path], [d3d::IncludeType], include: [abistr::CStrNonNull]\) -> [PathBuf].
    ///
    /// ### Examples
    /// ```rust
    /// # use thindx::*;
    /// # return; // doc tests have wrong dir
    /// let d3dc = d3d::Compiler::load_system(47).unwrap();
    ///
    /// let include = d3d::Include::from_path_fn(
    ///     r"thindx\test\data",
    ///     |dir, _t, file| Ok(dir.join(file.to_str().map_err(|_| D3D11_ERROR::FILE_NOT_FOUND)?))
    /// );
    ///
    /// let compiled = d3dc.compile_from_file(
    ///     r"thindx\test\data\include-chain-1.hlsl", None, &include,
    ///     "ps_main", "ps_4_0", d3d::Compile::Debug, d3d::CompileEffect::None
    /// );
    /// ```
    /// *   [_examples::d3dcompiler_02_compile]
    pub fn from_path_fn<'a>(dir: impl AsRef<Path> + 'a, f: impl 'a + Fn(&Path, d3d::IncludeType, abistr::CStrNonNull) -> Result<PathBuf, ErrorKind>) -> impl AsInclude + 'a {
        Self::from_fn_with_header(move |include_type, file_name: abistr::CStrNonNull, parent: Option<(&PathBuf, &[u8])>| {
            let dir = match parent {
                Some((parent_dir, _parent_data))    => &**parent_dir,
                None                                => dir.as_ref(),
            };
            let mut path = f(dir, include_type, file_name)?;
            let data = std::fs::read(&path).map_err(|err| err.raw_os_error().map_or(D3D11_ERROR::FILE_NOT_FOUND, |raw| ErrorKind::from_win32(raw as _)))?;
            if !path.pop() { return Err(D3D11_ERROR::FILE_NOT_FOUND); } // path: filename -> dir
            Ok((path, data))
        })
    }
}

//#cpp2rust ID3DInclude         = struct d3d::Include
//#cpp2rust ID3DInclude::Open   = d3d::Include::from_fn_with_header
//#cpp2rust ID3DInclude::Open   = d3d::Include::from_fn
//#cpp2rust ID3DInclude::Open   = d3d::Include::from_path_fn
//#cpp2rust ID3DInclude::Close  = std::ops::Drop::drop



struct ThinHeaderBlob<H> {
    data_bytes: usize, // not inclusive of `data_bytes` or `header`
    header:     H,
    // data follows
}

impl<H> ThinHeaderBlob<H> {
    pub fn alloc(header: H, data: &[u8]) -> Result<NonNull<Self>, ()> {
        let layout = Self::layout(data.len())?;
        let thb;
        unsafe {
            thb = std::alloc::alloc(layout) as *mut Self;
            std::ptr::write(thb, Self { data_bytes: data.len(), header });
            std::ptr::copy_nonoverlapping(data.as_ptr(), thb.add(1).cast(), data.len());
        }
        NonNull::new(thb).ok_or(())
    }

    /// ### ⚠️ Safety ⚠️
    /// *   `data` must belong to a valid allocated `ThinHeaderBlob` that has not been `free`d
    pub unsafe fn from_data(data: *mut u8) -> *mut Self {
        unsafe { (data as *mut Self).sub(1) }
    }

    /// ### ⚠️ Safety ⚠️
    /// *   `thb` must be a valid allocated `ThinHeaderBlob` that has not been `free`d
    pub unsafe fn to_data(thb: *mut Self) -> *mut u8 {
        unsafe { thb.add(1).cast() }
    }

    /// ### ⚠️ Safety ⚠️
    /// *   `thb` must be a valid allocated `ThinHeaderBlob` that has not been `free`d
    /// *   `thb` will no longer be valid after this call
    pub unsafe fn free(thb: *mut Self) {
        let _header : Self = unsafe { std::ptr::read(thb) };
        let layout = Self::layout(unsafe { (*thb).data_bytes }).unwrap();
        unsafe { std::alloc::dealloc(thb.cast(), layout) }
    }

    fn layout(data_bytes: usize) -> Result<Layout, ()> {
        let header = Layout::new::<Self>();
        let data   = Layout::array::<u8>(data_bytes).map_err(|_|())?;
        Ok(header.extend(data).map_err(|_| ())?.0)
    }
}
