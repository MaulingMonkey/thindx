use crate::ctypes::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dresourcestats)\]
/// D3DRESOURCESTATS
///
/// Resource statistics gathered by the D3DDEVINFO_ResourceManager when using the asynchronous query mechanism.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ResourceStats {
    /// Indicates if thrashing is occurring (e.g. too many resources used in a frame to have them all loaded.)
    pub thrashing:                  BOOL,

    /// Approximate number of bytes downloaded by the resource manager.
    pub approx_bytes_downloaded:    u32,

    /// Number of objects evicted.
    pub num_evicts:                 u32,

    /// Number of objects created in video memory.
    pub num_vid_creates:            u32,

    /// Priority of last object evicted.
    pub last_pri:                   u32,

    /// Number of objects set to the device.
    pub num_used:                   u32,

    /// Number of objects set to the device, which are already in video memory.
    pub num_used_in_vid_mem:        u32,

    /// Number of objects in video memory.
    pub working_set:                u32,

    /// Number of bytes in video memory.
    pub working_set_bytes:          u32,

    /// Total number of managed objects.
    pub total_managed:              u32,

    /// Total number of bytes of managed objects.
    pub total_bytes:                u32,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    ResourceStats => D3DRESOURCESTATS {
        thrashing                   => bThrashing,
        approx_bytes_downloaded     => ApproxBytesDownloaded,
        num_evicts                  => NumEvicts,
        num_vid_creates             => NumVidCreates,
        last_pri                    => LastPri,
        num_used                    => NumUsed,
        num_used_in_vid_mem         => NumUsedInVidMem,
        working_set                 => WorkingSet,
        working_set_bytes           => WorkingSetBytes,
        total_managed               => TotalManaged,
        total_bytes                 => TotalBytes,
    }
}

//#cpp2rust D3DRESOURCESTATS = d3d::ResourceStats
