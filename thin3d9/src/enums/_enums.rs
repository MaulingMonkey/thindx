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
            fn from(value: $d3d) -> Self { Self(value as _) }
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



mod back_buffer_type;           pub use back_buffer_type::*;
mod basis_type;                 pub use basis_type::*;
mod blend_op;                   pub use blend_op::*;
mod blend;                      pub use blend::*;
mod cmp_func;                   pub use cmp_func::*;
mod compose_rects_op;           pub use compose_rects_op::*;
mod cube_map_faces;             pub use cube_map_faces::*;
mod cull;                       pub use cull::*;
mod d3derr;                     pub use d3derr::*;
mod debug_monitor_tokens;       pub use debug_monitor_tokens::*;
mod decl_method;                pub use decl_method::*;
mod decl_type;                  pub use decl_type::*;
mod decl_usage;                 pub use decl_usage::*;
mod degree;                     pub use degree::*;
mod devtype;                    pub use devtype::*;
mod display_rotation;           pub use display_rotation::*;
mod fill_mode;                  pub use fill_mode::*;
mod fog_mode;                   pub use fog_mode::*;
mod format;                     pub use format::*;
mod light_type;                 pub use light_type::*;
mod material_color_source;      pub use material_color_source::*;
mod multi_sample_type;          pub use multi_sample_type::*;
mod pool;                       pub use pool::*;
mod patch_edge_style;           pub use patch_edge_style::*;
mod primitive_type;             pub use primitive_type::*;
mod query_type;                 pub use query_type::*;
mod render_state_type;          pub use render_state_type::*;
mod resource_type;              pub use resource_type::*;
mod sampler_state_type;         pub use sampler_state_type::*;
mod sampler_texture_type;       pub use sampler_texture_type::*;
mod scanline_ordering;          pub use scanline_ordering::*;
mod sgr;                        pub use sgr::*;
mod shade_mode;                 pub use shade_mode::*;
mod state_block_type;           pub use state_block_type::*;
mod stencil_op;                 pub use stencil_op::*;
mod stream_source;              pub use stream_source::*;
mod swap_effect;                pub use swap_effect::*;
mod texture_address;            pub use texture_address::*;
mod texture_filter_type;        pub use texture_filter_type::*;
mod texture_op;                 pub use texture_op::*;
mod texture_stage_state_type;   pub use texture_stage_state_type::*;
mod transform_state_type;       pub use transform_state_type::*;
mod zbuffer_type;               pub use zbuffer_type::*;
