use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_function_desc)\]
/// D3D11_FUNCTION_DESC
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct FunctionDesc<'s> {
    pub version:                        u32,
    pub creator:                        Option<&'s AbiCStr>,
    pub flags:                          Compile,
    pub constant_buffers:               u32,
    pub bound_resources:                u32,
    pub instruction_count:              u32,
    pub temp_register_count:            u32,
    pub temp_array_count:               u32,
    pub def_count:                      u32,
    pub dcl_count:                      u32,
    pub texture_normal_nstructions:     u32,
    pub texture_load_instructions:      u32,
    pub texture_comp_instructions:      u32,
    pub texture_bias_instructions:      u32,
    pub texture_gradient_nstructions:   u32,
    pub float_instruction_count:        u32,
    pub int_instruction_count:          u32,
    pub uint_instruction_count:         u32,
    pub static_flow_control_count:      u32,
    pub dynamic_flow_control_count:     u32,
    pub macro_instruction_count:        u32,
    pub array_instruction_count:        u32,
    pub mov_instruction_count:          u32,
    pub movc_instruction_count:         u32,
    pub conversion_instruction_count:   u32,
    pub bitwise_instruction_count:      u32,
    pub min_feature_level:              FeatureLevel,
    pub required_feature_flags:         ShaderRequires,
    pub name:                           Option<&'s AbiCStr>,
    pub function_parameter_count:       i32,
    pub has_return:                     bool32,
    pub has_10_level_9_vertex_shader:   bool32,
    pub has_10_level_9_pixel_shader:    bool32,
}

impl FunctionDesc<'_> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut D3D11_FUNCTION_DESC {
        self as *mut Self as *mut _
    }
}

test_layout! { FunctionDesc => unsafe D3D11_FUNCTION_DESC {
    version                         => Version,
    creator                         => Creator,
    flags                           => Flags,
    constant_buffers                => ConstantBuffers,
    bound_resources                 => BoundResources,
    instruction_count               => InstructionCount,
    temp_register_count             => TempRegisterCount,
    temp_array_count                => TempArrayCount,
    def_count                       => DefCount,
    dcl_count                       => DclCount,
    texture_normal_nstructions      => TextureNormalInstructions,
    texture_load_instructions       => TextureLoadInstructions,
    texture_comp_instructions       => TextureCompInstructions,
    texture_bias_instructions       => TextureBiasInstructions,
    texture_gradient_nstructions    => TextureGradientInstructions,
    float_instruction_count         => FloatInstructionCount,
    int_instruction_count           => IntInstructionCount,
    uint_instruction_count          => UintInstructionCount,
    static_flow_control_count       => StaticFlowControlCount,
    dynamic_flow_control_count      => DynamicFlowControlCount,
    macro_instruction_count         => MacroInstructionCount,
    array_instruction_count         => ArrayInstructionCount,
    mov_instruction_count           => MovInstructionCount,
    movc_instruction_count          => MovcInstructionCount,
    conversion_instruction_count    => ConversionInstructionCount,
    bitwise_instruction_count       => BitwiseInstructionCount,
    min_feature_level               => MinFeatureLevel,
    required_feature_flags          => RequiredFeatureFlags,
    name                            => Name,
    function_parameter_count        => FunctionParameterCount,
    has_return                      => HasReturn,
    has_10_level_9_vertex_shader    => Has10Level9VertexShader,
    has_10_level_9_pixel_shader     => Has10Level9PixelShader,
}}
