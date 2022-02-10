use crate::*;
use crate::ctypes::*;

use winapi::shared::winerror::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nn-d3d11shader-id3d11moduleinstance)\]
/// ID3D11ModuleInstance
///
/// A module-instance interface is used for resource rebinding.
#[derive(Clone)] #[repr(transparent)]
pub struct ModuleInstance(pub(crate) mcom::Rc<winapi::um::d3d11shader::ID3D11ModuleInstance>);

convert!(unsafe ModuleInstance => Unknown, winapi::um::d3d11shader::ID3D11ModuleInstance);

macro_rules! check { ( $hr:expr ) => {{
    let hr = $hr;
    if hr == S_FALSE {
        Ok(false)
    } else {
        fn_check_hr!(hr)?;
        Ok(true)
    }
}}}

// TODO: traitify

impl ModuleInstance {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindconstantbuffer)\]
    /// ID3D11ModuleInstance::BindConstantBuffer
    ///
    /// Rebinds a constant buffer from a source slot to a destination slot.
    ///
    /// ### Arguments
    /// *   `src_slot`      - The source slot number for rebinding.
    /// *   `dst_slot`      - The destination slot number for rebinding.
    /// *   `dst_offset`    - The offset in bytes of the destination slot for rebinding. The offset must have 16-byte alignment.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - `dst_slot` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   Ok(`true`)      - for a valid rebinding
    /// *   Ok(`false`)     - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_constant_buffer(&self, src_slot: u32, dst_slot: u32, dst_offset: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_constant_buffer => ID3D11ModuleInstance::BindConstantBuffer);
        let hr = unsafe { self.0.BindConstantBuffer(src_slot, dst_slot, dst_offset) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindconstantbufferbyname)\]
    /// ID3D11ModuleInstance::BindConstantBufferByName
    ///
    /// Rebinds a constant buffer by name to a destination slot.
    ///
    /// ### Arguments
    /// *   `name`          - The name of the constant buffer for rebinding.
    /// *   `dst_slot`      - The destination slot number for rebinding.
    /// *   `dst_offset`    - The offset in bytes of the destination slot for rebinding. The offset must have 16-byte alignment.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - `dst_slot` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_constant_buffer_by_name(&self, name: impl TryIntoAsCStr, dst_slot: u32, dst_offset: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_constant_buffer_by_name => ID3D11ModuleInstance::BindConstantBufferByName);
        let name = name.try_into().map_err(|e| fn_param_error!(name, e.into()))?;
        let hr = unsafe { self.0.BindConstantBufferByName(name.as_cstr(), dst_slot, dst_offset) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindresource)\]
    /// ID3D11ModuleInstance::BindResource
    ///
    /// Rebinds a texture or buffer from source slot to destination slot.
    ///
    /// ### Arguments
    /// *   `src_slot`      - The first source slot number for rebinding.
    /// *   `dst_slot`      - The first destination slot number for rebinding.
    /// *   `count`         - The number of slots for rebinding.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - A slot between `dst_slot .. dst_slot + count` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_resource(&self, src_slot: u32, dst_slot: u32, count: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_resource => ID3D11ModuleInstance::BindResource);
        let hr = unsafe { self.0.BindResource(src_slot, dst_slot, count) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindresourceasunorderedaccessview)\]
    /// ID3D11ModuleInstance::BindResourceAsUnorderedAccessView
    ///
    /// Rebinds a resource as an unordered access view (UAV) from source slot to destination slot.
    ///
    /// ### Arguments
    /// *   `src_srv_slot`  - The first source slot number for rebinding.
    /// *   `dst_uav_slot`  - The first destination slot number for rebinding.
    /// *   `count`         - The number of slots for rebinding.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - A slot between `dst_uav_slot .. dst_uav_slot + count` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_resource_as_unordered_access_view(&self, src_srv_slot: u32, dst_uav_slot: u32, count: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_resource_as_unordered_access_view => ID3D11ModuleInstance::BindResourceAsUnorderedAccessView);
        let hr = unsafe { self.0.BindResourceAsUnorderedAccessView(src_srv_slot, dst_uav_slot, count) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindresourceasunorderedaccessviewbyname)\]
    /// ID3D11ModuleInstance::BindResourceAsUnorderedAccessViewByName
    ///
    /// Rebinds a resource by name as an unordered access view (UAV) to destination slots.
    ///
    /// ### Arguments
    /// *   `srv_name`      - The name of the resource for rebinding.
    /// *   `dst_uav_slot`  - The first destination slot number for rebinding.
    /// *   `count`         - The number of slots for rebinding.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - A slot between `dst_uav_slot .. dst_uav_slot + count` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_resource_as_unordered_access_view_by_name(&self, srv_name: impl TryIntoAsCStr, dst_uav_slot: u32, count: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_resource_as_unordered_access_view_by_name => ID3D11ModuleInstance::BindResourceAsUnorderedAccessViewByName);
        let srv_name = srv_name.try_into().map_err(|e| fn_param_error!(srv_name, e.into()))?;
        let hr = unsafe { self.0.BindResourceAsUnorderedAccessViewByName(srv_name.as_cstr(), dst_uav_slot, count) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindresourcebyname)\]
    /// ID3D11ModuleInstance::BindResourceByName
    ///
    /// Rebinds a texture or buffer by name to destination slots.
    ///
    /// ### Arguments
    /// *   `name`          - The name of the texture or buffer for rebinding.
    /// *   `dst_slot`      - The first destination slot number for rebinding.
    /// *   `count`         - The number of slots for rebinding.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - A slot between `dst_slot .. dst_slot + count` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_resource_by_name(&self, name: impl TryIntoAsCStr, dst_slot: u32, count: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_resource_by_name => ID3D11ModuleInstance::BindResourceByName);
        let name = name.try_into().map_err(|e| fn_param_error!(name, e.into()))?;
        let hr = unsafe { self.0.BindResourceByName(name.as_cstr(), dst_slot, count) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindsampler)\]
    /// ID3D11ModuleInstance::BindSampler
    ///
    /// Rebinds a sampler from source slot to destination slot.
    ///
    /// ### Arguments
    /// *   `src_slot`      - The first source slot number for rebinding.
    /// *   `dst_slot`      - The first destination slot number for rebinding.
    /// *   `count`         - The number of slots for rebinding.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - A slot between `dst_slot .. dst_slot + count` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_sampler(&self, src_slot: u32, dst_slot: u32, count: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_sampler => ID3D11ModuleInstance::BindSampler);
        let hr = unsafe { self.0.BindSampler(src_slot, dst_slot, count) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindsamplerbyname)\]
    /// ID3D11ModuleInstance::BindSamplerByName
    ///
    /// Rebinds a sampler by name to destination slots.
    ///
    /// ### Arguments
    /// *   `name`          - The name of the sampler for rebinding.
    /// *   `dst_slot`      - The first destination slot number for rebinding.
    /// *   `count`         - The number of slots for rebinding.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - A slot between `dst_slot .. dst_slot + count` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_sampler_by_name(&self, name: impl TryIntoAsCStr, dst_slot: u32, count: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_sampler_by_name => ID3D11ModuleInstance::BindSamplerByName);
        let name = name.try_into().map_err(|e| fn_param_error!(name, e.into()))?;
        let hr = unsafe { self.0.BindSamplerByName(name.as_cstr(), dst_slot, count) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindunorderedaccessview)\]
    /// ID3D11ModuleInstance::BindUnorderedAccessView
    ///
    /// Rebinds an unordered access view (UAV) from source slot to destination slot.
    ///
    /// ### Arguments
    /// *   `src_slot`      - The first source slot number for rebinding.
    /// *   `dst_slot`      - The first destination slot number for rebinding.
    /// *   `count`         - The number of slots for rebinding.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - A slot between `dst_slot .. dst_slot + count` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_unordered_access_view(&self, src_slot: u32, dst_slot: u32, count: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_unordered_access_view => ID3D11ModuleInstance::BindUnorderedAccessView);
        let hr = unsafe { self.0.BindUnorderedAccessView(src_slot, dst_slot, count) };
        check!(hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11moduleinstance-bindunorderedaccessviewbyname)\]
    /// ID3D11ModuleInstance::BindUnorderedAccessViewByName
    ///
    /// Rebinds an unordered access view (UAV) by name to destination slots.
    ///
    /// ### Arguments
    /// *   `name`          - The name of the UAV for rebinding.
    /// *   `dst_slot`      - The first destination slot number for rebinding.
    /// *   `count`         - The number of slots for rebinding.
    ///
    /// ### Errors
    /// *   [E::FAIL]       - the rebinding is out-of-bounds
    /// *   [E::FAIL]       - A slot between `dst_slot .. dst_slot + count` was already bound?
    /// *   [E::FAIL]       - an otherwise invalid rebinding
    ///
    /// ### Returns
    /// *   `true`          - for a valid rebinding
    /// *   `false`         - for rebinding a nonexistent slot; that is, for which the shader reflection doesn’t have any data
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// // TODO
    /// ```
    pub fn bind_unordered_access_view_by_name(&self, name: impl TryIntoAsCStr, dst_slot: u32, count: u32) -> Result<bool, MethodError> {
        fn_context!(d3d11::ModuleInstance::bind_unordered_access_view_by_name => ID3D11ModuleInstance::BindUnorderedAccessViewByName);
        let name = name.try_into().map_err(|e| fn_param_error!(name, e.into()))?;
        let hr = unsafe { self.0.BindUnorderedAccessViewByName(name.as_cstr(), dst_slot, count) };
        check!(hr)
    }
}

//#cpp2rust ID3D11ModuleInstance                                            = d3d11::ModuleInstance
