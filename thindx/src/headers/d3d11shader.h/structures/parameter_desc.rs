use crate::*;

use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_parameter_desc)\]
/// D3D11_PARAMETER_DESC
///
/// ### See Also
/// * [d3d11::FunctionLinkingGraph] for examples
/// * [d3d11::FunctionLinkingGraph::set_input_signature]
/// * [d3d11::FunctionLinkingGraph::set_output_signature]
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct ParameterDesc<'s> {
    pub name:                   Option<&'s AbiCStr>,
    /// An input semantic name such as "POSITION0", "TEXCOORD0", "TEXCOORD1", etc.
    pub semantic_name:          Option<&'s AbiCStr>,
    pub type_:                  ShaderVariableType,
    pub class:                  ShaderVariableClass,
    pub rows:                   u32,
    pub columns:                u32,
    pub interpolation_mode:     InterpolationMode,
    pub flags:                  ParameterFlags,
    pub first_in_register:      u32,
    pub first_in_component:     u32,
    pub first_out_register:     u32,
    pub first_out_component:    u32,
}

impl<'s> ParameterDesc<'s> {
    /// Shorthand construction for ParamterDesc structs.
    ///
    /// ### Example
    /// ```rust
    /// use thindx::{cstr, SVT, SVC, Interpolation, PF, d3d11::ParameterDesc};
    /// let parameters = [
    ///     ParameterDesc::new(cstr!("inputPos"),  cstr!("POSITION0"), SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("inputTex"),  cstr!("TEXCOORD0"), SVT::Float, SVC::Vector, 1, 2, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("inputNorm"), cstr!("NORMAL0"),   SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    /// ];
    /// ```
    pub fn new(
        name:                   impl Into<Option<&'s AbiCStr>>,
        semantic_name:          impl Into<&'s AbiCStr>, // Seems fairly mandatory in testing
        type_:                  ShaderVariableType,
        class:                  ShaderVariableClass,
        rows:                   u32,
        columns:                u32,
        interpolation_mode:     InterpolationMode,
        flags:                  ParameterFlags,
        first_in_register:      u32,
        first_in_component:     u32,
        first_out_register:     u32,
        first_out_component:    u32,
    ) -> Self {
        Self {
            name:           name.into(),
            semantic_name:  Some(semantic_name.into()),
            type_, class, rows, columns, interpolation_mode, flags,
            first_in_register, first_in_component, first_out_register, first_out_component,
        }
    }
}

test_layout! { ParameterDesc => unsafe D3D11_PARAMETER_DESC {
    name                    => Name,
    semantic_name           => SemanticName,
    type_                   => Type,
    class                   => Class,
    rows                    => Rows,
    columns                 => Columns,
    interpolation_mode      => InterpolationMode,
    flags                   => Flags,
    first_in_register       => FirstInRegister,
    first_in_component      => FirstInComponent,
    first_out_register      => FirstOutRegister,
    first_out_component     => FirstOutComponent,
}}
