#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;
type D3DGETDATA = u32; // there's no actual type



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dgetdata-flush)\]
/// DWORD / D3DGETDATA_*
///
/// Controls how [IDirect3DQuery9Ext::get_data_inplace] behaves.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct GetData(D3DGETDATA);

flags! { GetData => D3DGETDATA; None, Flush }

#[allow(non_upper_case_globals)] impl GetData { // These are enum-like
    /// No flags
    pub const None      : GetData = GetData(0);

    /// [D3DGETDATA_FLUSH](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dgetdata-flush)
    pub const Flush     : GetData = GetData(D3DGETDATA_FLUSH);
}

//#cpp2rust D3DGETDATA_FLUSH    = d3d::GetData::Flush
