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
