//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/dx9-graphics-reference-d3d-functions)\]
//! Direct3D PIX events for annotating graphics debugger traces and captures.

use crate::*;

use abistr::AsCStr;

use winapi::shared::d3d9::*;
use winapi::shared::minwindef::DWORD;

use std::fmt::{self, Debug, Display, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_beginevent)\]
/// D3DPERF_BeginEvent
///
/// Marks the beginning of a user-defined event.
///
/// ### Returns
/// *   Ok(level_of_the_starting_event: [u32])
/// *   Err(...)                                If no PIX debugger is attached?
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # if false {
/// let scope = d3d::perf::begin_event(d3d::Color::argb(0), abistr::cstr16!("example scope"));
/// // ...
/// if scope.is_ok() { drop(d3d::perf::end_event()) }
/// # }
/// ```
pub fn begin_event(col: impl Into<d3d::Color>, name: impl abistr::TryIntoAsCStr<u16>) -> Result<u32, Error> {
    fn_context!(d3d::perf::begin_event => D3DPERF_BeginEvent);
    let col     = col.into().into();
    let name    = fn_param_try_into!(name)?;
    let level   = unsafe { D3DPERF_BeginEvent(col, name.as_cstr()) };
    if level < 0 {
        fn_err!(THINERR::NONSPECIFIC)
    } else {
        Ok(level as _)
    }
}

//#cpp2rust D3DPERF_BeginEvent = d3d::perf::event_scope
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_beginevent)\]
/// D3DPERF_BeginEvent .. D3DPERF_EndEvent scope
///
/// Marks the beginning and end of a user-defined event.
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # if false {
/// let _scope = d3d::perf::event_scope(d3d::Color::argb(0), abistr::cstr16!("example scope"));
/// // ...
/// # }
/// ```
pub fn event_scope(col: impl Into<d3d::Color>, name: impl abistr::TryIntoAsCStr<u16>) -> EventScope {
    begin_event(col, name).map(|s| EventScope { scope: Some(s) }).unwrap_or_default()
}

//#cpp2rust D3DPERF_BeginEvent = d3d::perf::event_scope_if
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_beginevent)\]
/// D3DPERF_BeginEvent .. D3DPERF_EndEvent scope
///
/// Marks the beginning and end of a user-defined event.
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # if false {
/// let _scope = d3d::perf::event_scope_if(
///     cfg!(debug_assertions),
///     d3d::Color::argb(0),
///     abistr::cstr16!("example scope")
/// );
/// // ...
/// # }
/// ```
pub fn event_scope_if(condition: bool, col: impl Into<d3d::Color>, name: impl abistr::TryIntoAsCStr<u16>) -> EventScope {
    if condition { event_scope(col, name) } else { EventScope::default() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_beginevent)\]
/// D3DPERF_BeginEvent .. D3DPERF_EndEvent scope
///
/// Keeps track of if D3DPERF_BeginEvent succeeded, and attempts to pair it with a D3DPERF_EndEvent.
///
/// | Open Method                   | Behavior      |
/// | ----------------------------- | ------------- |
/// | [event_scope]                 | Attempts to open a scope
/// | [event_scope_if]              | Attempts to open a scope if `condition` is set
///
/// | Close Method                  | Behavior      |
/// | ----------------------------- | ------------- |
/// | [EventScope::close_unchecked] | Closes only if opened, errors ignored
/// | [EventScope::close]           | Closes only if opened, errors returned to the user to handle
/// | [EventScope::drop]            | Closes only if opened, errors assert in debug builds
pub struct EventScope { scope: Option<u32> }

impl EventScope {
    //#cpp2rust D3DPERF_EndEvent = d3d::perf::EventScope::close_unchecked
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_endevent)\]
    /// D3DPERF_EndEvent
    ///
    /// Marks the end of a user-defined event, if this event scope was open to begin with.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # if false {
    /// let scope = d3d::perf::event_scope(d3d::Color::argb(0), abistr::cstr16!("example scope"));
    /// // ...
    /// scope.close_unchecked();
    /// # }
    /// ```
    pub fn close_unchecked(mut self) { drop(self.close_impl()) }

    //#cpp2rust D3DPERF_EndEvent = d3d::perf::EventScope::close
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_endevent)\]
    /// D3DPERF_EndEvent
    ///
    /// Marks the end of a user-defined event, if this event scope was open to begin with.
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*;
    /// # if false {
    /// let scope = d3d::perf::event_scope(d3d::Color::argb(0), abistr::cstr16!("example scope"));
    /// // ...
    /// scope.close();
    /// # }
    /// ```
    pub fn close(mut self) -> Result<(), EventMismatchError> { self.close_impl() }

    fn close_impl(&mut self) -> Result<(), EventMismatchError> {
        if let Some(expected) = self.scope.take() {
            let actual = end_event().map_err(|_| EventMismatchError { expected, actual: expected })?;
            if expected == actual { Ok(()) } else { Err(EventMismatchError { expected, actual }) }
        } else {
            Ok(())
        }
    }
}

