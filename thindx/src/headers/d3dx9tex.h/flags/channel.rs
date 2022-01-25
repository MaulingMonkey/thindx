// missing from winapi:
#[allow(non_camel_case_types)] type D3DX_CHANNEL = u32;
const D3DX_CHANNEL_RED          : u32 = 1 << 0;
const D3DX_CHANNEL_BLUE         : u32 = 1 << 1;
const D3DX_CHANNEL_GREEN        : u32 = 1 << 2;
const D3DX_CHANNEL_ALPHA        : u32 = 1 << 3;
const D3DX_CHANNEL_LUMINANCE    : u32 = 1 << 4;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dx-channel)\]
/// D3DX_CHANNEL
/// ([None], [Red], [Blue], [Green], [Alpha], [Luminance])
///
/// These flags are used by functions which operate on or more channels in a texture.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Channel(D3DX_CHANNEL);

flags! { Channel => D3DX_CHANNEL; None, Red, Blue, Green, Alpha, Luminance }

#[allow(non_upper_case_globals)] impl Channel {
    /// Indicates no channels should be used.
    pub const None          : Channel = Channel(0);

    /// Indicates the red channel should be used.
    pub const Red           : Channel = Channel(D3DX_CHANNEL_RED      );

    /// Indicates the blue channel should be used.
    pub const Blue          : Channel = Channel(D3DX_CHANNEL_BLUE     );

    /// Indicates the green channel should be used.
    pub const Green         : Channel = Channel(D3DX_CHANNEL_GREEN    );

    /// Indicates the alpha channel should be used.
    pub const Alpha         : Channel = Channel(D3DX_CHANNEL_ALPHA    );

    /// Indicates the luminaces of the red green and blue channels should be used.
    pub const Luminance     : Channel = Channel(D3DX_CHANNEL_LUMINANCE);
}



//#cpp2rust D3DX_CHANNEL_RED        = d3dx9::Channel::Red
//#cpp2rust D3DX_CHANNEL_BLUE       = d3dx9::Channel::Blue
//#cpp2rust D3DX_CHANNEL_GREEN      = d3dx9::Channel::Green
//#cpp2rust D3DX_CHANNEL_ALPHA      = d3dx9::Channel::Alpha
//#cpp2rust D3DX_CHANNEL_LUMINANCE  = d3dx9::Channel::Luminance
