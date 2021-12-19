use crate::d3d::{Bytecode, OptTextBlob, ReadOnlyBlob};

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::*;
use std::str::Utf8Error;



#[derive(Clone, Default)]
pub struct CompilerDiagnostics(OptTextBlob);

impl CompilerDiagnostics {
    pub fn new(value: impl Into<Self>) -> Self { value.into() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn to_utf8_lossy(&self) -> Cow<str> { self.0.to_utf8_lossy() }
    pub fn to_utf8(&self) -> Result<&str, Utf8Error> { self.0.to_utf8() }
    // TODO: parsing diagnostics iterator?
}

impl Debug   for CompilerDiagnostics { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "CompilerDiagnostics({:?})", self.to_utf8_lossy()) } }
impl Display for CompilerDiagnostics { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.to_utf8_lossy(), fmt) } }

impl From<Option<ReadOnlyBlob>> for CompilerDiagnostics { fn from(value: Option<ReadOnlyBlob>) -> Self { Self(OptTextBlob::new(     value )) } }
impl From<       ReadOnlyBlob > for CompilerDiagnostics { fn from(value:        ReadOnlyBlob ) -> Self { Self(OptTextBlob::new(Some(value))) } }



#[derive(Clone)]
pub struct ShaderWithDiagnostics {
    pub shader:         Bytecode<ReadOnlyBlob>,
    pub diagnostics:    CompilerDiagnostics,
}

impl ShaderWithDiagnostics {
    pub fn new(shader: Bytecode<ReadOnlyBlob>, diagnostics: CompilerDiagnostics) -> Self {
        Self { shader, diagnostics }
    }
}

impl AsRef<[u8]>                   for ShaderWithDiagnostics { fn as_ref(&    self) -> &    [u8]                   { self.shader.as_bytes() } }
impl AsRef<Bytecode<ReadOnlyBlob>> for ShaderWithDiagnostics { fn as_ref(&    self) -> &    Bytecode<ReadOnlyBlob> { &    self.shader } }
impl AsMut<Bytecode<ReadOnlyBlob>> for ShaderWithDiagnostics { fn as_mut(&mut self) -> &mut Bytecode<ReadOnlyBlob> { &mut self.shader } }

impl Debug for ShaderWithDiagnostics {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WithCompilerDiagnostics")
            .field("shader",        &self.shader)
            .field("diagnostics",   &self.diagnostics)
            .finish()
    }
}

impl Deref for ShaderWithDiagnostics {
    type Target = Bytecode<ReadOnlyBlob>;
    fn deref(&self) -> &Self::Target { &self.shader }
}

impl DerefMut for ShaderWithDiagnostics {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.shader }
}



#[derive(Clone, Default)]
pub struct WithCompilerDiagnostics<T> {
    value_with_weird_name_to_avoid_deref_conflicts: T,
    pub diagnostics: CompilerDiagnostics,
}

impl<T> WithCompilerDiagnostics<T> {
    pub fn new(value: T, diagnostics: CompilerDiagnostics) -> Self {
        Self {
            value_with_weird_name_to_avoid_deref_conflicts: value,
            diagnostics,
        }
    }
}

impl<T: AsRef<U>, U : ?Sized> AsRef<U> for WithCompilerDiagnostics<T> { fn as_ref(&    self) -> &    U { self.value_with_weird_name_to_avoid_deref_conflicts.as_ref() } }
impl<T: AsMut<U>, U : ?Sized> AsMut<U> for WithCompilerDiagnostics<T> { fn as_mut(&mut self) -> &mut U { self.value_with_weird_name_to_avoid_deref_conflicts.as_mut() } }

impl<T: Debug> Debug for WithCompilerDiagnostics<T> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("WithCompilerDiagnostics")
            .field("value", &self.value_with_weird_name_to_avoid_deref_conflicts)
            .field("diagnostics", &self.diagnostics)
            .finish()
    }
}

impl<T> Deref for WithCompilerDiagnostics<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.value_with_weird_name_to_avoid_deref_conflicts }
}

impl<T> DerefMut for WithCompilerDiagnostics<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value_with_weird_name_to_avoid_deref_conflicts }
}
