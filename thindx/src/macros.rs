#![cfg_attr(not(test), allow(unused_macros))]

/// ### Usage
/// ```no_run
/// struct_mapping! {
///     RustyStruct => D3D_STRUCT {
///         rusty_field_a => CppFieldA,
///         rusty_field_b => CppFieldB,
///     }
/// }
/// ```
macro_rules! struct_mapping {
    ($(
        $(#[$($meta_struct:tt)*])*
        $thin_struct:ty => $d3d_struct:ty {
            $( $(#[$($meta_field:tt)*])* $thin_field:ident => $d3d_field:ident ),*
            $(,)?
        }
    )*) => {
        $(
            struct_mapping! {
                @derive_from_meta
                $(#[$($meta_struct)*])*
                $thin_struct => $d3d_struct
            }
        )*

        #[test] fn layout() {
            use std::mem::*;
            use std::ptr::addr_of;
            use $crate::macros::*;

            $(
                $(  struct_mapping!(@meta_struct_validate $($meta_struct)* );)*
                $($(struct_mapping!(@meta_field_validate  $($meta_field)* );)*)*


                let thin = MaybeUninit::<$thin_struct>::uninit();
                let d3d  = MaybeUninit::<$d3d_struct >::uninit();
                let thin = thin.as_ptr();
                let d3d  = d3d .as_ptr();

                let ignore_align = struct_mapping!(@meta_ignore_align $($($meta_struct)*)*);

                assert_eq!( size_of::<$thin_struct>(),  size_of::<$d3d_struct>(),  "size_of {} != {}", stringify!($thin_struct), stringify!($d3d_struct));
                if !ignore_align {
                    assert_eq!(align_of::<$thin_struct>(), align_of::<$d3d_struct>(), "align_of {} != {}", stringify!($thin_struct), stringify!($d3d_struct));
                }

                let same_name = {
                    let d3d = stringify!($d3d_struct).to_lowercase().replace("_","");
                    let thin = stringify!($thin_struct).to_lowercase();
                    let thin = thin.strip_suffix("<'_>").unwrap_or(thin.as_str());
                    d3d.contains(thin)
                };

                let expect_renamed = struct_mapping!(@meta_renamed $($($meta_struct)*)*);
                if expect_renamed {
                    assert!(!same_name, "{} included in {}'s name, despite being marked as #[renamed]!", stringify!($thin_struct), stringify!($d3d_struct));
                } else {
                    assert!(same_name, "{} not included in {}'s name, nor marked as #[renamed]", stringify!($thin_struct), stringify!($d3d_struct));
                }

                $(unsafe {
                    assert_eq!(size_of_val_raw_sized(addr_of!((*thin).$thin_field)), size_of_val_raw_sized(addr_of!((*d3d).$d3d_field)),   "size_of {}::{} != {}::{}", stringify!($thin_struct), stringify!($thin_field), stringify!($d3d_struct), stringify!($d3d_field));
                    assert_eq!(      offset_of(thin, addr_of!((*thin).$thin_field)),        offset_of(d3d, addr_of!((*d3d).$d3d_field)), "offset_of {}::{} != {}::{}", stringify!($thin_struct), stringify!($thin_field), stringify!($d3d_struct), stringify!($d3d_field));

                    let same_name = stringify!($d3d_field).to_lowercase().replace("_","").contains(stringify!($thin_field).strip_prefix("r#").unwrap_or(stringify!($thin_field)).to_lowercase().replace("_","").as_str());
                    let expect_renamed = struct_mapping!(@meta_renamed $($($meta_field)*)*);
                    if expect_renamed {
                        assert!(!same_name, "{} included in {}'s name, despite being marked as #[renamed]!", stringify!($thin_field), stringify!($d3d_field));
                    } else {
                        assert!(same_name, "{} not included in {}'s name, nor marked as #[renamed]", stringify!($thin_field), stringify!($d3d_field));
                    }
                })*
            )*
        }
    };

    (@meta_struct_validate                      ) => {};
    (@meta_struct_validate renamed              ) => {};
    (@meta_struct_validate derive($($d:tt)*)    ) => {};
    (@meta_struct_validate ignore(align)        ) => {};
    (@meta_struct_validate $meta:meta           ) => { panic!("unexpected struct attribute: #[{}]", stringify!($meta)); };

    (@meta_field_validate)            => {};
    (@meta_field_validate renamed   ) => {};
    (@meta_field_validate $meta:meta) => { panic!("unexpected field attribute: #[{}]", stringify!($meta)); };

    (@meta_renamed)                       => { false };
    (@meta_renamed renamed    $($tt:tt)*) => { true };
    (@meta_renamed $meta:meta $($tt:tt)*) => { struct_mapping!(@meta_renamed $($tt)*) };

    (@meta_ignore_align)                            => { false };
    (@meta_ignore_align ignore(align)   $($tt:tt)*) => { true };
    (@meta_ignore_align $meta:meta      $($tt:tt)*) => { struct_mapping!(@meta_ignore_align $($tt)*) };

    (@derive_from_meta                                                          $thin_struct:ty => $d3d_struct:ty) => {};
    (@derive_from_meta #[derive(unsafe { $($d:ident),+$(,)? })] $(#[$($next:tt)+])* $thin_struct:ty => $d3d_struct:ty) => { $( struct_mapping! { @derive unsafe $d $thin_struct => $d3d_struct } )+ struct_mapping! { @derive_from_meta $(#[$($next)+])* $thin_struct => $d3d_struct } };
    (@derive_from_meta #[$($ignore:tt)*]                        $(#[$($next:tt)+])* $thin_struct:ty => $d3d_struct:ty) => { struct_mapping! { @derive_from_meta $(#[$($next)+])* $thin_struct => $d3d_struct } };

    (@derive unsafe AsRefD3D $thin_struct:ty => $d3d_struct:ty) => {    impl AsRef<$d3d_struct> for $thin_struct { fn as_ref(&    self)         -> &    $d3d_struct     { unsafe { std::mem::transmute(self) } } } };
    (@derive unsafe AsMutD3D $thin_struct:ty => $d3d_struct:ty) => {    impl AsMut<$d3d_struct> for $thin_struct { fn as_mut(&mut self)         -> &mut $d3d_struct     { unsafe { std::mem::transmute(self) } } } }; // XXX: Pointless?
    (@derive unsafe AsRef    $thin_struct:ty => $d3d_struct:ty) => {    impl AsRef<$thin_struct> for $d3d_struct { fn as_ref(&self)             -> &$thin_struct        { unsafe { std::mem::transmute(self) } } }
                                                                        impl AsRef<$d3d_struct> for $thin_struct { fn as_ref(&self)             -> &$d3d_struct         { unsafe { std::mem::transmute(self) } } } }; // XXX: Footgun?
    (@derive unsafe AsMut    $thin_struct:ty => $d3d_struct:ty) => {    impl AsMut<$thin_struct> for $d3d_struct { fn as_mut(&mut self)         -> &mut $thin_struct    { unsafe { std::mem::transmute(self) } } }
                                                                        impl AsMut<$d3d_struct> for $thin_struct { fn as_mut(&mut self)         -> &mut $d3d_struct     { unsafe { std::mem::transmute(self) } } } };
    (@derive unsafe Deref    $thin_struct:ty => $d3d_struct:ty) => {    impl std::ops::Deref    for $thin_struct { fn deref(&self)              -> &Self::Target        { unsafe { std::mem::transmute(self) } } type Target = $d3d_struct; } };
    (@derive unsafe DerefMut $thin_struct:ty => $d3d_struct:ty) => {    impl std::ops::DerefMut for $thin_struct { fn deref_mut(&mut self)      -> &mut Self::Target    { unsafe { std::mem::transmute(self) } } } };
    (@derive unsafe FromD3D  $thin_struct:ty => $d3d_struct:ty) => {    impl From<$d3d_struct>  for $thin_struct { fn from(value: $d3d_struct)  -> Self                 { unsafe { std::mem::transmute(value) } } } };
    (@derive unsafe IntoD3D  $thin_struct:ty => $d3d_struct:ty) => {    impl From<$thin_struct> for $d3d_struct  { fn from(value: $thin_struct) -> Self                 { unsafe { std::mem::transmute(value) } } } };
    (@derive unsafe FromInto $thin_struct:ty => $d3d_struct:ty) => {
        struct_mapping! { @derive unsafe FromD3D $thin_struct => $d3d_struct }
        struct_mapping! { @derive unsafe IntoD3D $thin_struct => $d3d_struct }
    };

    // AsPtr?  AsPtrMut?
}

// XXX: Unlike the pending nightly fn, this acquires safety by sacrificing `?Sized` support.
#[cfg(test)] pub const fn size_of_val_raw_sized<T>(_: *const T) -> usize { std::mem::size_of::<T>() }
#[cfg(test)] pub fn offset_of<S, F>(s: *const S, f: *const F) -> usize { (f as usize) - (s as usize) }

/// ### Usage
/// ```no_run
/// enumish! { RustyEnum => D3D_ENUM; FQN; RustyEnum::A, RustyEnum::B, RustyEnum::C }
/// enumish! { RustyEnum => D3D_ENUM;      A, B, C }
/// enumish! { RustyEnum => D3D_ENUM }
/// ```
macro_rules! enumish {
    ( $enumish:ty => $d3d:ty; FQN; $($a:ident :: $b:ident),* $(,)? ) => {
        impl std::fmt::Debug for $enumish {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $(
                        $a::$b          => write!(f, "{}", concat!(stringify!($a), "::", stringify!($b))),
                    )*
                    other               => write!(f, "{}({})", stringify!($enumish), other.0),
                }
            }
        }

        enumish!( $enumish => $d3d );
    };
    ( $enumish:ty => $d3d:ty; FQN; $($($ident:ident)::+),* $(,)? ) => {
        impl std::fmt::Debug for $enumish {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $(
                        $($ident)::+    => write!(f, "{}", concat!(stringify!($($ident)::+))),
                    )*
                    other               => write!(f, "{}({})", stringify!($enumish), other.0),
                }
            }
        }

        enumish!( $enumish => $d3d );
    };
    ( $enumish:ty => $d3d:ty; default: $default:ident != 0; $($tt:tt)* ) => {
        impl std::default::Default for $enumish {
            fn default() -> Self { Self::$default }
        }

        const _ : () = {
            const DEFAULT : $enumish = <$enumish>::$default;
            const ZEROED  : $enumish = <$enumish>::zeroed();
            assert!(!matches!(DEFAULT, ZEROED), "default was expected to be nonzero, but was 0");
        };

        enumish!( $enumish => $d3d; $($tt)* );
    };
    ( $enumish:ty => $d3d:ty; default: $default:ident == 0; $($tt:tt)* ) => {
        impl std::default::Default for $enumish {
            fn default() -> Self { Self::$default }
        }

        const _ : () = {
            const DEFAULT : $enumish = <$enumish>::$default;
            const ZEROED  : $enumish = <$enumish>::zeroed();
            assert!(matches!(DEFAULT, ZEROED), "default was expected to be 0, but wasn't");
        };

        enumish!( $enumish => $d3d; $($tt)* );
    };
    ( $enumish:ty => $d3d:ty; default: 0; $($tt:tt)* ) => {
        impl std::default::Default for $enumish {
            fn default() -> Self { Self(0) }
        }

        enumish!( $enumish => $d3d; $($tt)* );
    };
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

        enumish!( $enumish => $d3d );
    };
    ( $enumish:ty => $d3d:ty ) => {
        impl From<$enumish> for $d3d {
            fn from(value: $enumish) -> Self { value.0.into() }
        }

        #[cfg(feature = "impl-from-unchecked")]
        impl From<$d3d> for $enumish {
            fn from(value: $d3d) -> Self { Self(value as _) }
        }

        impl $enumish {
            /// Initialize to 0.
            pub const fn zeroed() -> Self { Self(0) }

            /// Convert from an underlying [winapi] `D3D...` type.
            /// This is *probably* safe... probably...
            pub const fn from_unchecked(d3d: $d3d) -> Self { Self(d3d as _) }

            /// Convert back into an underlying [winapi] `D3D...` type.
            pub const fn into_inner(self) -> $d3d { self.0 as _ }
        }
    };
}

