#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfillmode)\]
/// D3DFILLMODE
///
/// Defines constants describing the fill mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FillMode(D3DFILLMODE);
pub use FillMode as Fill;

enumish! { Fill => D3DFILLMODE; Point, Wireframe, Solid }

#[allow(non_upper_case_globals)] impl Fill { // These are enum-like
    pub const Point         : Fill = Fill(D3DFILL_POINT); // 1
    pub const Wireframe     : Fill = Fill(D3DFILL_WIREFRAME);
    pub const Solid         : Fill = Fill(D3DFILL_SOLID);
}

impl Default for Fill {
    fn default() -> Self { Fill::Solid } // 3
}
