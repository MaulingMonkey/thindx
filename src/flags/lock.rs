#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;
type D3DLOCK = u32; // there's no actual type

use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlock)\]
/// DWORD / D3DLOCK_*
///
/// A combination of zero or more locking options that describe the type of lock to perform.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Lock(D3DLOCK);

impl Lock {
    /// Convert a raw [D3DLOCK] value into a [Lock].  This is *probably* safe... probably...
    ///
    /// [D3DLOCK]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlock
    pub const fn from_unchecked(lock: D3DLOCK) -> Self { Self(lock) }

    /// Convert a [Lock] into a raw [D3DLOCK].
    ///
    /// [D3DLOCK]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlock
    pub const fn into(self) -> D3DLOCK { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Lock {
    /// No lock flags
    pub const None              : Lock = Lock(0);

    /// The application discards all memory within the locked region.
    /// For vertex and index buffers, the entire buffer will be discarded.
    /// This option is only valid when the resource is created with [Usage::Dynamic].
    pub const Discard           : Lock = Lock(D3DLOCK_DISCARD);

    /// Allows an application to gain back CPU cycles if the driver cannot lock the surface immediately.
    ///If this flag is set and the driver cannot lock the surface immediately, the lock call will return [D3DERR::WASSTILLDRAWING].
    /// This flag can only be used when locking a surface created using [Device::create_offscreen_plain_surface], [Device::create_render_target], or [Device::create_depth_stencil_surface].
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

impl BitOrAssign for Lock {
    fn bitor_assign(&mut self, other: Self) { self.0 |= other.0 }
}

impl BitOr for Lock {
    type Output = Self;
    fn bitor(self, other: Self) -> Self { Self(self.0 | other.0) }
}

impl Debug for Lock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Lock::Discard       => write!(f, "Lock::Discard"),
            Lock::DoNotWait     => write!(f, "Lock::DoNotWait"),
            Lock::NoDirtyUpdate => write!(f, "Lock::NoDirtyUpdate"),
            Lock::NoOverwrite   => write!(f, "Lock::NoOverwrite"),
            Lock::NoSysLock     => write!(f, "Lock::NoSysLock"),
            Lock::ReadOnly      => write!(f, "Lock::ReadOnly"),
            other               => write!(f, "Lock({})", other.0),
        }
    }
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for Lock {
    fn default() -> Self { Lock(0) }
}

impl From<Lock> for D3DLOCK {
    fn from(value: Lock) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DLOCK> for Lock {
    fn from(value: D3DLOCK) -> Self { Self(value) }
}
