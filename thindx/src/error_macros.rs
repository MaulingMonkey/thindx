
macro_rules! fn_err         { ( $kind:expr ) => {    Err($crate::Error(_THINDX_FN_CONTEXT, $kind)) }}
macro_rules! fn_error       { ( $kind:expr ) => {        $crate::Error(_THINDX_FN_CONTEXT, $kind)  }}
macro_rules! fn_check_hr    { ( $hr:expr )   => { $crate::Error::check(_THINDX_FN_CONTEXT, $hr)    }}
// TODO: collapse fn_check_hr! spam
macro_rules! fn_param_error { ( $param:ident, $kind:expr ) => { $crate::Error(_THINDX_FN_CONTEXT, ($kind).into()) } }
// TODO: use $param in error payload
// TODO: audit fn_param_error! for consistency

/// Annotate a Rust => C++ function mapping.
///
/// This is an alternative to `//#cpp2rust Cpp = Rust` comments, and also provides a constant for other `fn_*!` macros to read.
///
/// ### Usage
/// ```
/// fn_context!(Rust::method => ICpp::Method);
/// ```
macro_rules! fn_context {
    ( $thindx:path ) => {
        const _THINDX_FN_CONTEXT : &'static $crate::error_macros::FnContext = &$crate::error_macros::FnContext { // Ensure it's evaluated at compile time
            directx_method: None,
            thindx_method:  stringify!($thindx),

            module_path:    std::module_path!(),
            file:           std::file!(),
            line:           std::line!(),
            column:         std::column!(),
        };
    };
    ( $thindx:path => $directx:path ) => {
        const _THINDX_FN_CONTEXT : &'static $crate::error_macros::FnContext = &$crate::error_macros::FnContext { // Ensure it's evaluated at compile time
            directx_method: Some(stringify!($directx)),
            thindx_method:  stringify!($thindx),

            module_path:    std::module_path!(),
            file:           std::file!(),
            line:           std::line!(),
            column:         std::column!(),
        };
    };
}

#[allow(dead_code)] // not all fields yet used
pub(crate) struct FnContext {
    pub directx_method: Option<&'static str>,
    pub thindx_method:  &'static str,

    pub module_path:    &'static str,
    pub file:           &'static str,
    pub line:           u32,
    pub column:         u32,
}



#[test] fn asdf() {
    fn_context! { thindx::method => Method };
}
