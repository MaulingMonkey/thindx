// XXX: temporary attributes
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(missing_docs)]

use crate::Unknown;

mods! {
    inl mod interfaces {
        inl mod device;
        inl mod deviceex;
        inl mod buffers;
        inl mod direct3d;
        inl mod direct3dex;
        inl mod query;
        inl mod resource;
        inl mod shaders;
        inl mod state_block;
        inl mod surface;
        inl mod swap_chain;
        inl mod swap_chain_ex;
        inl mod texture;
        inl mod vertex_declaration;
        inl mod volume;
        mod device_misc;
    }

    inl mod wrappers {
        inl mod safe_device;
    }
}

// Misc
//convert!(unsafe Unknown,                            winapi::um::unknwnbase::IUnknown);
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


// d3d.h has: #define D3DAPI WINAPI
//#cpp2rust D3DAPI          = extern "system"
//#cpp2rust WINAPI          = extern "system"