/// ### Usage
/// ```no_run
/// flags! { RustyFlags => D3D_FLAGS; A, B, C }
/// ```
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
                        write!(f, "{}::none()", stringify!($flagish))
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
                        } else {
                            write!(f, "}}")?;
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
            /// Initialize to 0.
            pub const fn zeroed() -> Self { Self(0) }

            /// Initialize to 0.
            pub const fn none() -> Self { Self(0) }

            /// Convert from an underlying [winapi] `D3D...` type.
            /// This is *probably* safe... probably...
            pub const fn from_unchecked(d3d: $d3d) -> Self { Self(d3d as _) }

            /// Convert back into an underlying [winapi] `D3D...` type.
            pub const fn into_inner(self) -> $d3d { self.0 as _ }
        }

        impl std::default::Default for $flagish {
            fn default() -> Self { Self::none() }
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

/// COM conversion boilerplate
///
/// ### ⚠️ Safety ⚠️
/// *   Assumes `$outer` and `$inner` are `#[repr(transparent)]` wrappers around [mcom::Rc] and ABI compatible
/// *   Typechecked via some `From` impls, but sufficiently malicious `Deref` impls might be able to defeat that.
///
/// ### Usage
/// *   `convert!(unsafe $outer => $inner, $winapi);`
/// *   `convert!(unsafe $outer,           $winapi);`
macro_rules! convert {
    ( unsafe $outer:ty => $deref:ty, $winapi:ty ) => {
        convert!(unsafe $outer, $winapi);

        impl std::ops::Deref for $outer {
            type Target = $deref;
            fn deref(&self) -> &Self::Target { self.0.up_ref().into() }
        }
    };
    ( unsafe $outer:ty, $winapi:ty ) => {
        impl From<mcom::Rc<$winapi>> for $outer { fn from(value: mcom::Rc<$winapi>) -> Self { Self(value) } }
        impl From<$outer> for mcom::Rc<$winapi> { fn from(value: $outer) -> Self { value.0 } }

        impl From<&mcom::Rc<$winapi>> for &$outer { fn from(value: &mcom::Rc<$winapi>) -> Self {
            #[allow(clippy::undocumented_unsafe_blocks)]
            // SAFETY: ✔️ sound per documented `#[repr(transparent)]` safety precondition of this `unsafe`-marked macro
            unsafe { std::mem::transmute(value) }
        }}
        impl From<&$outer> for &mcom::Rc<$winapi> { fn from(value: &$outer) -> Self {
            #[allow(clippy::undocumented_unsafe_blocks)]
            // SAFETY: ✔️ sound per documented `#[repr(transparent)]` safety precondition of this `unsafe`-marked macro
            unsafe { std::mem::transmute(value) }
        }}

        unsafe impl $crate::Raw for $outer {
            type Raw = $winapi;

            unsafe fn from_raw(raw: *mut Self::Raw) -> Self { Self(unsafe { mcom::Rc::from_raw(raw) }) }
            unsafe fn from_raw_opt(raw: *mut Self::Raw) -> Option<Self> { Some(Self(unsafe { mcom::Rc::from_raw_opt(raw) }?)) }
            fn into_raw(self) -> *mut Self::Raw { self.0.into_raw() }
            fn as_raw(&self) -> *mut Self::Raw { self.0.as_ptr() }
        }
    };
}

macro_rules! mods {
    ( $( #[$attr:meta] )* inl      mod $mod:ident ;                $($tt:tt)* ) => { $(#[$attr])* pub(crate) mod $mod;                       #[allow(unused_imports)] pub use $mod::*; mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* inl      mod $mod:ident { $($body:tt)* } $($tt:tt)* ) => { $(#[$attr])* pub(crate) mod $mod { mods!{ $($body)* } } #[allow(unused_imports)] pub use $mod::*; mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* $vis:vis mod $mod:ident ;                $($tt:tt)* ) => { $(#[$attr])* $vis mod $mod;                                                                 mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* $vis:vis mod $mod:ident { $($body:tt)* } $($tt:tt)* ) => { $(#[$attr])* $vis mod $mod { mods!{ $($body)* } }                                           mods!{ $($tt)* } };
    () => {};
}
