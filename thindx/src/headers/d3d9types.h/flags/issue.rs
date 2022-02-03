#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;
type D3DISSUE = u32; // there's no actual type



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlock)\]
/// DWORD / D3DISSUE_*
///
/// Controls how [IDirect3DQuery9Ext::issue] behaves.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Issue(D3DISSUE);

flags! { Issue => D3DISSUE; None, Begin, End }

#[allow(non_upper_case_globals)] impl Issue { // These are enum-like
    /// No flags
    pub const None          : Issue = Issue(0);

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dissue-begin)\]
    /// D3DISSUE_BEGIN
    ///
    /// [Issue::Begin] is valid for [QueryType::Occlusion].
    pub const Begin         : Issue = Issue(D3DISSUE_BEGIN);

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dissue-end)\]
    /// D3DISSUE_END
    ///
    /// [Issue::End] is valid for [QueryType::VCache], [QueryType::ResourceManager], [QueryType::VertexStats], [QueryType::Event], and [QueryType::Occlusion]
    pub const End           : Issue = Issue(D3DISSUE_END);
}

//#cpp2rust D3DISSUE_BEGIN  = d3d::Issue::Begin
//#cpp2rust D3DISSUE_END    = d3d::Issue::End
