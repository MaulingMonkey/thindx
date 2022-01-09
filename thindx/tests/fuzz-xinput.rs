const BEFORE_COM    : Duration = Duration::from_secs(20);
const WITH_COM      : Duration = Duration::from_secs(20);



use thindx::*;

use mmrbi::*;

use winapi::shared::winerror::*;
use winapi::um::objbase::CoInitialize;

use std::ptr::null_mut;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;
use std::time::Duration;



macro_rules! fuzz {
    ( $expr:expr $(, $error:expr)* $(,)? ) => {
        fuzz(stringify!($expr), file!(), line!(), &[$($error),*], $expr)
    }
}

#[test] fn torment() {
    let should_init_com     = Arc::new(AtomicBool::new(false));
    let continue_running    = Arc::new(AtomicBool::new(true));

    let threads = (0 .. 8).map(|_|{
        let should_init_com     = should_init_com.clone();
        let continue_running    = continue_running.clone();
        std::thread::spawn(move || {
            let mut initialized_com = false;

            while !initialized_com || continue_running.load(Relaxed) {
                if !initialized_com && should_init_com.load(Relaxed) {
                    initialized_com = true;
                    initialize_com_for_the_first_time_on_this_thread();
                }

                for i in 0 ..= 100 { xinput::enable(i>>2 & 1 != 0) } // no error code, no xuser

                fuzz!(
                    #[allow(deprecated)] |u| xinput::get_audio_device_ids(u),
                    ERROR::BAD_ARGUMENTS,
                    // ERROR::DEVICE_NOT_CONNECTED, // get_audio_device_ids succeeds even when `u` isn't connected
                    THINERR::MISSING_DLL_EXPORT,    // untested
                );

                fuzz!(
                    #[allow(deprecated)] |u| xinput::get_dsound_audio_device_guids(u),
                    ERROR::BAD_ARGUMENTS,           // untested
                    ERROR::DEVICE_NOT_CONNECTED,    // untested
                    THINERR::MISSING_DLL_EXPORT,
                );

                for bt in [
                    xinput::BatteryDevType::Gamepad,
                    xinput::BatteryDevType::Headset,
                    xinput::BatteryDevType::from_unchecked(2),
                    xinput::BatteryDevType::from_unchecked(42),
                    xinput::BatteryDevType::from_unchecked(255),
                ].iter().copied() {
                    fuzz!(
                        |u| xinput::get_battery_information(u, bt),
                        ERROR::BAD_ARGUMENTS,
                        ERROR::DEVICE_NOT_CONNECTED,
                    );
                }

                for flag in [
                    xinput::Flag::None,
                    xinput::Flag::Gamepad,
                    xinput::Flag::from_unchecked(2),
                    xinput::Flag::from_unchecked(4),
                    xinput::Flag::from_unchecked(8),
                    xinput::Flag::from_unchecked(16),
                    xinput::Flag::from_unchecked(32),
                    xinput::Flag::from_unchecked(64),
                    xinput::Flag::from_unchecked(128),
                    xinput::Flag::from_unchecked(!0),
                ].iter().copied() {
                    fuzz!(
                        |u| xinput::get_capabilities(u, flag),
                        ERROR::BAD_ARGUMENTS,
                        ERROR::DEVICE_NOT_CONNECTED,
                    );
                }

                fuzz!(
                    |u| xinput::get_keystroke(u, ()),
                    ERROR::BAD_ARGUMENTS,
                    ERROR::DEVICE_NOT_CONNECTED,
                );

                fuzz!(
                    |u| xinput::get_state(u),
                    ERROR::BAD_ARGUMENTS,
                    ERROR::DEVICE_NOT_CONNECTED,
                );

                fuzz!(
                    #[allow(deprecated)] |u| xinput::get_state_ex(u),
                    ERROR::BAD_ARGUMENTS,
                    ERROR::DEVICE_NOT_CONNECTED,
                );

                for enable in [false, true].iter().copied() {
                    xinput::enable(enable);
                    for vibration in [
                        xinput::Vibration::default(),
                        xinput::Vibration { left_motor_speed: 1, right_motor_speed: 1 },
                    ].iter().copied() {
                        fuzz!(
                            |u| xinput::set_state(u, vibration),
                            ERROR::BAD_ARGUMENTS,
                            ERROR::DEVICE_NOT_CONNECTED,
                        );
                    }
                }
            }
        })
    });

    status!("Testing", "without COM initialized ({:?})", BEFORE_COM);
    sleep(BEFORE_COM);

    status!("Testing", "with COM initialized ({:?})", WITH_COM);
    should_init_com.store(true, Relaxed);
    sleep(WITH_COM);

    status!("Testing", "teardown");
    continue_running.store(false, Relaxed);
    for thread in threads { thread.join().unwrap() }
}

fn initialize_com_for_the_first_time_on_this_thread() {
    match unsafe { CoInitialize(null_mut()) } {
        S_OK    => {},
        S_FALSE => fatal!("CoInitialize(nullptr) returned S_FALSE: COM was already initialized on this thread!  Tests were trying to ensure xinput is sound to use pre-COM..."),
        err     => fatal!("CoInitialize(nullptr) returned 0x{:08X} (unexpected {})", err, if SUCCEEDED(err) { "success code" } else { "error" }),
    }
}

fn fuzz<T>(title: &str, file: &str, line: u32, codes: &[ErrorKind], f: impl Fn(xinput::User) -> Result<T, MethodError>) {
    for _ in 0 ..= 100 { let _ = f(xinput::User::Zero ); }
    for _ in 0 ..= 100 { let _ = f(xinput::User::One  ); }
    for _ in 0 ..= 100 { let _ = f(xinput::User::Two  ); }
    for _ in 0 ..= 100 { let _ = f(xinput::User::Three); }
    for _ in 0 ..= 100 { let _ = f(xinput::User::Any  ); }
    for _ in 0 ..= 100 { let _ = f(xinput::User::from_unchecked(4)); }
    for _ in 0 ..= 100 { let _ = f(xinput::User::from_unchecked(42)); }
    for _ in 0 ..= 100 { let _ = f(xinput::User::from_unchecked(67)); }
    for _ in 0 ..= 100 { let _ = f(xinput::User::from_unchecked(254)); }

    for u in (0 ..= 255).map(|u| xinput::User::from_unchecked(u)) {
        if let Err(err) = f(u) {
            let kind = err.kind();
            if !codes.contains(&kind) { fatal!(at: file, line: line as _, "when fuzzing {}: unexpected error {:?}", title, kind); }
        }
    }
}
