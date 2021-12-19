use crate::*;
use crate::d3d::ShaderWithDiagnostics;

use std::borrow::Borrow;
use std::convert::TryInto;
use std::fmt::{self, Debug, Formatter};
use std::ops::*;
use std::slice::SliceIndex;



/// Guaranteed valid DXBC or DXIL bytecode.
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
/// [Bytecode::trust] performs limited validation that might guard against accidentally invoking UB, but not against
/// intentionally crafted "malicious" bytecode, or perhaps not even basic filesystem corruption.
///
/// [Bytecode::trust_unchecked] performs no validation, and should probably only be used if you just generated the
/// bytecode in-memory via a well tested mechanism (e.g. my d3dcompiler.dll wrappers use this.)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bytecode<T: AsRef<[u8]>>(T);

impl<T: AsRef<[u8]>> Bytecode<T> {
    /// Trust that `inner` is valid DXBC or DXIL bytecode.
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
    /// [Bytecode::trust] performs some validation (currently the length field of DXBC bytecode headers) and may perform more in the future without semver changes.
    /// This guards against some accidental UB, but maliciously crafted bytecode - or even filesystem corruption - can probably bypass validation.
    pub unsafe fn trust(inner: T) -> Result<Self, Error> {
        // http://timjones.io/blog/archive/2015/09/02/parsing-direct3d-shader-bytecode
        let bytecode = inner.as_ref();
        if bytecode.get(0..=3) == Some(b"DXBC") {
            let total_size = match bytecode.get(24..=27) {
                Some(&[a,b,c,d]) => u32::from_le_bytes([a,b,c,d]),
                _other           => return Err(Error::new("Bytecode::trust", THINERR::INVALID_BYTECODE)),
            };
            if bytecode.len() != total_size.try_into().map_err(|_| Error::new("Bytecode::trust", THINERR::INVALID_BYTECODE))? {
                return Err(Error::new("Bytecode::trust", THINERR::INVALID_BYTECODE));
            }
        }

        Ok(Self(inner))
    }

    /// Trust that `inner` is valid DXBC or DXIL bytecode.
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
    /// [Bytecode::trust_unchecked] performs no validation.  Prefer [Bytecode::trust] which at least performs some.
    pub unsafe fn trust_unchecked(inner: T) -> Self { Self(inner) }

    pub fn as_bytes(&self) -> &[u8] { self.0.as_ref() }
    pub fn bytes<'s>(&'s self) -> impl Iterator<Item = u8> + 's { self.0.as_ref().iter().copied() }
    pub fn into_inner(self) -> T { self.0 }
    pub fn as_ref(&self) -> Bytecode<&[u8]> { Bytecode(self.0.as_ref()) }
}

impl<T: AsRef<[u8]>> AsRef <[u8]> for Bytecode<T> { fn as_ref(&self) -> &[u8] { self.0.as_ref() } }
impl<T: AsRef<[u8]>> Borrow<[u8]> for Bytecode<T> { fn borrow(&self) -> &[u8] { self.0.as_ref() } }
impl<T: AsRef<[u8]>> Debug for Bytecode<T> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug::fmt(self.0.as_ref(), fmt) } }
impl<T: AsRef<[u8]>> Deref for Bytecode<T> { fn deref(&self) -> &[u8] { self.0.as_ref() } type Target = [u8]; }

impl<T: AsRef<[u8]>, I> Index<I> for Bytecode<T> where I: SliceIndex<[u8]> {
    type Output = <I as SliceIndex<[u8]>>::Output;
    fn index(&self, index: I) -> &Self::Output { self.0.as_ref().index(index) }
}



pub trait AsBytecodeRef {
    fn as_ref(&self) -> Bytecode<&[u8]>;
}

impl AsBytecodeRef for &ShaderWithDiagnostics {
    fn as_ref(&self) -> Bytecode<&[u8]> { Bytecode::as_ref(&self.shader) }
}

impl<T: AsRef<[u8]>> AsBytecodeRef for &Bytecode<T> {
    fn as_ref(&self) -> Bytecode<&[u8]> { Bytecode::as_ref(self) }
}

impl AsBytecodeRef for Bytecode<&[u8]> {
    fn as_ref(&self) -> Bytecode<&[u8]> { *self }
}
