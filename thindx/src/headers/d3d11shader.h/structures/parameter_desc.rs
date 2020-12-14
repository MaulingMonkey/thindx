#[allow(unused_imports)] use crate::*;
use crate::ctypes::*;
use crate::d3d::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/ns-d3d11shader-d3d11_parameter_desc)\]
/// D3D11_PARAMETER_DESC
///
/// ### See Also
/// *   [d3d11::FunctionLinkingGraph] for examples
/// *   [d3d11::FunctionLinkingGraph::set_input_signature]
/// *   [d3d11::FunctionLinkingGraph::set_output_signature]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct ParameterDesc<'s> {
    pub name:                   Option<CStrNonNull<'s>>,
    /// An input semantic name + index such as "POSITION0", "TEXCOORD0", "TEXCOORD1", etc.
    pub semantic_name:          Option<CStrNonNull<'s>>,
    pub ty:                     ShaderVariableType,
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
    /// ### Arguments
    /// *   `name`                  - The name of the parameter (optional - for debugging / readable codegen.)
    /// *   `semantic_name`         - A semantic name + index such as `"POSITION0"`, `"TEXCOORD3"`, etc.
    /// *   `ty`                    - The type of individual components of the parameter (ex: [`SVT::Float`].)
    /// *   `class`                 - The class of the parameter (ex: [`SVC::Scalar`], [`SVC::Vector`], [`SVC::MatrixRows`], ...)
    /// *   `rows`                  - The number of rows for a matrix (otherwise typically `1`.)
    /// *   `columns`               - The number of columns for a matrix (or vector components, or `1` for a scalar.)
    /// *   `interpolation_mode`    - How the parameter is interpolated during rasterization.
    /// *   `flags`                 - Parameter flags (ex: <code>[PF::In] | [PF::Out]</code>.)
    /// *   `first_in_register`     - The first register to read this parameter from (or `0` if auto-assigning.)
    /// *   `first_in_component`    - The first component of the register to read this parameter from (`0`=`x`=`r`, `1`=`y`=`g`, ...? - or `0` if auto-assigning.)
    /// *   `first_out_register`    - The first register to write this parameter to (or `0` if auto-assigning.)
    /// *   `first_out_component`   - The first component of the register to write this parameter to (`0`=`x`=`r`, `1`=`y`=`g`, ...? - or `0` if auto-assigning.)
    ///
    /// ### Example
    /// ```rust
    /// use thindx::{cstr, d3d::{SVT, SVC, Interpolation, PF}, d3d11::ParameterDesc};
    /// let parameters = [
    ///     ParameterDesc::new(cstr!("inputPos"),  cstr!("POSITION0"), SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("inputTex"),  cstr!("TEXCOORD0"), SVT::Float, SVC::Vector, 1, 2, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    ///     ParameterDesc::new(cstr!("inputNorm"), cstr!("NORMAL0"),   SVT::Float, SVC::Vector, 1, 3, Interpolation::Linear, PF::In, 0, 0, 0, 0),
    /// ];
    /// ```
    pub fn new(
        name:                   impl Into<Option<CStrNonNull<'s>>>,
        semantic_name:          impl Into<CStrNonNull<'s>>, // Seems fairly mandatory in testing
        ty:                     ShaderVariableType,
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
            ty, class, rows, columns, interpolation_mode, flags,
            first_in_register, first_in_component, first_out_register, first_out_component,
        }
    }
}

test_layout! { ParameterDesc => unsafe winapi::um::d3d11shader::D3D11_PARAMETER_DESC {
    name                    => Name,
    semantic_name           => SemanticName,
    ty                      => Type,
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
