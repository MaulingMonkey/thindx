use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpresentstats)\]
/// D3DPRESENTSTATS
///
/// Describes swapchain statistics relating to PresentEx calls.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct PresentStats {
    /// Running count of successful Present calls made by a display device that is currently outputting to screen.
    /// This parameter is really the Present ID of the last Present call and is not necessarily the total number of Present API calls made.
    pub present_count:          u32,

    /// The vblank count at which the last Present was displayed on screen, the vblank count increments once every vblank interval.
    pub present_refresh_count:  u32,

    /// The vblank count when the scheduler last sampled the machine time by calling [QueryPerformanceCounter].
    ///
    /// [QueryPerformanceCounter]:  https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter
    pub sync_refresh_count:     u32,

    /// The scheduler's last sampled machine time, obtained by calling [QueryPerformanceCounter].
    ///
    /// [QueryPerformanceCounter]:  https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter
    pub sync_qpc_time:          u64,

    /// This value is not used.
    pub sync_gpu_time:          u64,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    PresentStats => D3DPRESENTSTATS {
        present_count           => PresentCount,
        present_refresh_count   => PresentRefreshCount,
        sync_refresh_count      => SyncRefreshCount,
        sync_qpc_time           => SyncQPCTime,
        sync_gpu_time           => SyncGPUTime
    }
}

//#cpp2rust D3DPRESENTSTATS = d3d::PresentStats
