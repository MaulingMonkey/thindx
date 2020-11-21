use std::marker::Sized;

pub unsafe trait Raw : Sized {
    type Raw : Sized;

    unsafe fn from_raw(raw: *mut Self::Raw) -> Self;
    fn into_raw(self) -> *mut Self::Raw;
    fn as_raw(&self) -> *mut Self::Raw;
}
