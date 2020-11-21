use crate::*;

mod direct3d;   pub use direct3d::Direct3D;
mod direct3dex;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)\]
/// Base COM interface all D3D9 types eventually derive from.
#[derive(Clone)] #[repr(transparent)]
pub struct Unknown(mcom::Rc<winapi::um::unknwnbase::IUnknown>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3d9ex)\]
/// (extends [Direct3D])
/// Factory for use in creating your initial [DeviceEx].
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct Direct3DEx(mcom::Rc<winapi::shared::d3d9::IDirect3D9Ex>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dbasetexture9)\]
/// (extends [Resource])
/// [Texture], [CubeTexture], or [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct BaseTexture(mcom::Rc<winapi::shared::d3d9::IDirect3DBaseTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dcubetexture9)\]
/// (extends [BaseTexture])
/// 6-faced 2D texture for use with [cube mapping](https://en.wikipedia.org/wiki/Cube_mapping)
#[derive(Clone)] #[repr(transparent)]
pub struct CubeTexture(mcom::Rc<winapi::shared::d3d9::IDirect3DCubeTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9)\]
/// Core interface used for general rendering, resource creation, etc.
#[derive(Clone)] #[repr(transparent)]
pub struct Device(mcom::Rc<winapi::shared::d3d9::IDirect3DDevice9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9ex)\]
/// (extends [Device])
/// Core interface used for general rendering, resource creation, etc.
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct DeviceEx(mcom::Rc<winapi::shared::d3d9::IDirect3DDevice9Ex>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dindexbuffer9)\]
/// (extends [Resource])
/// An [index buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing)
/// indexes verticies in a [VertexBuffer] when rendering.
#[derive(Clone)] #[repr(transparent)]
pub struct IndexBuffer(mcom::Rc<winapi::shared::d3d9::IDirect3DIndexBuffer9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dpixelshader9)\]
/// A [pixel/fragment shader](https://en.wikipedia.org/wiki/Shader#Pixel_shaders) is a GPU program, run on rasterized fragments.
#[derive(Clone)] #[repr(transparent)]
pub struct PixelShader(mcom::Rc<winapi::shared::d3d9::IDirect3DPixelShader9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dquery9)\]
/// An asyncronous GPU query for [occlusion or other information](https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries).
#[derive(Clone)] #[repr(transparent)]
pub struct Query(mcom::Rc<winapi::shared::d3d9::IDirect3DQuery9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dresource9)\]
/// [\*Texture\*](crate::BaseTexture), [Surface] (but not <strike>[Volume]</strike>!), [IndexBuffer], [VertexBuffer], but not <strike>[\*Shader](crate::PixelShader)</strike>!
#[derive(Clone)] #[repr(transparent)]
pub struct Resource(mcom::Rc<winapi::shared::d3d9::IDirect3DResource9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dstateblock9)\]
/// Used to [capture/save and restore](https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state)
/// changes to [Device] state.
#[derive(Clone)] #[repr(transparent)]
pub struct StateBlock(mcom::Rc<winapi::shared::d3d9::IDirect3DStateBlock9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dsurface9)\]
/// (extends [Resource])
/// A dense 2-dimensional region of data, often belonging to a [Texture]
#[derive(Clone)] #[repr(transparent)]
pub struct Surface(mcom::Rc<winapi::shared::d3d9::IDirect3DSurface9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9)\]
/// Manages swapping buffers for a view.
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChain(mcom::Rc<winapi::shared::d3d9::IDirect3DSwapChain9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dswapchain9ex)\]
/// (extends [SwapChain])
/// Adds more querying options.
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct SwapChainEx(mcom::Rc<winapi::shared::d3d9::IDirect3DSwapChain9Ex>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dtexture9)\]
/// (extends [BaseTexture])
/// A dense 2-dimensional set of "pixels"
#[derive(Clone)] #[repr(transparent)]
pub struct Texture(mcom::Rc<winapi::shared::d3d9::IDirect3DTexture9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexbuffer9)\]
/// (extends [Resource])
/// A [vertex buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing)
/// typically contains points of a mesh to be rendered.
#[derive(Clone)] #[repr(transparent)]
pub struct VertexBuffer(mcom::Rc<winapi::shared::d3d9::IDirect3DVertexBuffer9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexdeclaration9)\]
/// Describes the layout of the contents of a [VertexBuffer]
#[derive(Clone)] #[repr(transparent)]
pub struct VertexDeclaration(mcom::Rc<winapi::shared::d3d9::IDirect3DVertexDeclaration9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvertexshader9)\]
/// A [vertex shader](https://en.wikipedia.org/wiki/Shader#Vertex_shaders) transforms mesh verticies when rendering.
#[derive(Clone)] #[repr(transparent)]
pub struct VertexShader(mcom::Rc<winapi::shared::d3d9::IDirect3DVertexShader9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9)\]
/// A dense 3-dimensional region of data, often belonging to a [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct Volume(mcom::Rc<winapi::shared::d3d9::IDirect3DVolume9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolumetexture9)\]
/// (extends [BaseTexture])
/// A dense 3-dimensional set of "pixels"
#[derive(Clone)] #[repr(transparent)]
pub struct VolumeTexture(mcom::Rc<winapi::shared::d3d9::IDirect3DVolumeTexture9>);



