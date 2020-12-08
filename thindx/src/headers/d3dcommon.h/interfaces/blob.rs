use crate::*;

use winapi::um::d3dcommon::ID3DBlob;

use std::borrow::{Borrow, Cow};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Index;
use std::slice::SliceIndex;
use std::str::Utf8Error;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))\]
/// ID3DBlob
///
/// This interface is used to return arbitrary-length data.
///
/// ### Safety
///
/// This assumes `ReadOnlyBlob`s are read-only once created.
/// While enforced by the safe interface here, raw `unsafe` winapi can totally let you write to the buffer.
/// This is your one warning!
#[derive(Clone)] #[repr(transparent)]
pub struct ReadOnlyBlob(pub(crate) mcom::Rc<ID3DBlob>);

convert!(unsafe ReadOnlyBlob => Unknown, winapi::um::d3dcommon::ID3DBlob);

impl ReadOnlyBlob {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728745(v=vs.85))\]
    /// ID3DBlob::GetBufferSize
    ///
    /// Gets the size of the buffer.
    pub fn get_buffer_size(&self) -> usize {
        unsafe { self.0.GetBufferSize() }
    }

    /// [ID3DBlob::GetBufferPointer](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728744(v=vs.85)) +
    /// [ID3DBlob::GetBufferSize](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728745(v=vs.85))
    ///
    /// Gets the data of the buffer as a readonly slice.
    pub fn get_buffer(&self) -> &[u8] {
        let size = self.get_buffer_size();
        let ptr = unsafe { self.0.GetBufferPointer() } as *const u8;
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.get_buffer()
    }

    pub fn bytes<'s>(&'s self) -> impl Iterator<Item = u8> + 's {
        self.get_buffer().iter().copied()
    }
}

impl AsRef <[u8]> for ReadOnlyBlob { fn as_ref(&self) -> &[u8] { self.get_buffer() } }
impl Borrow<[u8]> for ReadOnlyBlob { fn borrow(&self) -> &[u8] { self.get_buffer() } }
impl Debug        for ReadOnlyBlob { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "ReadOnlyBlob({} bytes)", self.get_buffer_size()) } }
impl<I> Index<I>  for ReadOnlyBlob where I: SliceIndex<[u8]> { fn index(&self, index: I) -> &Self::Output { self.get_buffer().index(index) } type Output = I::Output; }

// can't impl Deref - conflicting impl with COM nonsense

// Index?
// IntoIterator?
// [Partial]Ord/[Partial]Eq? (nah - ambiguous between bytes vs com instances)
// Hash? (nah - might want to allow Borrow<[u32]>?)

// From?  Would require manual ID3DBlob impls in case d3dcompiler.dll isn't available/loaded?

#[test] fn layout() {
    use std::mem::*;
    assert_eq!(align_of::<Option<ReadOnlyBlob>>(), align_of::<*mut ID3DBlob>());
    assert_eq!(size_of::< Option<ReadOnlyBlob>>(), size_of::< *mut ID3DBlob>());
}



/// [ReadOnlyBlob] wrapper for `\0`-terminated UTF8ish data
#[derive(Clone, Default)]
pub struct TextBlob(Option<ReadOnlyBlob>);

impl TextBlob {
    pub fn new(value: impl Into<Self>) -> Self { value.into() }
    pub fn is_empty(&self) -> bool { self.0.as_ref().map_or(true, |blob| blob.get_buffer_size() == 0) }
    pub fn to_utf8_lossy(&self) -> Cow<str> { String::from_utf8_lossy(self.as_bytes()) }
    pub fn to_utf8(&self) -> Result<&str, Utf8Error> { std::str::from_utf8(self.as_bytes()) }
    // TODO: parsing diagnostics iterator?
}

impl Debug   for TextBlob { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "TextBlob({:?})", self.to_utf8_lossy()) } }
impl Display for TextBlob { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.to_utf8_lossy(), fmt) } }

impl From<Option<ReadOnlyBlob>> for TextBlob { fn from(value: Option<ReadOnlyBlob>) -> Self { Self(     value ) } }
impl From<       ReadOnlyBlob > for TextBlob { fn from(value:        ReadOnlyBlob ) -> Self { Self(Some(value)) } }

impl TextBlob {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_ref().map_or(&[], |blob| {
            let b = blob.get_buffer();
            if b.last() == Some(&0) {
                &b[..b.len()-1]
            } else {
                b
            }
        })
    }
}
