#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;
type D3DLOCK = u32; // there's no actual type



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlock)\]
/// DWORD / D3DLOCK_*
///
/// A combination of zero or more locking options that describe the type of lock to perform.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Lock(D3DLOCK);

flags! { Lock => D3DLOCK; None, Discard, DoNotWait, NoDirtyUpdate, NoOverwrite, NoSysLock, ReadOnly }

#[allow(non_upper_case_globals)] impl Lock { // These are enum-like
    /// No lock flags
    pub const None              : Lock = Lock(0);

    /// The application discards all memory within the locked region.
    /// For vertex and index buffers, the entire buffer will be discarded.
    /// This option is only valid when the resource is created with [Usage::Dynamic].
    pub const Discard           : Lock = Lock(D3DLOCK_DISCARD);

    /// Allows an application to gain back CPU cycles if the driver cannot lock the surface immediately.
    /// If this flag is set and the driver cannot lock the surface immediately, the lock call will return [D3DERR::WASSTILLDRAWING].
    /// This flag can only be used when locking a surface created using [IDirect3DDevice9Ext::create_offscreen_plain_surface], [IDirect3DDevice9Ext::create_render_target], or [IDirect3DDevice9Ext::create_depth_stencil_surface].
    /// This flag can also be used with a back buffer.
    pub const DoNotWait         : Lock = Lock(D3DLOCK_DONOTWAIT);

    /// By default, a lock on a resource adds a dirty region to that resource.
    /// This option prevents any changes to the dirty state of the resource.
    /// Applications should use this option when they have additional information about the set of regions changed during the lock operation.
    pub const NoDirtyUpdate     : Lock = Lock(D3DLOCK_NO_DIRTY_UPDATE);

    /// Indicates that memory that was referred to in a drawing call since the last lock without this flag will not be modified during the lock.
    /// This can enable optimizations when the application is appending data to a resource.
    /// Specifying this flag enables the driver to return immediately if the resource is in use, otherwise, the driver must finish using the resource before returning from locking.
    pub const NoOverwrite       : Lock = Lock(D3DLOCK_NOOVERWRITE);

    /// The default behavior of a video memory lock is to reserve a system-wide critical section, guaranteeing that no display mode changes will occur for the duration of the lock.
    /// This option causes the system-wide critical section not to be held for the duration of the lock.
    ///
    /// The lock operation is time consuming, but can enable the system to perform other duties, such as moving the mouse cursor.
    /// This option is useful for long-duration locks, such as the lock of the back buffer for software rendering that would otherwise adversely affect system responsiveness.
    pub const NoSysLock         : Lock = Lock(D3DLOCK_NOSYSLOCK);

    /// The application will not write to the buffer.
    /// This enables resources stored in non-native formats to save the recompression step when unlocking.
    pub const ReadOnly          : Lock = Lock(D3DLOCK_READONLY);
}



//#cpp2rust D3DLOCK_DISCARD         = d3d::Lock::Discard
//#cpp2rust D3DLOCK_DONOTWAIT       = d3d::Lock::DoNotWait
//#cpp2rust D3DLOCK_NO_DIRTY_UPDATE = d3d::Lock::NoDirtyUpdate
//#cpp2rust D3DLOCK_NOOVERWRITE     = d3d::Lock::NoOverwrite
//#cpp2rust D3DLOCK_NOSYSLOCK       = d3d::Lock::NoSysLock
//#cpp2rust D3DLOCK_READONLY        = d3d::Lock::ReadOnly
