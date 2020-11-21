#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype)\]
/// D3DRESOURCETYPE
/// Specifies the type of a [Resource]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ResourceType(D3DRESOURCETYPE);

impl ResourceType {
    /// Convert a raw [D3DRESOURCETYPE] into a [ResourceType].  This is *probably* safe... probably...
    ///
    /// [D3DRESOURCETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype
    pub const fn from_unchecked(resource_type: D3DRESOURCETYPE) -> Self { Self(resource_type) }

    /// Convert a [ResourceType] into a raw [D3DRESOURCETYPE].
    ///
    /// [D3DRESOURCETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcetype
    pub const fn into(self) -> D3DRESOURCETYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl ResourceType {
    /// [Surface] resource.
    pub const Surface       : ResourceType = ResourceType(D3DRTYPE_SURFACE);

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

#[cfg(feature = "impl-poor-defaults")]
impl Default for ResourceType {
    fn default() -> Self { ResourceType::from_unchecked(0) }
}

impl Debug for ResourceType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ResourceType::Surface       => write!(f, "ResourceType::Surface"),
            ResourceType::Volume        => write!(f, "ResourceType::Volume"),
            ResourceType::Texture       => write!(f, "ResourceType::Texture"),
            ResourceType::VolumeTexture => write!(f, "ResourceType::VolumeTexture"),
            ResourceType::CubeTexture   => write!(f, "ResourceType::CubeTexture"),
            ResourceType::VertexBuffer  => write!(f, "ResourceType::VertexBuffer"),
            ResourceType::IndexBuffer   => write!(f, "ResourceType::IndexBuffer"),
            other                       => write!(f, "ResourceType({})", other.0),
        }
    }
}

impl From<ResourceType> for D3DRESOURCETYPE {
    fn from(value: ResourceType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DRESOURCETYPE> for ResourceType {
    fn from(value: D3DRESOURCETYPE) -> Self { Self(value) }
}
