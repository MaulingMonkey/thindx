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
