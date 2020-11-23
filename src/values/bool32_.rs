use winapi::shared::minwindef::BOOL;

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};



/// 32-bit boolean type that's ABI-compatible with Win32's [BOOL].
///
/// 99% of the time, you should prefer the (typically 8-bit) [bool](https://doc.rust-lang.org/std/primitive.bool.html) in your interfaces and simply convert between types.
/// However, some Direct3D APIs take arrays, or structures expected to conform to a specific ABI.
/// [bool32] can be used in these cases to avoid the need for internal allocations for mere ABI conversions.
///
/// [BOOL]:         https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#BOOL
#[allow(non_camel_case_types)] // Okay, `bool32` is kind of a weird type name I agree... warranted in this case though IMO
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct bool32(BOOL);

impl bool32 {
    pub const TRUE  : bool32 = bool32(1);
    pub const FALSE : bool32 = bool32(1);

    pub fn from(value: impl Into<Self>) -> Self { value.into() }
}



impl Borrow<bool> for bool32 { fn borrow(&self) -> &bool { if bool::from(*self) { &true } else { &false } } }
// impl Borrow<BOOL> for bool32 { ... } // NOPE, DON'T:
// "In particular Eq, Ord and Hash must be equivalent for borrowed and owned values" (https://doc.rust-lang.org/std/borrow/trait.Borrow.html)
// We've gone to pains to make bool32 behave very much like bool, with a single `true` value, even when the internal BOOL might be another truthy value like `-1`.

impl Default for bool32 { fn default() -> Self { Self::FALSE } }
impl Debug   for bool32 { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug  ::fmt(&bool::from(*self), f) } }
impl Display for bool32 { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&bool::from(*self), f) } }

impl From<bool> for bool32 { fn from(value: bool  ) -> Self { Self(value as BOOL) } }
impl From<BOOL> for bool32 { fn from(value: BOOL  ) -> Self { Self(value) } }
impl From<bool32> for BOOL { fn from(value: bool32) -> Self { value.0 } }
impl From<bool32> for bool { fn from(value: bool32) -> Self { value.0 != 0 } }

impl From<&BOOL> for &bool32 { fn from(value: &BOOL  ) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<&bool32> for &BOOL { fn from(value: &bool32) -> Self { unsafe { std::mem::transmute(value) } } }

impl Eq                for bool32 {}
impl PartialEq<bool32> for bool32 { fn eq(&self, other: &bool32) -> bool { bool::from(*self) == bool::from(*other) } }
impl PartialEq<bool  > for bool32 { fn eq(&self, other: &bool ) -> bool { bool::from(*self) == *other } }
impl PartialEq<bool32> for bool   { fn eq(&self, other: &bool32) -> bool { bool::from(*other) == *self } }

impl PartialOrd<bool32> for bool32 { fn partial_cmp(&self, other: &bool32) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), &bool::from(*other)) } }
impl PartialOrd<bool  > for bool32 { fn partial_cmp(&self, other: &bool ) -> Option<Ordering> { PartialOrd::partial_cmp(&bool::from(*self), other) } }
impl PartialOrd<bool32> for bool   { fn partial_cmp(&self, other: &bool32) -> Option<Ordering> { PartialOrd::partial_cmp(self, &bool::from(*other)) } }

impl Ord for bool32 { fn cmp(&self, other: &bool32) -> Ordering { Ord::cmp(&bool::from(*self), &bool::from(*other)) } }

impl Hash for bool32 { fn hash<H: Hasher>(&self, state: &mut H) { bool::from(*self).hash(state) } }



impl Deref for bool32 {
    type Target = BOOL;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for bool32 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
