#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype)\]
/// D3DRESOURCETYPE
///
/// Specifies the type of a [Resource]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ResourceType(D3DRESOURCETYPE);
pub use ResourceType as RType;

enumish! { ResourceType => D3DRESOURCETYPE; Surface, Volume, Texture, VolumeTexture, CubeTexture, VertexBuffer, IndexBuffer }

#[allow(non_upper_case_globals)] impl ResourceType { // These are enum-like
    /// [Surface] resource.
    pub const Surface       : ResourceType = ResourceType(D3DRTYPE_SURFACE); // 1

    /// [Volume] resource.
    pub const Volume        : ResourceType = ResourceType(D3DRTYPE_VOLUME);

    /// [Texture] resource.
    pub const Texture       : ResourceType = ResourceType(D3DRTYPE_TEXTURE);

    /// [VolumeTexture] resource.
    pub const VolumeTexture : ResourceType = ResourceType(D3DRTYPE_VOLUMETEXTURE);

    /// [CubeTexture] resource.
    pub const CubeTexture   : ResourceType = ResourceType(D3DRTYPE_CUBETEXTURE);

    /// [VertexBuffer] resource.
    pub const VertexBuffer  : ResourceType = ResourceType(D3DRTYPE_VERTEXBUFFER);

    /// [IndexBuffer] resource.
    pub const IndexBuffer   : ResourceType = ResourceType(D3DRTYPE_INDEXBUFFER);
}

#[doc(hidden)] impl ResourceType {
    /// [Surface] resource.
    pub const SURFACE       : ResourceType = ResourceType(D3DRTYPE_SURFACE);

    /// [Volume] resource.
    pub const VOLUME        : ResourceType = ResourceType(D3DRTYPE_VOLUME);

    /// [Texture] resource.
    pub const TEXTURE       : ResourceType = ResourceType(D3DRTYPE_TEXTURE);

    /// [VolumeTexture] resource.
    pub const VOLUMETEXTURE : ResourceType = ResourceType(D3DRTYPE_VOLUMETEXTURE);

    /// [CubeTexture] resource.
    pub const CUBETEXTURE   : ResourceType = ResourceType(D3DRTYPE_CUBETEXTURE);

    /// [VertexBuffer] resource.
    pub const VERTEXBUFFER  : ResourceType = ResourceType(D3DRTYPE_VERTEXBUFFER);

    /// [IndexBuffer] resource.
    pub const INDEXBUFFER   : ResourceType = ResourceType(D3DRTYPE_INDEXBUFFER);
}

// #[cfg(feature = "impl-poor-defaults")] // XXX
impl Default for ResourceType {
    fn default() -> Self { ResourceType(0) }
}
