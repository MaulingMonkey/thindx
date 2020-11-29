use winapi::um::d3dcommon::ID3DBlob;



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
}

#[test] fn layout() {
    use std::mem::*;
    assert_eq!(align_of::<Option<ReadOnlyBlob>>(), align_of::<*mut ID3DBlob>());
    assert_eq!(size_of::< Option<ReadOnlyBlob>>(), size_of::< *mut ID3DBlob>());
}
