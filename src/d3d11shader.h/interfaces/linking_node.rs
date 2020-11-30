use crate::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11linkingnode)\]
/// ID3D11LinkingNode
///
/// A linking-node interface is used for shader linking.
#[derive(Clone)] #[repr(transparent)]
pub struct LinkingNode(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11LinkingNode>);

convert!(unsafe LinkingNode => Unknown, winapi::um::d3d11shader::ID3D11LinkingNode);

// This space intentionally left blank!
// It has no methods beyond the default IUnknown methods.
