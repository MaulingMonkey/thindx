use thin3d9::*;
use std::marker::Sized;



pub trait Direct3DExt : Sized {
    /// Create a Device for testing/documentation purpouses.
    ///
    /// Note that the lack of `unsafe means this defeats thin3d9's global soundness - see [Direct3D::create] for details!
    fn test() -> Self;
}

impl Direct3DExt for Direct3D {
    fn test() -> Self {
        unsafe { Direct3D::create(SdkVersion::default()).unwrap() }
    }
}
