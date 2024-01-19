use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use winapi::um::d3d11shader::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_shader_desc)\]
/// D3D11_SHADER_DESC
///
/// ### See Also
/// *   [d3d11::ShaderReflection::get_desc]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ShaderDesc<'s> {
    pub version:                        d3d11::ShaderVersion,
    /// e.g. "Microsoft (R) HLSL Shader Compiler 10.1"
    pub creator:                        CStrPtr<'s>,
    pub flags:                          Compile, // docs aren't super clear, but in testing this appears to be the right flag type
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

struct_mapping! {
    #[derive(unsafe { AsRefD3D, IntoD3D })]
    // forbidden: AsRef     (could invalidate `creator`)
    // forbidden: AsMut     (could invalidate `creator`)
    // forbidden: DerefMut  (could invalidate `creator`)
    // forbidden: FromD3D   (could invalidate `creator`)
    ShaderDesc<'_> => D3D11_SHADER_DESC {
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
    }
}

//#cpp2rust D3D11_SHADER_DESC                       = d3d11::ShaderDesc
