#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9::*;
type D3DCREATE = u32; // there's no actual type

#[cfg(feature = "9ex")] const D3DCREATE_DISABLE_PRINTSCREEN : D3DCREATE = 0x8000; // missing from winapi

use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcreate)\]
/// DWORD / D3DCREATE_*
///
/// Controls how [IDirect3D9Ext::create_device] behaves.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Create(D3DCREATE);

impl Create {
    pub const fn from_unchecked(create: D3DCREATE) -> Self { Self(create) }
    pub const fn into(self) -> D3DCREATE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Create {
    /// No flags
    pub const None                      : Create = Create(0);

    /// Application asks the device to drive all the heads that this master adapter owns.
    /// The flag is illegal on nonmaster adapters.
    /// If this flag is set, the presentation parameters passed to [IDirect3D9Ext::create_device] should point to an array of D3DPRESENT_PARAMETERS.
    /// The number of elements in D3DPRESENT_PARAMETERS should equal the number of adapters defined by the NumberOfAdaptersInGroup member of the [d3d9::Caps] structure.
    /// The DirectX runtime will assign each element to each head in the numerical order specified by the AdapterOrdinalInGroup member of [Caps].
    pub const AdapterGroupDevice        : Create = Create(D3DCREATE_ADAPTERGROUP_DEVICE);

    /// Direct3D will manage resources instead of the driver.
    /// Direct3D calls will not fail for resource errors such as insufficient video memory.
    pub const DisableDriverManagement   : Create = Create(D3DCREATE_DISABLE_DRIVER_MANAGEMENT);

    /// Like [Create::DisableDriverManagement], Direct3D will manage resources instead of the driver.
    /// Unlike [Create::DisableDriverManagement], [Create::DisableDriverManagementEx] will return errors for conditions such as insufficient video memory.
    pub const DisableDriverManagementEx : Create = Create(D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX);

    /// Causes the runtime not register hotkeys for Printscreen, Ctrl-Printscreen and Alt-Printscreen to capture the desktop or window content.
    ///
    /// Differences between Direct3D 9 and Direct3D 9Ex: This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const DisablePrintScreen        : Create = Create(D3DCREATE_DISABLE_PRINTSCREEN);

    /// Restrict computation to the main application thread.
    /// If the flag is not set, the runtime may perform software vertex processing and other computations in worker thread to improve performance on multi-processor systems.
    ///
    /// Differences between Windows XP and Windows Vista:  This flag is available on Windows Vista, Windows Server 2008, and Windows 7.
    pub const DisablePSGPThreading      : Create = Create(D3DCREATE_DISABLE_PSGP_THREADING);

    /// Enables the gathering of present statistics on the device.
    /// Calls to [SwapChainEx::get_present_statistics] will return valid data.
    ///
    /// Differences between Direct3D 9 and Direct3D 9Ex:  This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const EnablePresentStats        : Create = Create(D3DCREATE_ENABLE_PRESENTSTATS);

    /// Set the precision for Direct3D floating-point calculations to the precision used by the calling thread.
    /// If you do not specify this flag, Direct3D defaults to single-precision round-to-nearest mode for two reasons:
    /// *   Double-precision mode will reduce Direct3D performance.
    /// *   Portions of Direct3D assume floating-point unit exceptions are masked; unmasking these exceptions may result in undefined behavior.
    pub const FpuPreserve               : Create = Create(D3DCREATE_FPU_PRESERVE);

    /// Specifies hardware vertex processing.
    pub const HardwareVertexProcessing  : Create = Create(D3DCREATE_HARDWARE_VERTEXPROCESSING);

    /// Specifies mixed (both software and hardware) vertex processing.
    /// For Windows 10, version 1607 and later, use of this setting is not recommended. See [Create::SoftwareVertexProcessing].
    pub const MixedVertexProcessing     : Create = Create(D3DCREATE_MIXED_VERTEXPROCESSING);

    /// Specifies software vertex processing.
    /// For Windows 10, version 1607 and later, use of this setting is not recommended. Use [Create::HardwareVertexProcessing].
    ///
    /// Note: Unless hardware vertex processing is not available, the usage of software vertex processing is not recommended in Windows 10, version 1607 (and later versions) because the efficiency of software vertex processing was significantly reduced while improving the security of the implementation.
    pub const SoftwareVertexProcessing  : Create = Create(D3DCREATE_SOFTWARE_VERTEXPROCESSING);

    /// Indicates that the application requests Direct3D to be multithread safe.
    /// This makes a Direct3D thread take ownership of its global critical section more frequently, which can degrade performance.
    /// If an application processes window messages in one thread while making Direct3D API calls in another, the application must use this flag when creating the device.
    /// This window must also be destroyed before unloading d3d9.dll.
    pub const MultiThreaded             : Create = Create(D3DCREATE_MULTITHREADED);

    /// Indicates that Direct3D must not alter the focus window in any way.
    ///
    /// Note: If this flag is set, the application must fully support all focus management events, such as ALT+TAB and mouse click events.
    pub const NoWindowChanges           : Create = Create(D3DCREATE_NOWINDOWCHANGES);

    /// Specifies that Direct3D does not support Get* calls for anything that can be stored in state blocks.
    /// It also tells Direct3D not to provide any emulation services for vertex processing.
    /// This means that if the device does not support vertex processing, then the application can use only post-transformed vertices.
    pub const PureDevice                : Create = Create(D3DCREATE_PUREDEVICE);

    /// Allows screensavers during a fullscreen application.
    /// Without this flag, Direct3D will disable screensavers for as long as the calling application is fullscreen.
    /// If the calling application is already a screensaver, this flag has no effect.
    ///
    /// Differences between Direct3D 9 and Direct3D 9Ex: This flag is available in Direct3D 9Ex only.
    #[cfg(feature = "9ex")]
    pub const ScreenSaver               : Create = Create(D3DCREATE_SCREENSAVER);
}

impl BitOrAssign for Create {
    fn bitor_assign(&mut self, other: Self) { self.0 |= other.0 }
}

impl BitOr for Create {
    type Output = Self;
    fn bitor(self, other: Self) -> Self { Self(self.0 | other.0) }
}

impl Debug for Create {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.0 == 0 {
            return write!(f, "Create::None");
        }

