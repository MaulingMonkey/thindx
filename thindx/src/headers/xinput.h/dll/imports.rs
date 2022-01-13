use abistr::*;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::*;
use winapi::um::processthreadsapi::*;
use winapi::um::psapi::*;
use winapi::um::winnt::*;
use winapi::um::xinput::*;

use std::convert::*;
use std::ffi::c_void;
use std::io;
use std::mem::*;
use std::ptr::null_mut;



lazy_static::lazy_static! {
    static ref IMPORTS : Imports = Imports::load_from_linked_xinput().unwrap_or_default();
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
        // SAFETY: ⚠️ Technically unsound
        //  * We assume `hmodule`, if retrieved, is a valid xinput DLL.
        //  * We assume specific magic ordinals always map to specific un-named functions.
        unsafe {
            let hmodule = try_find_loaded_xinput().ok_or(io::ErrorKind::NotFound)?;

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

/// Tries to find the most XInput-y looking, already loaded, DLL:
/// *   DLLs that don't export `XInputGetState` will be straight up ignored.
/// *   DLLs with more characters matching the `xinput_` prefix will be prefered.
/// *   DLLs with shorter filenames will be prefered (e.g. `XInput1_4.dll` wins out over `xinput_9_0_3.dll`)
///
/// ### ⚠️ Safety ⚠️
///
/// Microsoft's PSAPI documentation makes it clear that some of the stuff this relies on for e.g. process module enumeration
/// are best effort debug functionality, not battle tested production quality tooling:
///
/// > "The EnumProcessModulesEx function is primarily designed for use by debuggers and similar applications that must
/// > extract module information from another process. If the module list in the target process is corrupted or not
/// > yet initialized, or if the module list changes during the function call as a result of DLLs being loaded or
/// > unloaded, EnumProcessModulesEx may fail or return incorrect information."
/// >
/// > <https://docs.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodulesex>
///
/// Additionally, there's technically nothing stopping you from loading an evil `xinput_.dll`, that takes priority over
/// the real xinput DLLs, that defines XInputGetState and exposes unsound functions, which immediately invokes undefined
/// behavior if you call into it.  Which means relying on the returned HMODULE to do just about anything is, *technically*,
/// unsound.
///
/// Well, perhaps eventually I'll verify xinput.dll is code-signed by Microsoft, which would fix that well enough for my
/// tastes, but for now this is good enough for me ;)
unsafe fn try_find_loaded_xinput() -> Option<HMODULE> {
    let proc = GetCurrentProcess();
    let mut modules = Vec::<HMODULE>::new();

    let mut max_retries = 64;

    loop {
        let available_bytes = u32::try_from(std::mem::size_of_val(&modules[..])).unwrap_or(!0);
        let mut needed_bytes : u32 = 0;
        let ok = EnumProcessModulesEx(proc, modules.as_mut_ptr(), available_bytes, &mut needed_bytes, LIST_MODULES_DEFAULT);
        if ok == FALSE {
            if max_retries == 0 { return None; }
            max_retries -= 1;
            continue; // temporary failure? retry!
        }
        let needed_elements = usize::try_from(needed_bytes).unwrap_or(!0usize) / size_of::<HMODULE>();
        if needed_bytes <= available_bytes {
            modules.shrink_to(needed_elements);
            break // success!
        } else {
            modules.resize(needed_elements, null_mut());
            continue // not enough modules
        }
    }

    modules.retain(|&m| load_proc_by_name(m, cstr!("XInputGetState")).is_some());

    match modules[..] {
        [] => None,
        [module] => Some(module),
        ref mut multiple => {
            let mut name = [0u8; 4096];
            multiple.sort_by_cached_key(|&m|{
                let len = GetModuleBaseNameA(proc, m, name.as_mut_ptr().cast(), name.len() as _) as usize;
                let name = &mut name[..len];
                name.make_ascii_lowercase();
                let prefix = b"xinput_";
                let matching = prefix.iter().copied().zip(name.iter().copied()).position(|(x,y)| x != y).unwrap_or(prefix.len());
                (matching * 1000 + 1000).saturating_sub(name.len()) // prioritize prefix matching "xinput_", then secondarilly prioritize shorter names.
            });
            multiple.last().copied()
        },
    }
}
