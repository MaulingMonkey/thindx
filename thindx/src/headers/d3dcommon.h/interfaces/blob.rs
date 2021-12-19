use crate::*;

use winapi::um::d3dcommon::ID3DBlob;

use std::borrow::{Borrow, Cow};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Index;
use std::slice::SliceIndex;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ff728743(v=vs.85))\]
/// ID3DBlob
///
/// This interface is used to return arbitrary-length data.
///
/// ### Safety
///
/// This assumes `Blob`s are read-only once created.
/// While enforced by the safe interface here, raw `unsafe` winapi can totally let you write to the buffer.
/// This is your one warning!
#[derive(Clone)] #[repr(transparent)]
pub struct Blob(pub(crate) mcom::Rc<ID3DBlob>);

convert!(unsafe Blob => Unknown, winapi::um::d3dcommon::ID3DBlob);

impl Blob {
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

impl AsRef <[u8]> for Blob { fn as_ref(&self) -> &[u8] { self.get_buffer() } }
impl Borrow<[u8]> for Blob { fn borrow(&self) -> &[u8] { self.get_buffer() } }
impl Debug        for Blob { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "Blob({} bytes)", self.get_buffer_size()) } }
impl<I> Index<I>  for Blob where I: SliceIndex<[u8]> { fn index(&self, index: I) -> &Self::Output { self.get_buffer().index(index) } type Output = I::Output; }

// can't impl Deref - conflicting impl with COM nonsense

// Index?
// IntoIterator?
// [Partial]Ord/[Partial]Eq? (nah - ambiguous between bytes vs com instances)
// Hash? (nah - might want to allow Borrow<[u32]>?)

// From?  Would require manual ID3DBlob impls in case d3dcompiler.dll isn't available/loaded?

#[test] fn layout() {
    use std::mem::*;
    assert_eq!(align_of::<Option<Blob>>(), align_of::<*mut ID3DBlob>());
    assert_eq!(size_of::< Option<Blob>>(), size_of::< *mut ID3DBlob>());
}



#[derive(Clone)]
#[repr(transparent)]
pub struct TextBlob(pub(crate) Blob);

impl TextBlob {
    // TODO: make public
    pub(crate) fn as_bytes(&self) -> &[u8] {
        let b = self.0.get_buffer();
        if b.is_empty() { b } else { &b[..b.len()-1] }
    }

    pub fn to_utf8_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.as_bytes())
    }
}

impl AsRef <[u8]> for TextBlob { fn as_ref(&self) -> &[u8] { self.as_bytes() } }
impl Borrow<[u8]> for TextBlob { fn borrow(&self) -> &[u8] { self.as_bytes() } }
impl Debug        for TextBlob { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Debug  ::fmt(&self.to_utf8_lossy(), fmt) } }
impl Display      for TextBlob { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.to_utf8_lossy(), fmt) } }
impl From< TextBlob> for String { fn from(value:  TextBlob) -> Self { value.to_utf8_lossy().into_owned() } }
impl From<&TextBlob> for String { fn from(value: &TextBlob) -> Self { value.to_utf8_lossy().into_owned() } }

impl<I> Index<I> for TextBlob where I: SliceIndex<[u8]> + SliceIndex<str> {
    type Output = <I as SliceIndex<[u8]>>::Output;
    fn index(&self, index: I) -> &Self::Output { self.as_bytes().index(index) }
}

impl<'v> TryFrom<&'v TextBlob> for &'v str {
    type Error = std::str::Utf8Error;
    fn try_from(value: &'v TextBlob) -> Result<Self, Self::Error> { std::str::from_utf8(value.as_bytes()) }
}

unsafe impl Raw for TextBlob {
    type Raw = <Blob as Raw>::Raw;
    unsafe fn from_raw(raw: *mut Self::Raw) -> Self { Self(Blob::from_raw(raw)) }
    unsafe fn from_raw_opt(raw: *mut Self::Raw) -> Option<Self> { Blob::from_raw_opt(raw).map(Self) }
    fn into_raw(self) -> *mut Self::Raw { self.0.into_raw() }
    fn as_raw(&self) -> *mut Self::Raw { self.0.as_raw() }
}
