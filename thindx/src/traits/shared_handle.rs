use winapi::shared::ntdef::HANDLE;



/// Placeholder for [Sharing Resources](https://docs.microsoft.com/en-us/windows/win32/direct3d9/dx9lh#sharing-resources)
///
/// This would generally be an in/out parameter.
/// Currently, the only type that implements this is `()`, which represents not using resource sharing.
///
/// ### Safety
///
/// Don't implement this trait yet.  Just use `()`.
pub unsafe trait SharedHandleParam {
    #[doc(hidden)] fn to_handle(self) -> *mut HANDLE;
}

unsafe impl SharedHandleParam for () {
    fn to_handle(self) -> *mut HANDLE { std::ptr::null_mut() }
}
