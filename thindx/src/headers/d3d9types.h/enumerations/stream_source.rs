#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;
type D3DSTREAMSOURCE = u32;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/efficiently-drawing-multiple-instances-of-geometry)\]
/// D3DSTREAMSOURCE
///
/// Used for specifying which streams should use instanced rendering.
///
/// See [Efficiently Drawing Multiple Instances of Geometry (Direct3D 9)] for details.
///
/// [Efficiently Drawing Multiple Instances of Geometry (Direct3D 9)]:  https://docs.microsoft.com/en-us/windows/win32/direct3d9/efficiently-drawing-multiple-instances-of-geometry
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct StreamSource(D3DSTREAMSOURCE);

impl StreamSource {
    /// Convert a raw [winapi] `D3DSTREAMSOURCE_*` into a [StreamSource].  This is *probably* safe... probably...
    pub const fn from_unchecked(resource_type: D3DSTREAMSOURCE) -> Self { Self(resource_type) }

    /// Convert a [StreamSource] into a raw [winapi] `D3DSTREAMSOURCE_*`.
    pub const fn into(self) -> D3DSTREAMSOURCE { self.0 }
}

#[allow(non_upper_case_globals)] impl StreamSource { // These are enum-like
    /// A regular, non-instanced vertex data stream
    pub fn regular() -> Self { Self(1) }

    /// `D3DSTREAMSOURCE_INDEXEDDATA | instances_to_draw`
    ///
    /// Only valid on stream 0.  Indicates that this stream contains the base geometry to (re)use between instances, and the number of instances to draw.
    //#allow_missing_argument_docs
    pub fn indexed_data(instances_to_draw: u32) -> Self {
        assert!(instances_to_draw & 0xC000_0000 == 0, "instances_to_draw infringing on D3DSTREAMSOURCE_* flags");
        Self(D3DSTREAMSOURCE_INDEXEDDATA | instances_to_draw)
    }

    /// `D3DSTREAMSOURCE_INSTANCEDATA | unknown`
    ///
    /// This stream contains data that varies per-instance.
    /// The only known valid combination, currently, is `StreamSource::instance_data(1)`.
    //#allow_missing_argument_docs
    pub fn instance_data(unknown: u32) -> Self {
        assert_eq!(unknown, 1);
        Self(D3DSTREAMSOURCE_INSTANCEDATA | unknown)
    }
}

impl Default for StreamSource {
    fn default() -> Self { StreamSource::regular() }
}

impl Debug for StreamSource {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if        self.0 == 1                                           { write!(f, "StreamSource::Regular") }
        else if self.0 & 0xC000_0000 == 0                               { write!(f, "StreamSource({})", self.0) }
        else if self.0 & 0xC000_0000 == D3DSTREAMSOURCE_INDEXEDDATA     { write!(f, "StreamSource::indexed_data(instances = {})",   self.0 & !0xC000_0000) }
        else if self.0 & 0xC000_0000 == D3DSTREAMSOURCE_INSTANCEDATA    { write!(f, "StreamSource::instance_data({})",              self.0 & !0xC000_0000) }
        else                                                            { write!(f, "StreamSource::???_data({})",                   self.0 & !0xC000_0000) }
    }
}

impl From<StreamSource> for D3DSTREAMSOURCE {
    fn from(value: StreamSource) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DSTREAMSOURCE> for StreamSource {
    fn from(value: D3DSTREAMSOURCE) -> Self { Self(value) }
}
