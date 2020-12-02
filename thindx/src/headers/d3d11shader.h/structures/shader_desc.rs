use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_desc)\]
/// D3D11_SHADER_DESC
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct ShaderDesc<'s> {
    pub version:                        u32,
    pub creator:                        Option<&'s AbiCStr>,
    pub flags:                          u32, // Compile?  docs aren't clear
    pub constant_buffers:               u32,
    pub bound_resources:                u32,
    pub input_parameters:               u32,
    pub output_parameters:              u32,
    pub instruction_count:              u32,
    pub temp_register_count:            u32,
    pub temp_array_count:               u32,
    pub def_count:                      u32,
    pub dcl_count:                      u32,
    pub texture_normal_instructions:    u32,
    pub texture_load_instructions:      u32,
    pub texture_comp_instructions:      u32,
    pub texture_bias_instructions:      u32,
    pub texture_gradient_instructions:  u32,
    pub float_instruction_count:        u32,
    pub int_instruction_count:          u32,
    pub uint_instruction_count:         u32,
    pub static_flow_control_count:      u32,
    pub dynamic_flow_control_count:     u32,
    pub macro_instruction_count:        u32,
    pub array_instruction_count:        u32,
    pub cut_instruction_count:          u32,
    pub emit_instruction_count:         u32,
    pub gs_output_topology:             PrimitiveTopology,
    pub gs_max_output_vertex_count:     u32,
    pub input_primitive:                Primitive,
    pub patch_constant_parameters:      u32,
    pub gs_instance_count:              u32,
    pub control_points:                 u32,
    pub hs_output_primitive:            TessellatorOutputPrimitive,
    pub hs_partitioning:                TessellatorPartitioning,
    pub tessellator_domain:             TessellatorDomain,
    pub barrier_instructions:           u32,
    pub interlocked_instructions:       u32,
    pub texture_store_instructions:     u32,
}

impl ShaderDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_SHADER_DESC {
        self as *const Self as *mut Self as *mut _
    }
}

test_layout! { ShaderDesc => unsafe D3D11_SHADER_DESC {
    version                         => Version,
    creator                         => Creator,
    flags                           => Flags,
    constant_buffers                => ConstantBuffers,
    bound_resources                 => BoundResources,
    input_parameters                => InputParameters,
    output_parameters               => OutputParameters,
    instruction_count               => InstructionCount,
    temp_register_count             => TempRegisterCount,
    temp_array_count                => TempArrayCount,
    def_count                       => DefCount,
    dcl_count                       => DclCount,
    texture_normal_instructions     => TextureNormalInstructions,
    texture_load_instructions       => TextureLoadInstructions,
    texture_comp_instructions       => TextureCompInstructions,
    texture_bias_instructions       => TextureBiasInstructions,
    texture_gradient_instructions   => TextureGradientInstructions,
    float_instruction_count         => FloatInstructionCount,
    int_instruction_count           => IntInstructionCount,
    uint_instruction_count          => UintInstructionCount,
    static_flow_control_count       => StaticFlowControlCount,
    dynamic_flow_control_count      => DynamicFlowControlCount,
    macro_instruction_count         => MacroInstructionCount,
    array_instruction_count         => ArrayInstructionCount,
    cut_instruction_count           => CutInstructionCount,
    emit_instruction_count          => EmitInstructionCount,
    gs_output_topology              => GSOutputTopology,
    gs_max_output_vertex_count      => GSMaxOutputVertexCount,
    input_primitive                 => InputPrimitive,
    patch_constant_parameters       => PatchConstantParameters,
    gs_instance_count               => cGSInstanceCount,
    control_points                  => cControlPoints,
    hs_output_primitive             => HSOutputPrimitive,
    hs_partitioning                 => HSPartitioning,
    tessellator_domain              => TessellatorDomain,
    barrier_instructions            => cBarrierInstructions,
    interlocked_instructions        => cInterlockedInstructions,
    texture_store_instructions      => cTextureStoreInstructions,
}}
