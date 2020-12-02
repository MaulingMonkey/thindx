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
[winapi]:                   http://docs.rs/winapi/0.3/
