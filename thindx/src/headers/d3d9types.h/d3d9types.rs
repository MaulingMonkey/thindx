// XXX: temporary attributes
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(missing_docs)]

pub use enumerations::*;
pub use flags::*;
pub use structures::*;
pub use values::*;

mod enumerations {
    mod back_buffer_type;                   pub use back_buffer_type::*;
    mod basis_type;                         pub use basis_type::*;
    mod blend_op;                           pub use blend_op::*;
    mod blend;                              pub use blend::*;
    mod cmp_func;                           pub use cmp_func::*;
    mod compose_rects_op;                   pub use compose_rects_op::*;
    mod cube_map_faces;                     pub use cube_map_faces::*;
    mod cull;                               pub use cull::*;
    mod debug_monitor_tokens;               pub use debug_monitor_tokens::*;
    mod decl_method;                        pub use decl_method::*;
    mod decl_type;                          pub use decl_type::*;
    mod decl_usage;                         pub use decl_usage::*;
    mod degree;                             pub use degree::*;
    mod devtype;                            pub use devtype::*;
    mod display_rotation;                   pub use display_rotation::*;
    mod fill_mode;                          pub use fill_mode::*;
    mod fog_mode;                           pub use fog_mode::*;
    mod format;                             pub use format::*;
    mod light_type;                         pub use light_type::*;
    mod material_color_source;              pub use material_color_source::*;
    mod multi_sample_type;                  pub use multi_sample_type::*;
    mod pool;                               pub use pool::*;
    mod patch_edge_style;                   pub use patch_edge_style::*;
    mod primitive_type;                     pub use primitive_type::*;
    mod query_type;                         pub use query_type::*;
    mod render_state_type;                  pub use render_state_type::*;
    mod resource_type;                      pub use resource_type::*;
    mod sampler_state_type;                 pub use sampler_state_type::*;
    mod sampler_texture_type;               pub use sampler_texture_type::*;
    mod scanline_ordering;                  pub use scanline_ordering::*;
    mod sgr;                                pub use sgr::*;
    mod shade_mode;                         pub use shade_mode::*;
    mod state_block_type;                   pub use state_block_type::*;
    mod stencil_op;                         pub use stencil_op::*;
    mod stream_source;                      pub use stream_source::*;
    mod swap_effect;                        pub use swap_effect::*;
    mod texture_address;                    pub use texture_address::*;
    mod texture_filter_type;                pub use texture_filter_type::*;
    mod texture_op;                         pub use texture_op::*;
    mod texture_stage_state_type;           pub use texture_stage_state_type::*;
    mod transform_state_type;               pub use transform_state_type::*;
    mod zbuffer_type;                       pub use zbuffer_type::*;
}

mod flags {
    mod create;                             pub use create::*;
    mod fvf;                                pub use fvf::*;
    mod get_data;                           pub use get_data::*;
    mod issue;                              pub use issue::*;
    mod lock;                               pub use lock::*;
    mod present;                            pub use present::*;
    mod usage;                              pub use usage::*;
}

mod structures {
    mod adapter_identifier;                 pub use adapter_identifier::*;
    mod box_;                               pub use box_::*;
    mod caps;                               pub use caps::*;
    mod clip_status;                        pub use clip_status::*;
    mod color_value;                        pub use color_value::*;
    mod display_mode_ex;                    pub use display_mode_ex::*;
    mod display_mode;                       pub use display_mode::*;
    mod index_buffer_desc;                  pub use index_buffer_desc::*;
    mod light;                              pub use light::*;
    mod material;                           pub use material::*;
    mod present_stats;                      pub use present_stats::*;
    mod raster_status;                      pub use raster_status::*;
    mod rect;                               pub use rect::*;
    mod surface_desc;                       pub use surface_desc::*;
    mod vector;                             pub use vector::*;
    mod vertex_buffer_desc;                 pub use vertex_buffer_desc::*;
    mod vertex_element;                     pub use vertex_element::*;
    mod viewport;                           pub use viewport::*;
    mod volume_desc;                        pub use volume_desc::*;
}

mod values {
    mod color;                              pub use color::*;
    mod luid;                               pub use luid::*;
    mod sdk_version;                        pub use sdk_version::*;
}
