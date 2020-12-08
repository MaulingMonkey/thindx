use crate::*;

use winapi::um::d3dcommon::ID3DBlob;

use std::borrow::{Borrow, Cow};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, Index};
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
#[derive(Clone)]
#[repr(transparent)]
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



/// [ReadOnlyBlob] wrapper for DXBC or DXIL bytecode.
#[derive(Clone)]
#[repr(transparent)]
pub struct CodeBlob(ReadOnlyBlob);

impl CodeBlob {
    /// Create a new [CodeBlob] from `value`, assuming it contains valid DXBC or DXIL bytecode.
    ///
    /// ### Safety
    ///
    /// By calling this fn, you assert that `value` contains valid DXBC or DXIL bytecode.
    /// While it doesn't result in instant undefined behavior to violate this constraint, many "safe" fns rely on this constraint, and will exhibit undefined behavior if passed an invalid blob.
    ///
    /// Parsing and deserialization APIs are prone to undefined behavior and CVEs, and executable bytecode formats are no exception.
    /// Possible consumers of this bytecode include d3dcompiler, d3d9, d3d11, d3d12, GPU drivers, various third party shader translators, etc.
    /// If there's not a single path to undefined behavior between all of those, I'll eat my hat!
    pub(crate) unsafe fn from_unchecked(value: ReadOnlyBlob) -> Self { Self(value) }

    pub fn len(&self)      -> usize { self.0.get_buffer_size() }
    pub fn is_empty(&self) -> bool  { self.0.get_buffer_size() == 0 }
    pub fn get_buffer(&self) -> &[u8] { self.0.get_buffer() } // TODO: remove
    pub fn as_bytes(&self) -> &[u8] { self.0.get_buffer() }
    pub fn as_bytecode(&self) -> &d3d::Bytecode { unsafe { d3d::Bytecode::from_unchecked(self.as_bytes()) } }
}

impl AsRef <[u8]> for CodeBlob { fn as_ref(&self) -> &[u8] { self.as_bytes() } }
impl Borrow<[u8]> for CodeBlob { fn borrow(&self) -> &[u8] { self.as_bytes() } }
impl AsRef <d3d::Bytecode> for CodeBlob { fn as_ref(&self) -> &d3d::Bytecode { self.as_bytecode() } }
impl Borrow<d3d::Bytecode> for CodeBlob { fn borrow(&self) -> &d3d::Bytecode { self.as_bytecode() } }
impl Debug for CodeBlob { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "CodeBlob({} bytes)", self.len()) } }
impl Deref for CodeBlob { fn deref(&self) -> &d3d::Bytecode { self.as_bytecode() } type Target = d3d::Bytecode; }

// DO NOT IMPLEMENT:
// From<Option<ReadOnlyBlob>> for TextBlob - "safe" bypass of bytecode validation
// From<       ReadOnlyBlob > for TextBlob - "safe" bypass of bytecode validation
// From<&[u8]>                for TextBlob - "safe" bypass of bytecode validation




/// [ReadOnlyBlob] wrapper for other binary data
#[derive(Clone)]
#[repr(transparent)]
pub struct BytesBlob(Option<ReadOnlyBlob>);

impl BytesBlob {
    pub fn new(value: impl Into<Self>) -> Self { value.into() }

    pub fn len(&self)      -> usize   { self.0.as_ref().map_or(0,    |blob| blob.get_buffer_size()) }
    pub fn is_empty(&self) -> bool    { self.0.as_ref().map_or(true, |blob| blob.get_buffer_size() == 0) }
    pub fn get_buffer(&self) -> &[u8] { self.0.as_ref().map_or(&[],  |blob| blob.get_buffer()) } // TODO: remove
    pub fn as_bytes(&self)   -> &[u8] { self.0.as_ref().map_or(&[],  |blob| blob.get_buffer()) }
}

impl AsRef <[u8]> for BytesBlob { fn as_ref(&self) -> &[u8] { self.as_bytes() } }
impl Borrow<[u8]> for BytesBlob { fn borrow(&self) -> &[u8] { self.as_bytes() } }
impl Debug for BytesBlob { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "BytesBlob({} bytes)", self.len()) } }
impl Deref for BytesBlob { fn deref(&self) -> &[u8] { self.as_bytes() } type Target = [u8]; }
impl From<Option<ReadOnlyBlob>> for BytesBlob { fn from(value: Option<ReadOnlyBlob>) -> Self { Self(     value ) } }
impl From<       ReadOnlyBlob > for BytesBlob { fn from(value:        ReadOnlyBlob ) -> Self { Self(Some(value)) } }
