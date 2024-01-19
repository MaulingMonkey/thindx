#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::minwindef::*;
const D3DCURSOR_IMMEDIATE_UPDATE : u32 = 1; // not defined in winapi



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcursorposition)\]
/// DWORD / D3DCURSOR_\*
///
/// Controls how [IDirect3DQuery9Ext::set_cursor_position] behaves.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Cursor(DWORD);

flags! { Cursor => DWORD; None, ImmediateUpdate }

#[allow(non_upper_case_globals)] impl Cursor { // These are enum-like
    /// No flags
    pub const None : Cursor = Cursor(0);

    /// [D3DCURSOR_IMMEDIATE_UPDATE](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcursorposition)
    pub const ImmediateUpdate : Cursor = Cursor(D3DCURSOR_IMMEDIATE_UPDATE);
}

//#cpp2rust D3DCURSOR_IMMEDIATE_UPDATE = d3d::Cursor::ImmediateUpdate
