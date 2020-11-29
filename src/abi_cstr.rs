use std::ffi::*;



pub struct AbiCStr(());

impl<'s> From<&'s AbiCStr> for &'s CStr {
    fn from(ptr: &'s AbiCStr) -> &'s CStr {
        unsafe { CStr::from_ptr(std::mem::transmute(ptr)) }
    }
}

impl<'s> From<&'s CStr> for &'s AbiCStr {
    fn from(cstr: &'s CStr) -> &'s AbiCStr {
        unsafe { std::mem::transmute(cstr.as_ptr()) }
    }
}

#[test] fn layout() {
    use winapi::ctypes::c_char;
    use std::mem::*;

    assert_eq!(align_of::<       &AbiCStr >(), align_of::<*const c_char>());
    assert_eq!( size_of::<       &AbiCStr >(),  size_of::<*const c_char>());
    assert_eq!(align_of::<Option<&AbiCStr>>(), align_of::<*const c_char>());
    assert_eq!( size_of::<Option<&AbiCStr>>(),  size_of::<*const c_char>());
}
