#[cfg(not(test))] macro_rules! test_layout { ( $($tt:tt)* ) => {} }
#[cfg(    test )] macro_rules! test_layout {
    (
        $thin:ty => unsafe $d3d:ty {
            $( $thin_field:ident => $d3d_field:ident ),*
            $(,)?
        }
    ) => {
        #[test] fn layout() {
            use std::mem::*;
            let thin = <$thin>::default();
            let d3d  = unsafe { zeroed::<$d3d>() };
            assert_eq!( size_of_val(&thin),  size_of_val(&d3d),  "size_of {} != {}", stringify!($thin), stringify!($d3d));
            assert_eq!(align_of_val(&thin), align_of_val(&d3d), "align_of {} != {}", stringify!($thin), stringify!($d3d));
            $(
                assert_eq!(size_of_val(&thin.$thin_field),      size_of_val(&d3d.$d3d_field),       "size_of {}::{} != {}::{}", stringify!($thin), stringify!($thin_field), stringify!($d3d), stringify!($d3d_field));
                assert_eq!(offset_of(&thin, &thin.$thin_field), offset_of(&d3d, &d3d.$d3d_field), "offset_of {}::{} != {}::{}", stringify!($thin), stringify!($thin_field), stringify!($d3d), stringify!($d3d_field));
            )*
        }
    };
}

#[cfg(test)] pub fn offset_of<S, F>(s: &S, f: &F) -> usize {
    let s : *const S = s;
    let f : *const F = f;
    (f as usize) - (s as usize)
}

macro_rules! enumish {
    ( $enumish:ty => $d3d:ty; $($ident:ident),* $(,)? ) => {
        impl std::fmt::Debug for $enumish {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $(
                        < $enumish > :: $ident =>  write!(f, concat!(stringify!($enumish), "::", stringify!($ident))),
                    )*
                    other                   => write!(f, "{}({})", stringify!($enumish), other.0),
                }
            }
        }

        impl From<$enumish> for $d3d {
            fn from(value: $enumish) -> Self { value.0.into() }
        }

        #[cfg(feature = "impl-from-unchecked")]
        impl From<$d3d> for $enumish {
            fn from(value: $d3d) -> Self { Self(value.into()) }
        }

        impl $enumish {
            /// Convert from an underlying [winapi] `D3D...` type.
            /// This is *probably* safe... probably...
            pub const fn from_unchecked(d3d: $d3d) -> Self { Self(d3d as _) }

            /// Convert back into an underlying [winapi] `D3D...` type.
            pub const fn into(self) -> $d3d { self.0 as _ }
        }
    }
}

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
            fn from(value: $d3d) -> Self { Self(value.into()) }
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
