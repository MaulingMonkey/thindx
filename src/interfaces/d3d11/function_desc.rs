use crate::*;

#[allow(unused_imports)] use winapi::um::d3dcommon::*;
#[allow(unused_imports)] use winapi::um::d3dcompiler::*;
#[allow(unused_imports)] use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_function_desc)\]
/// D3D11_FUNCTION_DESC
///
/// &amp;\[[u8]\] equivalent that's ABI-compatible with some D3D APIs
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct FunctionDesc<'s> {
    version:                        u32,
    creator:                        Option<CStrPtr<'s>>,
    flags:                          u32,
    constant_buffers:               u32,
    bound_resources:                u32,
    instruction_count:              u32,
    temp_register_count:            u32,
    temp_array_count:               u32,
    def_count:                      u32,
    dcl_count:                      u32,
    texture_normal_nstructions:     u32,
    texture_load_instructions:      u32,
    texture_comp_instructions:      u32,
    texture_bias_instructions:      u32,
    texture_gradient_nstructions:   u32,
    float_instruction_count:        u32,
    int_instruction_count:          u32,
    uint_instruction_count:         u32,
    static_flow_control_count:      u32,
    dynamic_flow_control_count:     u32,
    macro_instruction_count:        u32,
    array_instruction_count:        u32,
    mov_instruction_count:          u32,
    movc_instruction_count:         u32,
    conversion_instruction_count:   u32,
    bitwise_instruction_count:      u32,
    min_feature_level:              FeatureLevel,
    required_feature_flags:         u64,
    name:                           Option<CStrPtr<'s>>,
    function_parameter_count:       i32,
    has_return:                     bool32,
    has_10_level_9_vertex_shader:   bool32,
    has_10_level_9_pixel_shader:    bool32,
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
