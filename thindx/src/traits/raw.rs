use std::marker::Sized;



/// Allow conversion to/from raw (winapi) pointer types.
///
/// ### ⚠️ Safety ⚠️
/// Lots of code depends on implicit invariants of this trait for soundness:
/// *   [Raw::into_raw] and [Raw::as_raw] have sane implementations.
/// *   If [Raw::Raw] is a [winapi::Interface], it too must be "sane" (correct guid, sound implementation of the COM interface in question, etc.)
/// *   Probably more I'm not thinking of
pub unsafe trait Raw : Sized {
    /// The raw underlying winapi type
    type Raw : Sized;

    /// Take ownership from a raw winapi type, panicing if `raw` is null.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `raw` must either be null or a sane/valid instance of the type in question.
    ///
    /// ### Panics
    /// *   If `raw` is null
    //#allow_missing_argument_docs
    unsafe fn from_raw(raw: *mut Self::Raw) -> Self;

    /// Take ownership from a raw winapi type, returning [None] if `raw` is null.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `raw` must either be null or a sane/valid instance of the type in question.
    //#allow_missing_argument_docs
    unsafe fn from_raw_opt(raw: *mut Self::Raw) -> Option<Self>;

    /// Give up / leak ownership into a raw winapi pointer type.
    fn into_raw(self) -> *mut Self::Raw;

    /// Allow access as a raw winapi pointer type.
    fn as_raw(&self) -> *mut Self::Raw;
}
