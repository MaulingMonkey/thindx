#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype)\]
/// D3DRESOURCETYPE
///
/// Specifies the type of a [Resource]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
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

impl ResourceType {
    pub const fn zeroed() -> Self { Self(0) }
}

//#cpp2rust D3DRESOURCETYPE         = d3d::ResourceType
//#cpp2rust D3DRTYPE_SURFACE        = d3d::RType::Surface
//#cpp2rust D3DRTYPE_VOLUME         = d3d::RType::Volume
//#cpp2rust D3DRTYPE_TEXTURE        = d3d::RType::Texture
//#cpp2rust D3DRTYPE_VOLUMETEXTURE  = d3d::RType::VolumeTexture
//#cpp2rust D3DRTYPE_CUBETEXTURE    = d3d::RType::CubeTexture
//#cpp2rust D3DRTYPE_VERTEXBUFFER   = d3d::RType::VertexBuffer
//#cpp2rust D3DRTYPE_INDEXBUFFER    = d3d::RType::IndexBuffer
