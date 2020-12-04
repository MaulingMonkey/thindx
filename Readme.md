# thindx - **Thin** **D**irect**X** wrappers

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/thindx.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/thindx)
[![License](https://img.shields.io/crates/l/thindx.svg)](https://github.com/MaulingMonkey/thindx)
[![Build Status](https://github.com/MaulingMonkey/thindx/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/thindx/actions?query=workflow%3Arust)
<!--
[![crates.io](https://img.shields.io/crates/v/thindx.svg)](https://crates.io/crates/thindx)
[![docs.rs](https://docs.rs/thindx/badge.svg)](https://docs.rs/thindx)
-->
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/thindx/status.svg)](https://deps.rs/repo/github/MaulingMonkey/thindx) -->

Thin DirectX wrappers for Rust.

### Why not [winapi] directly?

*   This crate aims to make fns safe/sound/slightly rusty when possible
*   Attempts to verify API soundness through mass unit tests, even if they mostly test DirectX's behavior.
*   Doc comments for one-stop intellisense, safety documentation, etc.

### Why not `<other graphics crate>`?

*   Most other graphics crates focus on **hiding** the underlying graphics API as much as possible, improving application portability.
*   This crate **surfaces** the underlying graphics API as much as possible, improving the potential for interoperability with other graphics code / use in middleware applications.

### Is this crate sound?

Probably not.  I'm exposing a huge legacy C++ API to Rust.  Mistakes will happen.

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



### Project Status

| Overall       | Done  |
| ------------- | ----- |
| crates.io     | ❌    |
| docs.rs       | ❌    |

| DirectX           | API   | Tests | Sound | Stable| Notes |
| ----------------- | ----- | ----- | ----- | ----- | ----- |
| `d3dcompiler.dll` | ✔️    | ⚠️    | ⚠️    | ❌    | first goal to stabilize
| `dxcompiler.dll`  | ❌    | ❌    | ❌    | ❌    |
| d3d9              | ⚠️    | ❌    | ❌    | ❌    | separate, private branch to be merged
| d3d11             | ❌    | ❌    | ❌    | ❌    |
| d3d12             | ❌    | ❌    | ❌    | ❌    |
| dxgi              | ❌    | ❌    | ❌    | ❌    |
| dinput            | ❌    | ❌    | ❌    | ❌    |
| dsound            | ❌    | ❌    | ❌    | ❌    |
| xinput            | ❌    | ❌    | ❌    | ❌    |
| xaudio2           |
| **Low Priority**  |
| d2d               |
| dcompute          |
| dwrite            |
| dxr               |
| xact3             |
| **Not Planned**   |       |       |       |       | feel free to express interest though!
| d3d10             |
| ddraw             |
| dplay             |
| xaudio1           |

| Legend    | Description   |
| --------- | ------------- |
| API       | API coverage seems mostly complete
| Tests     | API has basic tests
| Sound     | API has enough tests to convince me of it's soundness
| Stable    | API seems stable enough to start giving semver treatment



<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



<!-- references -->
[winapi]:                   http://docs.rs/winapi/0.3/
