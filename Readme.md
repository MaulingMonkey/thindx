# thindx - **Thin** **D**irect**X** wrappers

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/thindx.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/thindx)
[![Build Status](https://github.com/MaulingMonkey/thindx/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/thindx/actions?query=workflow%3Arust)
[![dependency status](https://deps.rs/repo/github/MaulingMonkey/thindx/status.svg)](https://deps.rs/repo/github/MaulingMonkey/thindx)

[![crates.io](https://img.shields.io/crates/v/thindx.svg)](https://crates.io/crates/thindx)
[![docs.rs](https://docs.rs/thindx/badge.svg)](https://docs.rs/thindx)
[![License](https://img.shields.io/crates/l/thindx.svg)](https://github.com/MaulingMonkey/thindx)

Thin DirectX wrappers for Rust.

### Why not [winapi] directly?

*   This crate aims to make fns safe/sound/slightly rusty when possible
*   Attempts to verify API soundness through mass unit tests, even if they mostly test DirectX's behavior.
*   Doc comments for one-stop intellisense, safety documentation, etc.

### Why not `<other graphics crate>`?

*   Most other graphics crates focus on **hiding** the underlying graphics API as much as possible, improving application portability.
*   This crate **surfaces** the underlying graphics API as much as possible, improving the potential for interoperability with other graphics code / use in middleware applications.

### ❌ This crate is probably unsound! ❌

I'm exposing a huge legacy C++ API to Rust.  Mistakes will happen.

That said, soundness *is* a **very high priority** goal.  `thindx` will add things like extra bounds checks, parameter
validation, extra init, etc. if need be in order to ensure soundness in safe fns whenever possible.  When it's not
possible to validate unsoundness away, the fns in question should be marked `unsafe`.  This crate strives to be sounder
than whatever manual FFI you'd write yourself would be, and that's a **high** bar.

But there are some practical limits to this.  If a background driver thread invokes UB if it fails to allocate memory,
without any direct correlation to specific API misuse like a large integer overflowing, that's a bug I can't sanely
mitigate via safe fns.  I mean, theoretically I could write a pure-software clone of the entire DirectX runtime... but no.

Additionally, while I'm seeking to validate my APIs via testing, older implementations of the APIs in question may have
more bugs / unchecked parameters / ??? that I'll fail to mitigate due to being unable to trigger them myself.  While I'm
happy to investigate, accept pull requests, expand test coverage, etc. it's worth assuming this crate is unsound on
older versions unless you've tested yourself.

### ⚠️ API major version churn ⚠️

Individual `fn`s are likely to gain/lose `unsafe`, traits, etc. in a neverending attempt to make DirectX access sound.
As such, `thindx` itself will likely always suffer from major version churn.
This isn't too much of a problem until two crates wish to share / pass `thindx` types between themselves.
It might be possible to somewhat stabilize some types by exiling them into subcrates, but this has not yet been done.
Additionally, individual extension traits / functions / methods will likely never get the same treatment (no need?)



### Project Status

[examples](https://docs.rs/thindx/*/thindx/_examples/)
•
[header coverage](https://docs.rs/thindx/*/thindx/_headers/)
•
[docs.rs](https://docs.rs/thindx)
•
[lib.rs](https://lib.rs/crates/thindx)
•
[crates.io](https://crates.io/crates/thindx)

| DirectX           | API   | Tests | Sound | Stable| Notes |
| ----------------- | ----- | ----- | ----- | ----- | ----- |
| `d3dcompiler.dll` | ✔️    | ⚠️    | ⚠️    | ❌    | first goal to stabilize
| `dxcompiler.dll`  | ❌    | ❌    | ❌    | ❌    |
| d3d9              | ⚠️    | ⚠️    | ❌    | ❌    |
| d3d11             | ❌    | ❌    | ❌    | ❌    |
| d3d12             | ❌    | ❌    | ❌    | ❌    |
| dxgi              | ❌    | ❌    | ❌    | ❌    |
| dinput            | ❌    | ❌    | ❌    | ❌    |
| xinput            | ✔️    | ❌    | ❌    | ❌    |
| xaudio2           | ❌    | ❌    | ❌    | ❌    |
| **Low Priority**  |
| d2d               | | | | | I prefer d3d wrappers?
| dcompute          | | | | |
| dsound            | | | | | prefer xaudio2? (might be needed for gamepad headset audio?)
| dwrite            | | | | | consider uniscribe?
| dxr               | | | | |
| xact3             | | | | |
| **Not Planned**   | | | | | feel free to express interest though!
| d3d10             | | | | | d3d11 is the same but saner, and just as portable?
| ddraw             | | | | | prefer d2d, gdi, or other less ancient api?
| dplay             | | | | | prefer steamworks, raw sockets, etc.?
| xaudio1           | | | | | does this even exist? prefer xaudio2

| Legend    | Description   |
| --------- | ------------- |
| API       | API coverage seems mostly complete
| Tests     | API has basic tests
| Sound     | API has enough tests to convince me of it's soundness
| Stable    | API seems stable enough to start giving semver treatment



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



<!-- references -->
[winapi]:                   http://docs.rs/winapi/0.3/
