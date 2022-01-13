use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;
use std::fmt::{self, Debug, Formatter};



/// \[<strike>docs.microsoft.com</strike>\]
/// XUSER_\*
///
/// A user index, typically from 0 ..= 3.
///
/// **NOTE:** Despite most `XInput*` functions expecting a [`u32`], [Keystroke](crate::xinput::Keystroke) stores a [`u8`].  For ABI convenience, this stronger type is also a [`u8`]!
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)]
#[repr(transparent)]
pub struct User(u8);

// in lieu of enumish! { ... }
impl User { #[allow(missing_docs)] pub const fn from_unchecked(index: u8) -> Self { Self(index) } }
impl Debug for User     { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { if *self == User::Any { Debug::fmt("User::Any", fmt) } else { write!(fmt, "User({})", self.0) } } }
impl From<User> for  u8 { fn from(user: User) -> Self { user.0        } } // XInputGet* requires DWORDs
impl From<User> for u32 { fn from(user: User) -> Self { user.0.into() } } // XInputGet* requires DWORDs

#[allow(non_upper_case_globals)] impl User {
    #[allow(missing_docs)] pub const Zero  : User = User(0);
    #[allow(missing_docs)] pub const One   : User = User(1);
    #[allow(missing_docs)] pub const Two   : User = User(2);
    #[allow(missing_docs)] pub const Three : User = User(3);

    /// Not a user "index" per se - can be passed to e.g. XInputGetState to get the state of "any" gamepad.
    /// (The first still connected?  The first currently active?  Often similar to [`User::Zero`], but not always!)
    pub const Any   : User = User(XUSER_INDEX_ANY as _);

    /// XUSER_MAX_COUNT - The maximum number of valid, typical user indicies.
    pub const MAX_COUNT : u8 = XUSER_MAX_COUNT as _;

    /// Iterator over valid user indicies [User]\(0\) .. [User]\(4\)
    pub fn iter_valid() -> impl Iterator<Item = User> { [User::Zero, User::One, User::Two, User::Three].iter().copied() }
}
