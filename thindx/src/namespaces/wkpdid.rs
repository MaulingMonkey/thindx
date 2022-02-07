//! **W**ell **K**nown **P**rivate **D**ata **ID**entifier s
//!
//! See Also
//! *   [d3d9::IDirect3DResource9Ext::set_private_data]
//! *   [d3d9::IDirect3DVolume9Ext::set_private_data]
//! *   [IDirect3DResource9::SetPrivateData](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dresource9-setprivatedata)
//! *   [IDirect3DVolume9::SetPrivateData](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dvolume9-setprivatedata)
//! *   [ID3D11Device::SetPrivateData](https://docs.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11device-setprivatedata)
//! *   [d3dcommon.h:981-983](https://github.com/apitrace/dxsdk/blob/master/Include/d3dcommon.h#L981-L983)

#![allow(non_upper_case_globals)]

use crate::*;



// https://github.com/apitrace/dxsdk/blob/d964b66467aaa734edbc24326da8119f5f063dd3/Include/d3dcommon.h#L981-L983

/// A debug object name, displayed in some graphics debuggers.
///
/// The name is encoded in some narrow charset, interpreted by multiple applications, possibly on multiple computers if e.g. debug captures were transfered between systems.
/// As such, any sane name should *probably* stick to a ASCII-safe subset, or use [wkpdid::D3DDebugObjectNameW] if supported instead.
pub const D3DDebugObjectName     : Guid = guid!("429b8c22-9188-4b0c-8742-acb0bf85c200");

/// A UTF16ish debug object name, displayed in some graphics debuggers.
pub const D3DDebugObjectNameW    : Guid = guid!("4cca5fd8-921f-42c8-8566-70caf2a9b741");

/// A UTF16ish comment string.
///
/// >   I don't know what this is actually used for?<br>
/// >   &ndash; MaulingMonkey
pub const CommentStringW         : Guid = guid!("d0149dc0-90e8-4ec8-8144-e900ad266bb2");



//#cpp2rust WKPDID_D3DDebugObjectName   = wkpdid::D3DDebugObjectName
//#cpp2rust WKPDID_D3DDebugObjectNameW  = wkpdid::D3DDebugObjectNameW
//#cpp2rust WKPDID_CommentStringW       = wkpdid::CommentStringW
