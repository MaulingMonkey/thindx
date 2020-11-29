use winapi::ctypes::c_char;

use std::marker::PhantomData;

use std::ffi::*;
use std::ptr::*;



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CStrPtr<'s> {
    cstr:           NonNull<c_char>,
    phantom_data:   PhantomData<&'s c_char>,
}

impl<'s> From<CStrPtr<'s>> for &'s CStr {
    fn from(ptr: CStrPtr<'s>) -> &'s CStr {
        unsafe { CStr::from_ptr(ptr.cstr.as_ptr()) }
    }
}

impl<'s> From<&'s CStr> for CStrPtr<'s> {
    fn from(cstr: &'s CStr) -> CStrPtr<'s> {
        CStrPtr {
            cstr: unsafe { NonNull::new_unchecked(cstr.as_ptr() as *mut _) },
            phantom_data: PhantomData
        }
    }
}

#[test] fn layout() {
    use std::mem::*;
    assert_eq!(align_of::<CStrPtr>(), align_of::<*const c_char>());
    assert_eq!( size_of::<CStrPtr>(),  size_of::<*const c_char>());
    assert_eq!(align_of::<Option<CStrPtr>>(), align_of::<*const c_char>());
    assert_eq!( size_of::<Option<CStrPtr>>(),  size_of::<*const c_char>());
}
