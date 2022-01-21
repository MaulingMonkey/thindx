#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dswapeffect)\]
/// D3DSWAPEFFECT
///
/// Defines swap effects.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SwapEffect(D3DSWAPEFFECT);

enumish! { SwapEffect => D3DSWAPEFFECT; Discard, Flip, Copy, Overlay, FlipEx }

#[allow(non_upper_case_globals)] impl SwapEffect { // These are enum-like
    /// When a swap chain is created with a swap effect of [SwapEffect::Flip] or [SwapEffect::Copy], the runtime will guarantee that an [IDirect3DDevice9Ext::present] operation will not affect the content of any of the back buffers.
    /// Unfortunately, meeting this guarantee can involve substantial video memory or processing overheads, especially when implementing flip semantics for a windowed swap chain or copy semantics for a full-screen swap chain.
    /// An application may use the [SwapEffect::Discard] swap effect to avoid these overheads and to enable the display driver to select the most efficient presentation technique for the swap chain.
    /// This is also the only swap effect that may be used when specifying a value other than [MultiSample::None] for the MultiSampleType member of D3DPRESENT_PARAMETERS.
    ///
    /// Like a swap chain that uses [SwapEffect::Flip], a swap chain that uses [SwapEffect::Discard] might include more than one back buffer, any of which may be accessed using [IDirect3DDevice9Ext::get_back_buffer] or [SwapChain::get_back_buffer].
    /// The swap chain is best envisaged as a queue in which 0 always indexes the back buffer that will be displayed by the next present operation and from which buffers are discarded when they have been displayed.
    ///
    /// An application that uses this swap effect cannot make any assumptions about the contents of a discarded back buffer and should therefore update an entire back buffer before invoking a Present operation that would display it.
    /// Although this is not enforced, the debug version of the runtime will overwrite the contents of discarded back buffers with random data to enable developers to verify that their applications are updating the entire back buffer surfaces correctly.
    pub const Discard       : SwapEffect = SwapEffect(D3DSWAPEFFECT_DISCARD);

    /// The swap chain might include multiple back buffers and is best envisaged as a circular queue that includes the front buffer.
    /// Within this queue, the back buffers are always numbered sequentially from 0 to (n - 1), where n is the number of back buffers, so that 0 denotes the least recently presented buffer.
    /// When present is invoked, the queue is "rotated" so that the front buffer becomes back buffer (n - 1), while the back buffer 0 becomes the new front buffer.
    pub const Flip          : SwapEffect = SwapEffect(D3DSWAPEFFECT_FLIP);

    /// This swap effect may be specified only for a swap chain comprising a single back buffer.
    /// Whether the swap chain is windowed or full-screen, the runtime will guarantee the semantics implied by a copy-based present operation, namely that the operation leaves the content of the back buffer unchanged, instead of replacing it with the content of the front buffer as a flip-based present operation would.
    ///
    /// For a full-screen swap chain, the runtime uses a combination of flip operations and copy operations, supported if necessary by hidden back buffers, to accomplish the present operation.
    /// Accordingly, the presentation is synchronized with the display adapter's vertical retrace and its rate is constrained by the chosen presentation interval.
    /// A swap chain specified with the [Present::IntervalImmediate] flag is the only exception.
    /// (Refer to the description of the PresentationIntervals member of the D3DPRESENT_PARAMETERS structure.)
    /// In this case, a Present operation copies the back buffer content directly to the front buffer without waiting for the vertical retrace.
    pub const Copy          : SwapEffect = SwapEffect(D3DSWAPEFFECT_COPY);

    /// Use a dedicated area of video memory that can be overlayed on the primary surface.
    /// No copy is performed when the overlay is displayed.
    /// The overlay operation is performed in hardware, without modifying the data in the primary surface.
    ///
    /// Differences between Direct3D 9 and Direct3D 9Ex: [SwapEffect::Overlay] is only available in Direct3D9Ex running on Windows 7 (or more current operating system).
    pub const Overlay       : SwapEffect = SwapEffect(D3DSWAPEFFECT_OVERLAY);

    /// Designates when an application is adopting flip mode, during which time an application's frame is passed instead of copied to the Desktop Window Manager(DWM) for composition when the application is presenting in windowed mode.
    /// Flip mode allows an application to more efficiently use memory bandwidth as well as enabling an application to take advantage of full-screen-present statistics.
    /// Flip mode does not affect full-screen behavior.
    ///
    /// Note: If you create a swap chain with [SwapEffect::FlipEx], you can't override the `hDeviceWindow` member of the `D3DPRESENT_PARAMETERS` structure when you present a new frame for display.
    /// That is, you must pass `null_mut()` to the `hDestWindowOverride` parameter of [IDirect3DDevice9ExExt::present_ex] to instruct the runtime to use the `hDeviceWindow` member of `D3DPRESENT_PARAMETERS` for the presentation.
    ///
    /// Differences between Direct3D 9 and Direct3D 9Ex: D3DSWAPEFFECT_FLIPEX is only available in Direct3D9Ex running on Windows 7 (or more current operating system).
    pub const FlipEx        : SwapEffect = SwapEffect(D3DSWAPEFFECT_FLIPEX);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for SwapEffect {
    fn default() -> Self { SwapEffect(0) }
}

//#cpp2rust D3DSWAPEFFECT           = d3d::SwapEffect
//#cpp2rust D3DSWAPEFFECT_DISCARD   = d3d::SwapEffect::Discard
//#cpp2rust D3DSWAPEFFECT_FLIP      = d3d::SwapEffect::Flip
//#cpp2rust D3DSWAPEFFECT_COPY      = d3d::SwapEffect::Copy
//#cpp2rust D3DSWAPEFFECT_OVERLAY   = d3d::SwapEffect::Overlay
//#cpp2rust D3DSWAPEFFECT_FLIPEX    = d3d::SwapEffect::FlipEx
