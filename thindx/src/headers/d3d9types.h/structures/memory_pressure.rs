use crate::ctypes::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dmemorypressure)\]
/// D3DMEMORYPRESSURE
///
/// Contains data for memory pressure reporting.
///
// ### See Also
// TODO
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)] // !Pod: trailing padding on x64
pub struct MemoryPressure {
    /// The number of bytes that were evicted by the process during the duration of the query.
    pub bytes_evicted_from_process:     Pack4OnX86<u64>,

    /// The total number of bytes placed in nonoptimal memory segments,
    /// due to inadequate space within the preferred memory segments.
    pub size_of_inefficient_allocation: Pack4OnX86<u64>,

    /// The overall efficiency of the memory allocations that were placed in nonoptimal memory.
    /// The value is expressed as a percentage.
    /// For example, `95` indicates that the allocations placed in nonpreferred memory segments are 95% efficient.
    /// This number should not be considered an exact measurement.
    pub level_of_efficiency:            u32,

    // 32 bits of implicit trailing padding on 64-bit
}

#[test] fn align_vs_cpp() {
    if cfg!(target_arch = "x86") { // or itanium?
        assert_eq!(20, std::mem::size_of::<MemoryPressure>());
        assert_eq!( 4, std::mem::align_of::<MemoryPressure>());
    } else {
        assert_eq!(std::mem::align_of::<D3DMEMORYPRESSURE>(), std::mem::align_of::<MemoryPressure>());
        if cfg!(target_arch = "x86_64") {
            assert_eq!(24, std::mem::size_of::<MemoryPressure>());
            assert_eq!( 8, std::mem::align_of::<MemoryPressure>());
        }
    }
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    #[ignore(align)] // See align_vs_cpp - winapi has alignment 1, real alignment should be 4 on x86, 8 otherwise
    MemoryPressure => D3DMEMORYPRESSURE {
        bytes_evicted_from_process      => BytesEvictedFromProcess,
        size_of_inefficient_allocation  => SizeOfInefficientAllocation,
        level_of_efficiency             => LevelOfEfficiency,
    }
}

//#cpp2rust D3DMEMORYPRESSURE = d3d::MemoryPressure
