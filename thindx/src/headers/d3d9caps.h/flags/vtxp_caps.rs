use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvtxpcaps)\]
/// D3DVTXPCAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct VtxPCaps(DWORD);

flags! {
    VtxPCaps => DWORD;
    None, DirectionalLights, LocalViewer, MaterialSource7, NoTexGenNonLocalViewer, PositionalLights, TexGen, TexGenSphereMap, Tweening,
}

#[allow(non_upper_case_globals)] impl VtxPCaps {
    pub const None                          : VtxPCaps = VtxPCaps(0);
    pub const DirectionalLights             : VtxPCaps = VtxPCaps(D3DVTXPCAPS_DIRECTIONALLIGHTS);
    pub const LocalViewer                   : VtxPCaps = VtxPCaps(D3DVTXPCAPS_LOCALVIEWER);
    pub const MaterialSource7               : VtxPCaps = VtxPCaps(D3DVTXPCAPS_MATERIALSOURCE7);
    pub const NoTexGenNonLocalViewer        : VtxPCaps = VtxPCaps(D3DVTXPCAPS_NO_TEXGEN_NONLOCALVIEWER);
    pub const PositionalLights              : VtxPCaps = VtxPCaps(D3DVTXPCAPS_POSITIONALLIGHTS);
    pub const TexGen                        : VtxPCaps = VtxPCaps(D3DVTXPCAPS_TEXGEN);
    pub const TexGenSphereMap               : VtxPCaps = VtxPCaps(D3DVTXPCAPS_TEXGEN_SPHEREMAP);
    pub const Tweening                      : VtxPCaps = VtxPCaps(D3DVTXPCAPS_TWEENING);
}
