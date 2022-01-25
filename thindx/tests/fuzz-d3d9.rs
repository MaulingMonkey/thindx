#![cfg(feature = "d3dx9")]

use thindx::*;



#[test] fn basic_calls() {
    let device : d3d9::Device = dev::d3d9::device_pure();
    let _err = unsafe { d3dx9::check_cube_texture_requirements(&device, None, None, d3d::Usage::None, None, d3d::Pool::Managed) };
}
