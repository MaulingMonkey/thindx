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



mod adapter_identifier;     pub use adapter_identifier::*;
mod r#box;                  pub use r#box::*;
mod caps;                   pub use caps::*;
mod clip_status;            pub use clip_status::*;
mod color_value;            pub use color_value::*;
mod display_mode;           pub use display_mode::*;
mod index_buffer_desc;      pub use index_buffer_desc::*;
mod light;                  pub use light::*;
mod material;               pub use material::*;
mod rect;                   pub use rect::*;
mod surface_desc;           pub use surface_desc::*;
mod vector;                 pub use vector::*;
mod vertex_buffer_desc;     pub use vertex_buffer_desc::*;
mod vertex_element;         pub use vertex_element::*;
mod viewport;               pub use viewport::*;
mod volume_desc;            pub use volume_desc::*;