        write!(f, "Create::{{")?;
        let mut bits = self.0;
        let mut prev = false;
        for bit in 0..31 {
            let mask = 1 << bit;
            if mask & bits != 0 {
                let str = match Self(mask) {
                    Create::AdapterGroupDevice          => "AdapterGroupDevice",
                    Create::DisableDriverManagement     => "DisableDriverManagement",
                    Create::DisableDriverManagementEx   => "DisableDriverManagementEx",
                    #[cfg(feature = "9ex")]
                    Create::DisablePrintScreen          => "DisablePrintScreen",
                    Create::DisablePSGPThreading        => "DisablePSGPThreading",
                    #[cfg(feature = "9ex")]
                    Create::EnablePresentStats          => "EnablePresentStats",
                    Create::FpuPreserve                 => "FpuPreserve",
                    Create::HardwareVertexProcessing    => "HardwareVertexProcessing",
                    Create::MixedVertexProcessing       => "MixedVertexProcessing",
                    Create::SoftwareVertexProcessing    => "SoftwareVertexProcessing",
                    Create::MultiThreaded               => "MultiThreaded",
                    Create::NoWindowChanges             => "NoWindowChanges",
                    Create::PureDevice                  => "PureDevice",
                    #[cfg(feature = "9ex")]
                    Create::ScreenSaver                 => "ScreenSaver",
                    _other                              => continue,
                };

                if prev { write!(f, " | ")?; }
                prev = true;
                write!(f, "{}", str)?;
                bits &= !mask;
            }
        }
        if bits != 0 {
            if prev { write!(f, " | ")?; }
            write!(f, "0x{:08x}", bits)?;
        }
        write!(f, "}}")
    }
}

impl From<Create> for D3DCREATE {
    fn from(value: Create) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DCREATE> for Create {
    fn from(value: D3DCREATE) -> Self { Self(value) }
}

//#cpp2rust D3DCREATE                               = d3d::Create
//#cpp2rust D3DCREATE_FPU_PRESERVE                  = d3d::Create::FpuPreserve
//#cpp2rust D3DCREATE_MULTITHREADED                 = d3d::Create::MultiThreaded
//#cpp2rust D3DCREATE_PUREDEVICE                    = d3d::Create::PureDevice
//#cpp2rust D3DCREATE_SOFTWARE_VERTEXPROCESSING     = d3d::Create::SoftwareVertexProcessing
//#cpp2rust D3DCREATE_HARDWARE_VERTEXPROCESSING     = d3d::Create::HardwareVertexProcessing
//#cpp2rust D3DCREATE_MIXED_VERTEXPROCESSING        = d3d::Create::MixedVertexProcessing
//#cpp2rust D3DCREATE_DISABLE_DRIVER_MANAGEMENT     = d3d::Create::DisableDriverManagement
//#cpp2rust D3DCREATE_ADAPTERGROUP_DEVICE           = d3d::Create::AdapterGroupDevice
//#cpp2rust D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX  = d3d::Create::DisableDriverManagementEx
//#cpp2rust D3DCREATE_NOWINDOWCHANGES               = d3d::Create::NoWindowChanges
//#cpp2rust D3DCREATE_DISABLE_PSGP_THREADING        = d3d::Create::DisablePSGPThreading
//#cpp2rust D3DCREATE_ENABLE_PRESENTSTATS           = d3d::Create::EnablePresentStats
//#cpp2rust D3DCREATE_DISABLE_PRINTSCREEN           = d3d::Create::DisablePrintScreen
//#cpp2rust D3DCREATE_SCREENSAVER                   = d3d::Create::ScreenSaver
