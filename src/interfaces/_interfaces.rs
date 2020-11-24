use crate::*;

mod device;         pub use device::Device;
mod device_draw;
mod buffers;        pub use buffers::*;
mod direct3d;       pub use direct3d::*;
mod direct3dex;     pub use direct3dex::*;
mod query;          pub use query::*;
mod shaders;        pub use shaders::*;
mod state_block;    pub use state_block::*;
mod surface;        pub use surface::*;
mod swap_chain;     pub use swap_chain::*;
mod texture;        pub use texture::*;
mod vertex_declaration; pub use vertex_declaration::*;
mod device_misc;


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)\]
/// Base COM interface all D3D9 types eventually derive from.
#[derive(Clone)] #[repr(transparent)]
pub struct Unknown(mcom::Rc<winapi::um::unknwnbase::IUnknown>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9ex)\]
/// (extends [Device])
/// Core interface used for general rendering, resource creation, etc.
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct DeviceEx(mcom::Rc<winapi::shared::d3d9::IDirect3DDevice9Ex>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dresource9)\]
/// [\*Texture\*](crate::BaseTexture), [Surface] (but not <strike>[Volume]</strike>!), [IndexBuffer], [VertexBuffer], but not <strike>[\*Shader](crate::PixelShader)</strike>!
#[derive(Clone)] #[repr(transparent)]
pub struct Resource(mcom::Rc<winapi::shared::d3d9::IDirect3DResource9>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dvolume9)\]
/// A dense 3-dimensional region of data, often belonging to a [VolumeTexture]
#[derive(Clone)] #[repr(transparent)]
pub struct Volume(mcom::Rc<winapi::shared::d3d9::IDirect3DVolume9>);



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
            unsafe fn from_raw_opt(raw: *mut Self::Raw) -> Option<Self> { Some(Self(mcom::Rc::from_raw_opt(raw)?)) }
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
