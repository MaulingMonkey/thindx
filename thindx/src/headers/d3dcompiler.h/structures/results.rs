use crate::*;
use crate::d3d::*;

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};



/// { shader: [Bytecode]\<[Blob]\>, errors: Option&lt;[TextBlob]&gt; } returned by [Compiler::compile]/[compile2](Compiler::compile2)
pub struct Compiled {
    pub shader:     Bytecode<Blob>,
    pub errors:     Option<TextBlob>,
}

/// { shader: [Bytecode]\<[Blob]\>, errors: Option&lt;[Bytecode]\<[Blob]\>&gt; } returned by [d3d11::Linker::link]
pub type Linked = Compiled;


/// { shader: [TextBlob], errors: Option&lt;[TextBlob]&gt; } returned by [Compiler::preprocess]
pub struct Preprocessed {
    pub shader: TextBlob,
    pub errors: Option<TextBlob>,
}


pub type CompileError       = BuildError<Bytecode<Blob>>;
pub type LinkError          = BuildError<Bytecode<Blob>>;
pub type PreprocessError    = BuildError<TextBlob>;



/// { kind: [ErrorKind], method: Option&lt;&amp;'static str&gt;, shader: Option&lt;S&gt;, errors: Option&lt;[Blob]&gt; }
pub struct BuildError<S> {
    pub kind:       ErrorKind,
    pub method:     Option<&'static str>,
    pub shader:     Option<S>, // may or may not have generated an output despite errors
    pub errors:     Option<TextBlob>, // may or may not have generated error messages depending on the error kind
}

impl<S> BuildError<S> {
    pub fn errors_utf8_lossy(&self) -> Option<Cow<str>> {
        Some(self.errors.as_ref()?.to_utf8_lossy())
    }
}

impl<S> From<Error> for BuildError<S> {
    fn from(e: Error) -> Self {
        let Error { kind, method, errors } = e;
        Self { kind, method, errors, shader: None }
    }
}

impl<S> std::error::Error for BuildError<S> {}

impl<S> Debug for BuildError<S> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("BuildError")
            .field("kind", &self.kind)
            .field("shader", &self.shader.as_ref().map(|_| ..))
            .field("errors", &self.errors_utf8_lossy())
            .finish()
    }
}

impl<S> Display for BuildError<S> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "Error compiling shader: {:?}", self.kind)?;
        if let Some(errors) = self.errors_utf8_lossy() {
            writeln!(fmt, "\n{}", errors)?;
        }
        Ok(())
    }
}