impl Default for EventScope {
    fn default() -> Self { Self { scope: None } }
}

impl Drop for EventScope {
    //#cpp2rust D3DPERF_EndEvent = d3d::perf::EventScope::drop
    fn drop(&mut self) {
        if let Err(err) = self.close_impl() {
            assert!(!cfg!(debug_assertions), "{}", err);
        }
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_endevent)\]
/// D3DPERF_EndEvent
///
/// Marks the end of a user-defined event.
///
/// ### Returns
/// *   Ok(level_of_the_ending_event: [u32])
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # if false {
/// let scope = d3d::perf::begin_event(d3d::Color::argb(0), abistr::cstr16!("example scope"));
/// // ...
/// if scope.is_ok() { drop(d3d::perf::end_event()) }
/// # }
/// ```
pub fn end_event() -> Result<u32, Error> {
    fn_context!(d3d::perf::end_event => D3DPERF_EndEvent);
    let level   = unsafe { D3DPERF_EndEvent() };
    if level < 0 {
        fn_err!(THINERR::NONSPECIFIC)
    } else {
        Ok(level as _)
    }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_getstatus)\]
/// D3DPERF_GetStatus
///
/// Determine the current state of the profiler from the target program.
///
/// ### Returns
/// *   `0`    If PIX is **not** profiling the target program.
/// *   `?`    If PIX is profiling the target program
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// let is_pix_attached = d3d::perf::get_status() != 0;
/// dbg!(is_pix_attached); // false
/// ```
pub fn get_status() -> u32 {
    fn_context!(d3d::perf::get_status => D3DPERF_GetStatus);
    unsafe { D3DPERF_GetStatus() }
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_queryrepeatframe)\]
/// D3DPERF_QueryRepeatFrame
///
/// Determine whether a performance profiler is requesting that the current frame be resubmitted to Direct3D for performance analysis.
///
/// **NOTE:** May not be supported / may always return `false`.
///
/// ### Returns
/// *   `true`      If the caller should repeat the same sequence of calls.
/// *   `false`     If the caller should continue.
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// let should_rerender_frame = d3d::perf::query_repeat_frame();
/// dbg!(should_rerender_frame); // false
/// ```
pub fn query_repeat_frame() -> bool {
    fn_context!(d3d::perf::query_repeat_frame => D3DPERF_QueryRepeatFrame);
    let r = unsafe { D3DPERF_QueryRepeatFrame() };
    r != 0
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_setmarker)\]
/// D3DPERF_SetMarker
///
/// Mark an instantaneous event.
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # if false {
/// let scope = d3d::perf::begin_event(d3d::Color::argb(0), abistr::cstr16!("example scope"));
/// // ...
/// if scope.is_ok() { drop(d3d::perf::set_marker(d3d::Color::argb(0), abistr::cstr16!("example scope"))) }
/// // ...
/// if scope.is_ok() { drop(d3d::perf::end_event()) }
/// # }
/// ```
pub fn set_marker(col: impl Into<d3d::Color>, name: impl abistr::TryIntoAsCStr<u16>) -> Result<(), Error> {
    fn_context!(d3d::perf::set_marker => D3DPERF_SetMarker);
    let col     = col.into().into();
    let name    = fn_param_try_into!(name)?;
    unsafe { D3DPERF_SetMarker(col, name.as_cstr()) };
    Ok(())
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_setoptions)\]
/// D3DPERF_SetOptions
///
/// Set profiler options specified by the target program.
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// d3d::perf::set_options(d3d::perf::Options::ForbidPix);
/// d3d::perf::set_options(d3d::perf::Options::None);
/// # for pow in 0 .. 32 {
/// #   d3d::perf::set_options(d3d::perf::Options::from_unchecked(1 << pow));
/// # }
/// # d3d::perf::set_options(d3d::perf::Options::None);
/// ```
pub fn set_options(options: impl Into<Options>) {
    fn_context!(d3d::perf::set_options => D3DPERF_SetOptions);
    // MSDN's "Syntax" sections misdocuments this as having an `int` return.
    // The "Return value" section states it "doesn't return a value".
    // The function returns `void` in the headers, so we trust the latter.
    let options = options.into().into();
    unsafe { D3DPERF_SetOptions(options) };
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_setoptions#parameters)\]
/// D3DPERF_SetOptions's dwOptions
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Options(DWORD);
flags! { Options => DWORD; None, ForbidPix }
#[allow(non_upper_case_globals)] impl Options {
    /// No flags
    pub const None : Options = Options(0);

    /// Notify PIX that the target program does not give permission to be profiled.
    pub const ForbidPix : Options = Options(1);
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_setregion)\]
/// D3DPERF_SetRegion
///
/// Mark a series of frames with the specified color and name.
///
/// **NOTE:** May not be supported (or may overwrite the results of an earlier [d3d::perf::begin_event]?)
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # if false {
/// let scope = d3d::perf::begin_event(d3d::Color::argb(0), abistr::cstr16!("example scope"));
/// // ...
/// if scope.is_ok() { drop(d3d::perf::set_region(d3d::Color::argb(0), abistr::cstr16!("example scope"))) }
/// // ...
/// if scope.is_ok() { drop(d3d::perf::end_event()) }
/// # }
/// ```
pub fn set_region(col: impl Into<d3d::Color>, name: impl abistr::TryIntoAsCStr<u16>) -> Result<(), Error> {
    fn_context!(d3d::perf::set_region => D3DPERF_SetRegion);
    let col     = col.into().into();
    let name    = fn_param_try_into!(name)?;
    unsafe { D3DPERF_SetRegion(col, name.as_cstr()) };
    Ok(())
}



