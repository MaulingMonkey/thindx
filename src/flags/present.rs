#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9caps::*;
type D3DPRESENT = u32; // there's no actual type



// "Flags for IDirect3DSwapChain9::Present"

// D3DPRESENT_* from d3d9.h instead of d3dcaps.h
const D3DPRESENT_DONOTWAIT                  : D3DPRESENT = 0x00000001;
const D3DPRESENT_LINEAR_CONTENT             : D3DPRESENT = 0x00000002;

// D3DPRESENT_* from d3d9.h instead of d3dcaps.h - d3d9ex only
const D3DPRESENT_DONOTFLIP                  : D3DPRESENT = 0x00000004;
const D3DPRESENT_FLIPRESTART                : D3DPRESENT = 0x00000008;
const D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR  : D3DPRESENT = 0x00000010;
const D3DPRESENT_UPDATEOVERLAYONLY          : D3DPRESENT = 0x00000020;
const D3DPRESENT_HIDEOVERLAY                : D3DPRESENT = 0x00000040;
const D3DPRESENT_UPDATECOLORKEY             : D3DPRESENT = 0x00000080;
const D3DPRESENT_FORCEIMMEDIATE             : D3DPRESENT = 0x00000100;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpresent)\]
/// DWORD / D3DPRESENT_*
///
/// Describes the relationship between the adapter refresh rate and the rate at which [Device::present] or [SwapChain::present] operations are completed.
/// These values also serve as flag values for the PresentationIntervals field of [Caps].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Present(D3DPRESENT);

flags! {
    Present => D3DPRESENT;

    // None,            // Aliases D3DPRESENT_INTERVAL_DEFAULT
    // DoNotWait,       // Aliases D3DPRESENT_INTERVAL_ONE
    // LinearContent,   // Aliases D3DPRESENT_INTERVAL_TWO
    // DoNotFlip,       // Aliases D3DPRESENT_INTERVAL_THREE
    // FlipRestart,     // Aliases D3DPRESENT_INTERVAL_FOUR

    ForceImmediate, IntervalDefault, IntervalOne, IntervalTwo, IntervalThree, IntervalFour, IntervalImmediate,
    VideoRestrictToMonitor, UpdateOverlayOnly, HideOverlay, UpdateColorKey,
}

#[allow(non_upper_case_globals)] impl Present { // These are enum-like
    pub const None                      : Present = Present(0);
    pub const DoNotFlip                 : Present = Present(D3DPRESENT_DONOTFLIP);
    pub const DoNotWait                 : Present = Present(D3DPRESENT_DONOTWAIT);
    pub const FlipRestart               : Present = Present(D3DPRESENT_FLIPRESTART);
    pub const ForceImmediate            : Present = Present(D3DPRESENT_FORCEIMMEDIATE);
    pub const IntervalDefault           : Present = Present(D3DPRESENT_INTERVAL_DEFAULT);
    pub const IntervalOne               : Present = Present(D3DPRESENT_INTERVAL_ONE);
    pub const IntervalTwo               : Present = Present(D3DPRESENT_INTERVAL_TWO);
    pub const IntervalThree             : Present = Present(D3DPRESENT_INTERVAL_THREE);
    pub const IntervalFour              : Present = Present(D3DPRESENT_INTERVAL_FOUR);
    pub const IntervalImmediate         : Present = Present(D3DPRESENT_INTERVAL_IMMEDIATE);
    pub const LinearContent             : Present = Present(D3DPRESENT_LINEAR_CONTENT);
    pub const VideoRestrictToMonitor    : Present = Present(D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR);
    pub const UpdateOverlayOnly         : Present = Present(D3DPRESENT_UPDATEOVERLAYONLY);
    pub const HideOverlay               : Present = Present(D3DPRESENT_HIDEOVERLAY);
    pub const UpdateColorKey            : Present = Present(D3DPRESENT_UPDATECOLORKEY);
}

impl Default for Present {
    fn default() -> Self { Present::None }
}
