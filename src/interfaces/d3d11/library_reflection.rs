/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11libraryreflection)\]
/// ID3D11LibraryReflection
///
/// A library-reflection interface accesses library info.
#[derive(Clone)] #[repr(transparent)]
pub struct LibraryReflection(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11LibraryReflection>);
