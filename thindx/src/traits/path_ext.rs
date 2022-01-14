use crate::*;
use std::os::windows::ffi::*;
use std::path::Path;



impl<T: AsRef<Path>> PathExt for T {}
pub(crate) trait PathExt : AsRef<Path> {
    /// Returns a `\0`-terminated `Vec<u16>` containing no interior `\0`s.
    fn to_wcstr(&self, method: &'static str) -> Result<Vec<u16>, MethodError> {
        let path = self.as_ref().as_os_str().encode_wide().chain(Some(0)).collect::<Vec<_>>();
        if path[..path.len()-1].contains(&0) {
            Err(MethodError(method, THINERR::STRING_CONTAINS_NULS))
        } else {
            Ok(path)
        }
    }
}
