use crate::*;

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;



/// Guaranteed to contain valid DXBC or DXIL bytecode.
///
/// ### Safety
///
/// Parsing and deserialization APIs are prone to undefined behavior and CVEs, and executable bytecode formats are no exception.
/// Possible consumers of this bytecode include d3dcompiler, d3d9, d3d11, d3d12, GPU drivers, various third party shader translators, etc.
/// If there's not a single path to undefined behavior between all of those, I'll eat my hat!
///
/// You could reasonably argue those are all third party bugs, and not Rust's fault.
/// I take a more conservative approach:  if Rust code wants to be sound, it should only pass valid bytecode to these existing C++ codebases.
///
/// [Bytecode::from] performs limited validation that might guard against accidentally invoking UB, but not against
/// intentionally crafted "malicious" bytecode, or perhaps not even basic filesystem corruption.
///
/// [Bytecode::from_unchecked] performs no validation, and should probably only be used if you just generated the
/// bytecode in-memory via a well tested mechanism (e.g. my d3dcompiler.dll wrappers use this.)
#[repr(transparent)]
pub struct Bytecode([u8]);

impl Bytecode {
    /// Trust that `bytecode` contains valid DXBC or DXIL bytecode.
    ///
    /// ### Safety
    ///
    /// Parsing and deserialization APIs are prone to undefined behavior and CVEs, and executable bytecode formats are no exception.
    /// Possible consumers of this bytecode include d3dcompiler, d3d9, d3d11, d3d12, GPU drivers, various third party shader translators, etc.
    /// If there's not a single path to undefined behavior between all of those, I'll eat my hat!
    ///
    /// You could reasonably argue those are all third party bugs, and not Rust's fault.
    /// I take a more conservative approach:  if Rust code wants to be sound, it should only pass valid bytecode to these existing C++ codebases.
    ///
    /// [Bytecode::from] performs some validation (currently the length field of DXBC bytecode headers) and may perform more in the future without semver changes.
    /// This guards against some accidental UB, but maliciously crafted bytecode - or even filesystem corruption - can probably bypass validation.
    #[xallow(missing_argument_docs)]
    pub unsafe fn from(bytecode: &[u8]) -> Result<&Self, Error> {
        // http://timjones.io/blog/archive/2015/09/02/parsing-direct3d-shader-bytecode
        if bytecode.get(0..=3) == Some(b"DXBC") {
            let total_size = match bytecode.get(24..=27) {
                Some(&[a,b,c,d]) => u32::from_le_bytes([a,b,c,d]),
                _other           => return Err(Error::new("Bytecode::from", THINERR::INVALID_BYTECODE)),
            };
            if bytecode.len() != total_size.try_into().map_err(|_| Error::new("Bytecode::from", THINERR::INVALID_BYTECODE))? {
                return Err(Error::new("Bytecode::from", THINERR::INVALID_BYTECODE));
            }
        }

        Ok(Self::from_unchecked(bytecode))
    }

    /// Trust that `bytecode` contains valid DXBC or DXIL bytecode.
    ///
    /// ### Safety
    ///
    /// Parsing and deserialization APIs are prone to undefined behavior and CVEs, and executable bytecode formats are no exception.
    /// Possible consumers of this bytecode include d3dcompiler, d3d9, d3d11, d3d12, GPU drivers, various third party shader translators, etc.
    /// If there's not a single path to undefined behavior between all of those, I'll eat my hat!
    ///
    /// You could reasonably argue those are all third party bugs, and not Rust's fault.
    /// I take a more conservative approach:  if Rust code wants to be sound, it should only pass valid bytecode to these existing C++ codebases.
    ///
    /// [Bytecode::from_unchecked] performs no validation.  Prefer [Bytecode::from] which at least performs some.
    #[xallow(missing_argument_docs)]
    pub unsafe fn from_unchecked(bytecode: &[u8]) -> &Self { std::mem::transmute(bytecode) }

    /// Get the bytecode as a slice of bytes.
    pub fn as_bytes(&self) -> &[u8] { self.0.as_ref() }

    /// Get the bytecode as an iterator of bytes.
    pub fn bytes<'s>(&'s self) -> impl Iterator<Item = u8> + 's { self.0.as_ref().iter().copied() }
}

impl AsRef <[u8]> for Bytecode { fn as_ref(&self) -> &[u8] { self.0.as_ref() } }
impl Borrow<[u8]> for Bytecode { fn borrow(&self) -> &[u8] { self.0.as_ref() } }
impl Debug for Bytecode { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.0.as_ref(), fmt) } }
impl Deref for Bytecode { fn deref(&self) -> &[u8] { self.0.as_ref() } type Target = [u8]; }