/// [D3DPERF_BeginEvent] .. [D3DPERF_EndEvent] were incorrectly matched
///
/// [D3DPERF_BeginEvent]:   https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_beginevent
/// [D3DPERF_EndEvent]:     https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3d9/nf-d3d9-d3dperf_endevent
pub struct EventMismatchError {
    expected:   u32,
    actual:     u32, // or == expected if none was present
}

impl std::error::Error for EventMismatchError {
    fn description(&self) -> &str { "EventMismatchError: D3DPERF_BeginEvent .. D3DPERF_EndEvent were incorrectly paired" }
}

impl Debug for EventMismatchError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let Self { expected, actual } = *self;
        if expected == actual {
            write!(fmt, "d3d::perf::EventMismatchError {{ expected: {expected}, actual: N/A }}")
        } else {
            write!(fmt, "d3d::perf::EventMismatchError {{ expected: {expected}, actual: {actual} }}")
        }
    }
}

impl Display for EventMismatchError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let Self { expected, actual } = *self;
        if expected == actual {
            write!(fmt, "d3d::perf::EventMismatchError: D3DPERF_EndEvent failed (unclosed D3DPERF_BeginEvent)")
        } else {
            write!(fmt, "d3d::perf::EventMismatchError: D3DPERF_BeginEvent opened scope {expected}, but D3DPERF_EndEvent closed scope {actual}")
        }
    }
}



#[test] fn test_scope_spam() {
    println!("testing unbalanced ends");
    for _ in 0 .. 1000 { drop(end_event()) }

    println!("testing unscoped markers");
    for _ in 0 .. 1000 { drop(set_marker(d3d::Color::argb(0), abistr::cstr16!("testing unscoped marker 123"))) }

    println!("testing unscoped regions");
    for _ in 0 .. 1000 { drop(set_region(d3d::Color::argb(0), abistr::cstr16!("testing unscoped region 123"))) }

    println!("testing begin");
    for _ in 0 .. 1000 { drop(begin_event(d3d::Color::argb(0), abistr::cstr16!("testing event 123"))) }

    println!("testing scoped marker");
    for _ in 0 .. 1000 { drop(set_marker(d3d::Color::argb(0), abistr::cstr16!("testing marker 123"))) }

    println!("testing scoped region");
    for _ in 0 .. 1000 { drop(set_region(d3d::Color::argb(0), abistr::cstr16!("testing region 123"))) }

    println!("testing end");
    for _ in 0 .. 1000 { drop(end_event()) }

    let mut scopes = Vec::new();

    println!("testing scope spam");
    for _ in 0 .. 1000 { scopes.push(event_scope(d3d::Color::argb(0), abistr::cstr16!("testing event scope"))) }
    while !scopes.is_empty() { scopes.pop(); } // default drop order not guaranteed

    println!("testing conditional scope spam");
    for i in 0 .. 1000 { scopes.push(event_scope_if(i%2==0, d3d::Color::argb(0), abistr::cstr16!("testing event scope"))) }
    while !scopes.is_empty() { scopes.pop(); } // default drop order not guaranteed
}
