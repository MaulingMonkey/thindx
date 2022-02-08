// XXX: temporary attributes
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(missing_docs)]

mods! {
    inl mod enumerations {
        inl mod back_buffer_type;
        inl mod basis_type;
        inl mod blend_op;
        inl mod blend;
        inl mod cmp_func;
        inl mod compose_rects_op;
        inl mod cube_map_faces;
        inl mod cull;
        inl mod debug_monitor_tokens;
        inl mod decl_method;
        inl mod decl_type;
        inl mod decl_usage;
        inl mod degree;
        inl mod devtype;
        inl mod display_rotation;
        inl mod fill_mode;
        inl mod fog_mode;
        inl mod format;
        inl mod light_type;
        inl mod material_color_source;
        inl mod multi_sample_type;
        inl mod pool;
        inl mod patch_edge_style;
        inl mod primitive_type;
        inl mod query_type;
        inl mod render_state_type;
        inl mod resource_type;
        inl mod sampler_state_type;
        inl mod sampler_state_value;
        inl mod sampler_texture_type;
        inl mod scanline_ordering;
        inl mod sgr;
        inl mod shade_mode;
        inl mod state_block_type;
        inl mod stencil_op;
        inl mod stream_source;
        inl mod swap_effect;
        inl mod texture_address;
        inl mod texture_filter_type;
        inl mod texture_op;
        inl mod texture_stage_state_type;
        inl mod transform_state_type;
        inl mod zbuffer_type;
    }

    inl mod flags {
        inl mod create;
        inl mod fvf;
        inl mod get_data;
        inl mod issue;
        inl mod lock;
        inl mod present;
        inl mod present_flag;
        inl mod usage;
    }

    inl mod macros {
        inl mod makefourcc;
    }

    inl mod structures {
        inl mod adapter_identifier;
        inl mod box_;
        inl mod clip_status;
        inl mod color_value;
        inl mod device_creation_parameters;
        inl mod display_mode_ex;
        inl mod display_mode_filter;
        inl mod display_mode;
        inl mod index_buffer_desc;
        inl mod light;
        inl mod locked_box;
        inl mod locked_rect;
        inl mod material;
        inl mod matrix;
        inl mod present_stats;
        inl mod raster_status;
        inl mod rect;
        inl mod surface_desc;
        inl mod vector;
        inl mod vertex_buffer_desc;
        inl mod vertex_element;
        inl mod viewport;
        inl mod volume_desc;
    }

    inl mod values {
        inl mod color;
        inl mod luid;
        inl mod sdk_version;
        inl mod shader_version;
    }
}
