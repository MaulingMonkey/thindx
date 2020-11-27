# thin3dcompiler - **Thin** D**3DCompiler** wrappers

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/thin3dcompiler.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/thin3dcompiler)
[![crates.io](https://img.shields.io/crates/v/thin3dcompiler.svg)](https://crates.io/crates/thin3dcompiler)
[![docs.rs](https://docs.rs/thin3dcompiler/badge.svg)](https://docs.rs/thin3dcompiler)
[![License](https://img.shields.io/crates/l/thin3dcompiler.svg)](https://github.com/MaulingMonkey/thin3dcompiler)
[![Build Status](https://github.com/MaulingMonkey/thin3dcompiler/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/thin3dcompiler/actions?query=workflow%3Arust)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/thin3dcompiler/status.svg)](https://deps.rs/repo/github/MaulingMonkey/thin3dcompiler) -->

Thin [D3DCompiler] wrappers for Rust.

### Why not [winapi] directly?

*   This crate aims to make fns safe/sound/slightly rusty when possible
*   Attempts to verify API soundness through mass unit tests, even if they mostly test D3DCompiler's behavior.
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
[D3DCompiler]:              https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/
[winapi]:                   http://docs.rs/winapi/0.3/
