use abistr::*;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::*;
use winapi::um::winnt::*;
use winapi::um::xinput::XINPUT_STATE;
use winapi::um::xinput::XInputGetState;

use std::ffi::c_void;
use std::io;
use std::mem::transmute;
use std::ptr::null_mut;



lazy_static::lazy_static! {
    static ref IMPORTS : Imports = Imports::load_from_linked_xinput().unwrap_or(Default::default());
}

#[allow(non_snake_case)]
#[derive(Default)]
pub(crate) struct Imports {
    // Official Imports

    /// \[[MSDN](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids)]
    /// Get DirectSound Audio Device GUIDs (N/A for Windows Store apps).
    ///
    /// | XInput | State    |
    /// | ------ | -------- |
    /// | 1.4   | N/A       |
    /// | 1.3   | Available |
    /// | 9.1.0 | Available |
    pub XInputGetDSoundAudioDeviceGuids: Option<unsafe extern "system" fn(dwUserIndex: DWORD, pDSoundRenderGuid: *mut GUID, pDSoundCaptureGuid: *mut GUID) -> DWORD>,

    /// \[[MSDN](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids)]
    /// Get XAudio2 Device Names.
    ///
    /// | XInput | State    |
    /// | ------ | -------- |
    /// | 1.4   | Available |
    /// | 1.3   | N/A       |
    /// | 9.1.0 | N/A       |
    pub XInputGetAudioDeviceIds: Option<unsafe extern "system" fn(dwUserIndex: DWORD, pRenderDeviceId: LPWSTR, pRenderCount: *mut UINT, pCaptureDeviceId: LPWSTR, pCaptureCount: *mut UINT) -> DWORD>,



    // Super Secret Shady Ordinal-Only Imports
    // https://gist.github.com/robindegen/9446175
    // https://www.reddit.com/r/ReverseEngineering/comments/148faa/xbox_360_controller_on_windows/c7ayith/

    // Ordinals 100-103, available as of XInput1_3.dll
    pub _XInputGetStateEx:               Option<unsafe extern "system" fn(dwUserIndex: DWORD, pState: *mut XINPUT_STATE) -> DWORD>,
    pub _XInputWaitForGuideButton:       Option<unsafe extern "system" fn(dwUserIndex: DWORD, dwFlag: DWORD, pUnknown: *mut c_void) -> DWORD>,
    pub _XInputCancelGuideButtonWait:    Option<unsafe extern "system" fn(dwUserIndex: DWORD) -> DWORD>,
    pub _XInputPowerOffController:       Option<unsafe extern "system" fn(dwUserIndex: DWORD) -> DWORD>,

    // Ordinals 104 / 108, available as of XInput1_4.dll
    // _XInputGetBaseBusInformation (Ordinal 104)
    // _XInputGetCapabilitiesEx     (Ordinal 108)
}

impl Imports {
    pub fn get() -> &'static Self { &*IMPORTS }

    fn load_from_linked_xinput() -> Result<Self, io::Error> {
        unsafe {
            // By design, we'll never FreeLibrary.  Even if we guard all calls into XInput with a mutex, that won't
            // prevent any threads XInput has started from being in the middle of executing XInput code.  And after
            // calling into XInput, there's at least two new threads - one from InputHost.dll, one from ntdll.dll.
            let mut hmodule = null_mut();
            let succeeded = GetModuleHandleExW(
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_PIN,
                (XInputGetState as unsafe extern "system" fn(_, _) -> _) as *mut _,
                &mut hmodule,
            );
            if succeeded == 0 { return Err(io::Error::last_os_error()) }
            if hmodule.is_null() { return Err(io::Error::last_os_error()); }

            Ok(Self {
                XInputGetDSoundAudioDeviceGuids:    load_proc_by_name(hmodule, cstr!("XInputGetDSoundAudioDeviceGuids") ).map(|p| transmute(p)),
                XInputGetAudioDeviceIds:            load_proc_by_name(hmodule, cstr!("XInputGetAudioDeviceIds")         ).map(|p| transmute(p)),

                _XInputGetStateEx:                  load_proc_by_ordinal(hmodule, 100).map(|p| transmute(p)),
                _XInputWaitForGuideButton:          load_proc_by_ordinal(hmodule, 101).map(|p| transmute(p)),
                _XInputCancelGuideButtonWait:       load_proc_by_ordinal(hmodule, 102).map(|p| transmute(p)),
                _XInputPowerOffController:          load_proc_by_ordinal(hmodule, 103).map(|p| transmute(p)),

                // I don't have type information for these... yet
                // _XInputGetBaseBusInformation:    load_proc_by_ordinal(hmodule, 104).map(|p| transmute(p)),
                // _XInputGetCapabilitiesEx:        load_proc_by_ordinal(hmodule, 108).map(|p| transmute(p)),
            })
        }
    }
}



/// ### ⚠️ Safety ⚠️
///
/// *   `hmodule` is assumed to be valid HMODULE.
/// *   The resulting FARPROC must be safely.
unsafe fn load_proc_by_name(hmodule: HMODULE, name: CStrNonNull) -> Option<FARPROC> {
    let a = GetProcAddress(hmodule, name.as_ptr());
    if a.is_null() { None } else { Some(a) }
}

/// ### ⚠️ Safety ⚠️
///
/// *   `hmodule` is assumed to be a valid HMODULE.
/// *   New DLLs often change the ordinals of their functions.  This is super sketchy!
/// *   The resulting FARPROC must be safely.
unsafe fn load_proc_by_ordinal(hmodule: HMODULE, ordinal: WORD) -> Option<FARPROC> {
    let a = GetProcAddress(hmodule, ordinal as usize as *const _);
    if a.is_null() { None } else { Some(a) }
}
