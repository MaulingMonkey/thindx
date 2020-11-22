/// Placeholder for [Sharing Resources](https://docs.microsoft.com/en-us/windows/win32/direct3d9/dx9lh#sharing-resources)
///
/// This would generally be an in/out parameter.
/// Currently, the only type that implements this is `()`, which represents not using resource sharing.
pub unsafe trait SharedHandleParam {
    #[doc(hidden)] fn do_not_implement_me(&self) {}
}

unsafe impl SharedHandleParam for () {
    fn do_not_implement_me(&self) {}
}
