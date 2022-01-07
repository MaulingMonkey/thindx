pub use ::abibool::bool32   as BOOL;
pub use ::abibool::bool8    as BOOLEAN;
pub(crate) use ::abistr::{AsCStr, AsOptCStr};
pub use ::abistr::{CStrPtr, CStrNonNull};

/// [str], [String], [CStr](std::ffi::CStr), [CString](std::ffi::CString), [abistr::CStrNonNull], [abistr::CStrPtr], or `()` (null)
///
/// [abistr::TryIntoAsOptCStr] under the hood.
///
pub use abistr::TryIntoAsOptCStr as OptPCSTR;

/// [str], [String], [CStr](std::ffi::CStr), [CString](std::ffi::CString), [abistr::CStrNonNull]
///
/// [abistr::TryIntoAsOptCStr] under the hood.
///
pub use abistr::TryIntoAsCStr as PCSTR;
