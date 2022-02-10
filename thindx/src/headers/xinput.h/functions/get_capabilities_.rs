use crate::*;
use crate::xinput::*;

use bytemuck::Zeroable;

use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)\]
/// XInputGetCapabilities
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`Flag`]
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`Flag::None`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`User`] in bounds, but without a gamepad
pub fn get_capabilities(user_index: impl Into<u32>, flags: Flag) -> Result<Capabilities, Error> {
    fn_context!(xinput::get_capabilities => XInputGetCapabilities);
    let mut caps = Capabilities::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  is well tested
    //  * `flags`       is decently tested (0, 1, 2 (OOB), 4, 8, 16, 32, 64, 128, 0xFFFFFFFF)
    //  * `caps`        is out-only, no cbSize field, fixed size, sane
    let code = unsafe { XInputGetCapabilities(user_index.into(), flags.into(), caps.as_mut()) };
    check_success!(code)?;
    Ok(caps)
}

#[test] fn test_valid_params() {
    if let Err(err) = get_capabilities(0u32, Flag::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(1u32, Flag::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(2u32, Flag::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(3u32, Flag::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }

    if let Err(err) = get_capabilities(0u32, Flag::None   ) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(1u32, Flag::None   ) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(2u32, Flag::None   ) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(3u32, Flag::None   ) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_capabilities(User::Any,                 Flag::Gamepad));            // Bad User (any)
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_capabilities(User::from_unchecked(4),   Flag::Gamepad));            // Bad User (obb)
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_capabilities(User::Zero,                Flag::from_unchecked(42))); // Bad Flag (obb)
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_capabilities(User::Zero,                Flag::from_unchecked(!0))); // Bad Flag (obb)
    for u in User::iter_invalid() {
        assert_eq!(ERROR::BAD_ARGUMENTS, get_capabilities(u, Flag::Gamepad)); // Bad user only
        assert_eq!(ERROR::BAD_ARGUMENTS, get_capabilities(u, Flag::from_unchecked(42))); // Bad Flag (obb)
        assert_eq!(ERROR::BAD_ARGUMENTS, get_capabilities(u, Flag::from_unchecked(!0))); // Bad Flag (obb)
    }
}
