use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::marker::PhantomData;
use std::ptr::*;


/// Indicates this type can be treated as an HWND.
///
/// ### ⚠️ Safety ⚠️
/// By implementing this trait, you assert that `as_hwnd` returns either:
/// *   [std::ptr::null_mut]\(\)
/// *   A handle to a window belonging to the current thread.
///
/// If you have any sense at all, you'll ensure said handle remains valid at
/// least until a `&mut self` method is called, or the type is dropped, or
/// perhaps even longer.
pub unsafe trait AsHWND {
    #[allow(missing_docs)]
    fn as_hwnd(&self) -> HWND;
}

unsafe impl AsHWND for () {
    fn as_hwnd(&self) -> HWND { std::ptr::null_mut() }
}



/// A safer/sounder alternative to [HWND]
///
/// [HWND]: https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-
#[derive(Debug)] // cmp/hash?
#[repr(transparent)] pub struct SafeHWND<'w> {
    hwnd:       NonNull<HWND__>,
    phantom:    PhantomData<&'w HWND>,
}

impl SafeHWND<'static> {
    /// Assert that `hwnd` is safe for the duration of the resulting `SafeHWND`'s lifetime.
    ///
    /// Prefer the lifetime-bounded [SafeHWND::assert] when possible.
    ///
    /// ### ⚠️ Safety ⚠️
    /// By using this method, you assert that `hwnd` will be valid for the entire duration of `Self`.
    /// This is enforced by `assert!(...)` upon construction and drop, but that's potentially *after* undefined behavior has been invoked.
    pub unsafe fn assert_unbounded(hwnd: HWND) -> Option<Self> {
        assert!(hwnd.is_null() || unsafe { IsWindow(hwnd) } != 0);
        Some(Self { hwnd: NonNull::new(hwnd)?, phantom: PhantomData })
    }
}

impl<'w> SafeHWND<'w> {
    /// Assert that `hwnd` is safe for the duration of the resulting `SafeHWND`'s lifetime.
    ///
    /// To encourage limited lifetimes, this takes `hwnd` by reference.
    ///
    /// ### ⚠️ Safety ⚠️
    /// By using this method, you assert that `hwnd` will be valid for the entire duration of `Self`.
    /// This is enforced by `assert!(...)` upon construction and drop, but that's potentially *after* undefined behavior has been invoked.
    pub unsafe fn assert(hwnd: &'w HWND) -> Option<Self> {
        let hwnd = *hwnd;
        assert!(hwnd.is_null() || unsafe { IsWindow(hwnd) } != 0);
        Some(Self { hwnd: NonNull::new(hwnd)?, phantom: PhantomData })
    }

    /// Check the validity of the underlying [HWND] handle.
    ///
    /// Chances are, if this ever returns `false`, you've already invoked undefined behavior.
    ///
    /// [HWND]: https://learn.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-
    pub fn is_valid(&self) -> bool {
        unsafe { IsWindow(self.hwnd.as_ptr()) != 0 }
    }
}

impl<'a> Clone for SafeHWND<'a> {
    fn clone(&self) -> Self {
        assert!(self.is_valid(), "SafeHWND wasn't valid when cloned!  Likely undefined behavior!");
        Self { hwnd: self.hwnd, phantom: self.phantom }
    }
}

impl std::ops::Drop for SafeHWND<'_> {
    fn drop(&mut self) {
        assert!(self.is_valid(), "SafeHWND wasn't valid when dropped!  Likely undefined behavior!");
    }
}

unsafe impl AsHWND for SafeHWND<'_> {
    fn as_hwnd(&self) -> HWND { self.hwnd.as_ptr() }
}

unsafe impl AsHWND for Option<SafeHWND<'_>> {
    fn as_hwnd(&self) -> HWND { self.as_ref().map_or(null_mut(), |sw| sw.hwnd.as_ptr()) }
}



#[cfg(test)] mod tests {
    use super::*;
    use dev::win32::*;
    use dev::d3d9::AsHWND;

    #[test] fn layout() {
        use std::mem::*;

        assert_eq!(size_of ::<SafeHWND>(), size_of ::<HWND>());
        assert_eq!(align_of::<SafeHWND>(), align_of::<HWND>());
        assert_eq!(size_of ::<Option<SafeHWND>>(), size_of ::<HWND>());
        assert_eq!(align_of::<Option<SafeHWND>>(), align_of::<HWND>());
        assert_eq!(0, unsafe { std::mem::transmute::<Option<SafeHWND>, usize>(None) }, "Option<SafeHWND> should be bytemuck::Zeroable");
    }

    #[test] #[should_panic] fn should_panic_on_create() {
        let _safe = unsafe { SafeHWND::assert(&(!42 as HWND)) };
    }

    #[test] #[should_panic] fn should_panic_on_drop() {
        let window = create_window("destroyed before dropped");
        let safe = unsafe { SafeHWND::assert(&(!42 as HWND)).unwrap() };
        unsafe { CloseWindow(window.as_hwnd()) };
        std::mem::drop(safe);
    }

    #[test] fn should_not_panic_null() {
        let safe = unsafe { SafeHWND::assert(&std::ptr::null_mut()) };
        std::mem::drop(safe);
    }

    #[test] fn should_not_panic_outlived() {
        let window = create_window("destroyed before dropped").as_hwnd();
        let safe = unsafe { SafeHWND::assert(&window).unwrap() };
        std::mem::drop(safe);
        unsafe { CloseWindow(window) };
    }
}
