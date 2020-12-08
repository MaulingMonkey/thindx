//! Use [d3d::Compiler] to inspect shader bytecode
use thindx::*;
use thindx::d3d::*;
//use thindx::d3d11::*;

fn dump_library() {
    let compiler = d3d::Compiler::new(47).unwrap();
    let library = compiler.compile_from_file(r"test\data\library.hlsl", None, None, (), "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
    let _library = compiler.reflect_library::<d3d11::LibraryReflection>(&library).unwrap();
    let library  = compiler.reflect_library_11(&library).unwrap(); // equivalent shorthand

    println!("library");
    println!("=======");
    println!("{:?}\n", library.get_desc().unwrap());
    for function in library.functions().unwrap() {
        let desc = function.get_desc().unwrap();
        println!("{:#?}\n", desc);
        // TODO: plenty
    }
}

fn dump_shader() {
    let compiler = d3d::Compiler::new(47).unwrap();
    let shader  = compiler.compile_from_file(r"test\data\basic.hlsl", None, None, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let _shader = compiler.reflect::<d3d11::ShaderReflection>(&shader).unwrap();
    let shader  = compiler.reflect11(&shader).unwrap(); // equivalent shorthand

    let desc = shader.get_desc().unwrap();
    println!("shader");
    println!("======");
    println!(".get_bitwise_instruction_count()      == {:?}", shader.get_bitwise_instruction_count());
    println!(".get_conversion_instruction_count()   == {:?}", shader.get_conversion_instruction_count());
    println!(".get_movc_instruction_count()         == {:?}", shader.get_movc_instruction_count());
    println!(".get_mov_instruction_count()          == {:?}", shader.get_mov_instruction_count());
    println!(".get_gs_input_primitive()             == {:?}", shader.get_gs_input_primitive());
    println!(".get_min_feature_level()              == {:?}", shader.get_min_feature_level());
    println!(".get_num_interface_slots()            == {:?}", shader.get_num_interface_slots());
    println!(".get_requires_flags()                 == {:?}", shader.get_requires_flags());
    println!(".get_thread_group_size()              == {:?}", shader.get_thread_group_size());
    println!(".is_sample_frequency_shader()         == {:?}", shader.is_sample_frequency_shader());
    println!(".get_input_parameter_desc(..) = {:#?}",  (0..desc.input_parameters            ).map(|i| shader.get_input_parameter_desc(i)            ).collect::<Vec<_>>());
    println!(".get_output_parameter_desc(..) = {:#?}", (0..desc.output_parameters           ).map(|i| shader.get_output_parameter_desc(i)           ).collect::<Vec<_>>());
    println!(".get_patch_parameter_desc(..) = {:#?}",  (0..desc.patch_constant_parameters   ).map(|i| shader.get_patch_constant_parameter_desc(i)   ).collect::<Vec<_>>());
    println!(".get_desc() == {:#?}", desc);
    // TODO: get_constant_buffer_by_index
    // TODO: get_constant_buffer_by_name
    // TODO: get_resource_binding_desc
    // TODO: get_resource_binding_desc_by_name
    // TODO: get_variable_by_name
}

fn main() {
    dump_library();
    print!("\n\n\n");
    dump_shader();
}
