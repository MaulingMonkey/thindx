use crate::*;
use crate::ctypes::*;
use crate::d3d::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_function_desc)\]
/// D3D11_FUNCTION_DESC
///
/// ### Example
/// ```text
/// FunctionDesc {
///     version: 4293918800,
///     creator: Some("Microsoft (R) HLSL Shader Compiler 10.1"),
///     flags: Compile::{Debug|NoPreshader},
///     constant_buffers: 1,
///     bound_resources: 1,
///     instruction_count: 2,
///     temp_register_count: 0,
///     temp_array_count: 0,
///     def_count: 0,
///     dcl_count: 2,
///     texture_normal_instructions: 0,
///     texture_load_instructions: 0,
///     texture_comp_instructions: 0,
///     texture_bias_instructions: 0,
///     texture_gradient_instructions: 0,
///     float_instruction_count: 1,
///     int_instruction_count: 0,
///     uint_instruction_count: 0,
///     static_flow_control_count: 1,
///     dynamic_flow_control_count: 0,
///     macro_instruction_count: 0,
///     array_instruction_count: 0,
///     mov_instruction_count: 0,
///     movc_instruction_count: 0,
///     conversion_instruction_count: 0,
///     bitwise_instruction_count: 0,
///     min_feature_level: FeatureLevel::_11_0,
///     required_feature_flags: ShaderRequires::None,
///     name: Some("scale4"),
///     function_parameter_count: 1,
///     has_return: true,
///     has_10_level_9_vertex_shader: false,
///     has_10_level_9_pixel_shader: false,
/// }
/// ```
///
/// ### See Also
/// *   [d3d11::FunctionReflection::get_desc]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct FunctionDesc<'s> {
    pub version:                        u32,
    pub creator:                        ConstCStrPtrNullIsEmpty<'s>, // maybe never null?
    pub flags:                          Compile,
    pub constant_buffers:               u32,
    pub bound_resources:                u32,
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
    pub mov_instruction_count:          u32,
    pub movc_instruction_count:         u32,
    pub conversion_instruction_count:   u32,
    pub bitwise_instruction_count:      u32,
    pub min_feature_level:              FeatureLevel,
    pub required_feature_flags:         ShaderRequires,
    pub name:                           ConstCStrPtrNullIsEmpty<'s>, // maybe never null?
    pub function_parameter_count:       i32,
    pub has_return:                     BOOL,
    pub has_10_level_9_vertex_shader:   BOOL,
    pub has_10_level_9_pixel_shader:    BOOL,
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
