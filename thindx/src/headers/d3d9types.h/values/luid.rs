#[allow(unused_imports)] use crate::*;

use winapi::shared::ntdef::LUID;

use std::cmp::Ordering;
use std::ops::*;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/previous-versions/windows/hardware/drivers/ff549708(v=vs.85))\]
/// LUID
///
/// The locally unique identifier (LUID) is a 64-bit value guaranteed to be unique only on the system on which it was generated.
/// The uniqueness of an LUID is guaranteed only until the system is restarted.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Luid(LUID);

impl Deref    for Luid { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = LUID; }
impl DerefMut for Luid { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<LUID> for Luid { fn from(value: LUID) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Luid> for LUID { fn from(value: Luid) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Luid> for u64  { fn from(value: Luid) -> Self { unsafe { std::mem::transmute(value) } } }

impl Default    for Luid { fn default() -> Self { Self(unsafe { std::mem::zeroed() }) } }
impl Debug      for Luid { fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "Luid(0x{:016x})", u64::from(*self)) } }
impl PartialEq  for Luid { fn eq(&self, other: &Luid) -> bool { u64::from(*self) == u64::from(*other) } }
impl Eq         for Luid {}
impl PartialOrd for Luid { fn partial_cmp(&self, other: &Luid) -> Option<Ordering> { u64::from(*self).partial_cmp(&u64::from(*other)) } }
impl Ord        for Luid { fn cmp(&self, other: &Self) -> Ordering { u64::from(*self).cmp(&u64::from(*other)) } }
impl Hash       for Luid { fn hash<H: Hasher>(&self, state: &mut H) { u64::from(*self).hash(state) } }

//#cpp2rust LUID = Luid
