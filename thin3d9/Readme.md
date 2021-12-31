# thin3d9 - **Thin** Direct**3D9** wrappers

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/thin3d9.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/thin3d9)
[![crates.io](https://img.shields.io/crates/v/thin3d9.svg)](https://crates.io/crates/thin3d9)
[![docs.rs](https://docs.rs/thin3d9/badge.svg)](https://docs.rs/thin3d9)
[![License](https://img.shields.io/crates/l/thin3d9.svg)](https://github.com/MaulingMonkey/thin3d9)
[![Build Status](https://github.com/MaulingMonkey/thin3d9/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/thin3d9/actions?query=workflow%3Arust)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/thin3d9/status.svg)](https://deps.rs/repo/github/MaulingMonkey/thin3d9) -->

Thin [Direct3D9] wrappers for Rust, designed to be suitable for:
*   Greenfield projects targeting ancient APIs for some reason
*   Graphics middleware looking to integrate into existing D3D9 codebases

### Why not [winapi] directly?

*   This crate aims to make fns safe/sound/slightly rusty when possible
*   Attempts to verify API soundness through mass unit tests, even if they mostly test Direct3D9's behavior.
*   Doc comments for one-stop intellisense, safety documentation, etc.

### Why not `<other graphics crate>`?

*   Most other graphics crates focus on **hiding** the underlying graphics API as much as possible, improving application portability.
*   This crate **surfaces** the underlying graphics API as much as possible, improving the potential for interoperability with other graphics code / use in middleware applications.

### Is this crate sound?

Probably not.  I'm exposing a huge legacy C++ API to Rust.  Mistakes will happen.

That said, soundness *is* a **very high priority** goal.  `thin3d9` will add things like extra bounds checks, parameter
validation, extra init, etc. if need be in order to ensure soundness in safe fns whenever possible.  When it's not
possible to validate unsoundness away, the fns in question should be marked `unsafe`.  This crate strives to be sounder
than whatever manual FFI you'd write yourself would be, and that's a **high** bar.

But there are some practical limits to this.  If a background driver thread invokes UB if it fails to allocate memory,
without any direct correlation to specific API misuse like a large integer overflowing, that's a bug I can't sanely
mitigate via safe fns.  I mean, theoretically I could write a pure-software clone of the entire DirectX runtime... but no.

Additionally, while I'm seeking to validate my APIs via testing, older implementations of the APIs in question may have
more bugs / unchecked parameters / ??? that I'll fail to mitigate due to being unable to trigger them myself.  Ditto for
the proxy objects returned by various graphics debuggers.  While I'm happy to investigate, accept pull requests, expand
test coverage, etc. it's worth assuming this crate is unsound on older versions unless you've tested yourself.



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
[Direct3D9]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/
[winapi]:                   http://docs.rs/winapi/0.3/
