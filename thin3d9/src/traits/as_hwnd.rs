use crate::*;

use winapi::um::winuser::*;

use std::marker::PhantomData;


/// Indicates this type can be treated as an HWND.
///
/// ### Safety
///
/// By implementing this trait, you assert that `as_hwnd` returns either:
/// *   [std::ptr::null_mut]\(\)
/// *   A handle to a window belonging to the current thread.
///
/// If you have any sense at all, you'll ensure said handle remains valid at
/// least until a `&mut self` method is called, or the type is dropped, or
/// perhaps even longer.
pub unsafe trait AsHWND {
    fn as_hwnd(&self) -> HWND;
}

unsafe impl AsHWND for () {
    fn as_hwnd(&self) -> HWND { std::ptr::null_mut() }
}



pub struct SafeHWND<'w> {
    hwnd:       HWND,
    phantom:    PhantomData<&'w HWND>,
}

impl<'w> SafeHWND<'w> {
    /// Assert that `hwnd` is safe for the duration of the resulting `SafeHWND`'s lifetime.
    ///
    /// To encourage limited lifetimes, this takes `hwnd` by reference.
    pub unsafe fn assert(hwnd: &'w HWND) -> Self {
        let hwnd = *hwnd;
        assert!(hwnd.is_null() || IsWindow(hwnd) != 0);
        Self { hwnd, phantom: PhantomData }
    }
}

impl std::ops::Drop for SafeHWND<'_> {
    fn drop(&mut self) {
        assert!(self.hwnd.is_null() || unsafe { IsWindow(self.hwnd) } != 0);
    }
}

unsafe impl AsHWND for SafeHWND<'_> {
    fn as_hwnd(&self) -> HWND { self.hwnd }
}



#[test] #[should_panic] fn should_panic_on_create() {
    let _safe = unsafe { SafeHWND::assert(&(42 as HWND)) };
}

#[test] #[should_panic] fn should_panic_on_drop() {
    let window = extra::create_window("destroyed before dropped");
    let safe = unsafe { SafeHWND::assert(&(42 as HWND)) };
    unsafe { CloseWindow(window) };
    std::mem::drop(safe);
}

#[test] fn should_not_panic_null() {
    let safe = unsafe { SafeHWND::assert(&std::ptr::null_mut()) };
    std::mem::drop(safe);
}

#[test] fn should_not_panic_outlived() {
    let window = extra::create_window("destroyed before dropped");
    let safe = unsafe { SafeHWND::assert(&window) };
    std::mem::drop(safe);
    unsafe { CloseWindow(window) };
}
