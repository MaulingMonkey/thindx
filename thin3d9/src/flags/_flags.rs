macro_rules! flags {
    ( $flagish:ty => $d3d:ty; $($ident:ident),* $(,)? ) => {
        impl std::fmt::Debug for $flagish {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    // Single flag cases
                    $(
                        <$flagish>::$ident => write!(f, concat!(stringify!($flagish), "::", stringify!($ident))),
                    )*
                    // No flags
                    #[allow(unreachable_patterns)] Self(0) => {
                        write!(f, "{}(0)", stringify!($flagish))
                    },
                    // Multiple flags or unnamed flags only
                    other => {
                        let mut bits = other.0;
                        $(
                            if bits & <$flagish>::$ident.0 == <$flagish>::$ident.0 && <$flagish>::$ident.0 != 0 {
                                if bits == other.0 {
                                    write!(f, "{}::{{{}", stringify!($flagish), stringify!($ident))?;
                                } else {
                                    write!(f, "|{}", stringify!($ident))?;
                                }
                                bits &= !<$flagish>::$ident.0;
                            }
                        )*
                        if bits == other.0 {
                            write!(f, "{}(0x{:08x})", stringify!($flagish), bits)?;
                        } else if bits != 0 {
                            write!(f, "|0x{:08x}}}", bits)?;
                        }
                        Ok(())
                    },
                }
            }
        }

        impl From<$flagish> for $d3d {
            fn from(value: $flagish) -> Self { value.0.into() }
        }

        #[cfg(feature = "impl-from-unchecked")]
        impl From<$d3d> for $flagish {
            fn from(value: $d3d) -> Self { Self(value as _) }
        }

        impl $flagish {
            /// Convert from an underlying [winapi] `D3D...` type.
            /// This is *probably* safe... probably...
            pub const fn from_unchecked(d3d: $d3d) -> Self { Self(d3d as _) }

            /// Convert back into an underlying [winapi] `D3D...` type.
            pub const fn into(self) -> $d3d { self.0 as _ }
        }

        impl std::ops::BitOrAssign for $flagish {
            fn bitor_assign(&mut self, other: Self) { self.0 |= other.0 }
        }

        impl std::ops::BitOr for $flagish {
            type Output = Self;
            fn bitor(self, other: Self) -> Self { Self(self.0 | other.0) }
        }
    }
}



mod create;                 pub use create::*;
mod fvf;                    pub use fvf::*;
mod get_data;               pub use get_data::*;
mod issue;                  pub use issue::*;
mod lock;                   pub use lock::*;
mod present;                pub use present::*;
mod usage;                  pub use usage::*;
