use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DPCMPCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct PCmpCaps(DWORD);

flags! {
    PCmpCaps => DWORD;
    None, Always, Equal, Greater, GreaterEqual, Less, LessEqual, Never, NotEqual,
}

#[allow(non_upper_case_globals)] impl PCmpCaps {
    pub const None                          : PCmpCaps = PCmpCaps(0);
    pub const Always                        : PCmpCaps = PCmpCaps(D3DPCMPCAPS_ALWAYS);
    pub const Equal                         : PCmpCaps = PCmpCaps(D3DPCMPCAPS_EQUAL);
    pub const Greater                       : PCmpCaps = PCmpCaps(D3DPCMPCAPS_GREATER);
    pub const GreaterEqual                  : PCmpCaps = PCmpCaps(D3DPCMPCAPS_GREATEREQUAL);
    pub const Less                          : PCmpCaps = PCmpCaps(D3DPCMPCAPS_LESS);
    pub const LessEqual                     : PCmpCaps = PCmpCaps(D3DPCMPCAPS_LESSEQUAL);
    pub const Never                         : PCmpCaps = PCmpCaps(D3DPCMPCAPS_NEVER);
    pub const NotEqual                      : PCmpCaps = PCmpCaps(D3DPCMPCAPS_NOTEQUAL);
}

//#cpp2rust D3DPCMPCAPS_ALWAYS          = d3d::PCmpCaps::Always
//#cpp2rust D3DPCMPCAPS_EQUAL           = d3d::PCmpCaps::Equal
//#cpp2rust D3DPCMPCAPS_GREATER         = d3d::PCmpCaps::Greater
//#cpp2rust D3DPCMPCAPS_GREATEREQUAL    = d3d::PCmpCaps::GreaterEqual
//#cpp2rust D3DPCMPCAPS_LESS            = d3d::PCmpCaps::Less
//#cpp2rust D3DPCMPCAPS_LESSEQUAL       = d3d::PCmpCaps::LessEqual
//#cpp2rust D3DPCMPCAPS_NEVER           = d3d::PCmpCaps::Never
//#cpp2rust D3DPCMPCAPS_NOTEQUAL        = d3d::PCmpCaps::NotEqual