/// ### Safety
///
/// * Assumes `$outer` is `#[repr(transparent)]`
/// * Typechecked via some `From` impls, but sufficiently malicious `Deref` impls might be able to defeat that.
macro_rules! convert {
    ( unsafe $outer:ty => $deref:ty, $winapi:ty ) => {
        convert!(unsafe $outer, $winapi);

        impl std::ops::Deref for $outer {
            type Target = $deref;
            fn deref(&self) -> &Self::Target { self.0.up_ref().into() }
        }
    };
    ( unsafe $outer:ty, $winapi:ty ) => {
        impl From<mcom::Rc<$winapi>> for $outer { fn from(value: mcom::Rc<$winapi>) -> Self { Self(value) } }
        impl From<$outer> for mcom::Rc<$winapi> { fn from(value: $outer) -> Self { value.0 } }

        impl From<&mcom::Rc<$winapi>> for &$outer { fn from(value: &mcom::Rc<$winapi>) -> Self { unsafe { std::mem::transmute(value) } } }
        impl From<&$outer> for &mcom::Rc<$winapi> { fn from(value: &$outer) -> Self { unsafe { std::mem::transmute(value) } } }

        unsafe impl traits::Raw for $outer {
            type Raw = $winapi;

            unsafe fn from_raw(raw: *mut Self::Raw) -> Self { Self(mcom::Rc::from_raw(raw)) }
            fn into_raw(self) -> *mut Self::Raw { self.0.into_raw() }
            fn as_raw(&self) -> *mut Self::Raw { self.0.as_ptr() }
        }
    };
}

// Misc
convert!(unsafe Unknown,                            winapi::um::unknwnbase::IUnknown);
convert!(unsafe Query               => Unknown,     winapi::shared::d3d9::IDirect3DQuery9);
convert!(unsafe StateBlock          => Unknown,     winapi::shared::d3d9::IDirect3DStateBlock9);
convert!(unsafe VertexDeclaration   => Unknown,     winapi::shared::d3d9::IDirect3DVertexDeclaration9);

// Singletonish stuff
convert!(unsafe Device              => Unknown,     winapi::shared::d3d9::IDirect3DDevice9);
convert!(unsafe Direct3D            => Unknown,     winapi::shared::d3d9::IDirect3D9);
convert!(unsafe SwapChain           => Unknown,     winapi::shared::d3d9::IDirect3DSwapChain9);
#[cfg(feature = "9ex")] convert!(unsafe DeviceEx    => Device,      winapi::shared::d3d9::IDirect3DDevice9Ex);
#[cfg(feature = "9ex")] convert!(unsafe Direct3DEx  => Direct3D,    winapi::shared::d3d9::IDirect3D9Ex);
#[cfg(feature = "9ex")] convert!(unsafe SwapChainEx => SwapChain,   winapi::shared::d3d9::IDirect3DSwapChain9Ex);

// Resources
convert!(unsafe Resource            => Unknown,     winapi::shared::d3d9::IDirect3DResource9);
convert!(unsafe Surface             => Resource,    winapi::shared::d3d9::IDirect3DSurface9);
convert!(unsafe BaseTexture         => Resource,    winapi::shared::d3d9::IDirect3DBaseTexture9);
convert!(unsafe CubeTexture         => BaseTexture, winapi::shared::d3d9::IDirect3DCubeTexture9);
convert!(unsafe VolumeTexture       => BaseTexture, winapi::shared::d3d9::IDirect3DVolumeTexture9);
convert!(unsafe Texture             => BaseTexture, winapi::shared::d3d9::IDirect3DTexture9);
convert!(unsafe IndexBuffer         => Resource,    winapi::shared::d3d9::IDirect3DIndexBuffer9);
convert!(unsafe VertexBuffer        => Resource,    winapi::shared::d3d9::IDirect3DVertexBuffer9);
// Not resources despite what you might expect
convert!(unsafe PixelShader         => Unknown,     winapi::shared::d3d9::IDirect3DPixelShader9);
convert!(unsafe VertexShader        => Unknown,     winapi::shared::d3d9::IDirect3DVertexShader9);
convert!(unsafe Volume              => Unknown,     winapi::shared::d3d9::IDirect3DVolume9);
