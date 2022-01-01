mods! {
    #[path=r"dll\_dll.rs"] inl mod dll;

    inl mod constants {
        inl mod compile;
        inl mod compile_effect;
        inl mod compile_secdata;
        inl mod compress_shader;
        inl mod disasm;
        inl mod get_inst_offsets;
        inl mod shader_requires;
    }

    inl mod enumerations {
        inl mod blob_part;
    }

    inl mod flags {
        inl mod compiler_strip_flags;
    }

    inl mod structures {
        inl mod shader_data;
    }
}
