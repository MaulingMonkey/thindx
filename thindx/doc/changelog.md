Revision history

# 0.0.0-unsound.x

### **0.0.0-unsound.6** - Safety, coverage, natvis
* API additions
    * [`d3d`] constants, device fns, safe(?) buffer/texture construction for [`examples::d3d9_02_xinput`]
    * [`guid!`]
    * [`make_four_cc`]
* External changes
    * `*.natvis` files for improved debugging
    * Readme.md overhaul
    * Github pages (preview docs, test coverage)
* Internal changes
    * `cpp2rust.txt` spread out into `//#cpp2rust` comments

### **0.0.0-unsound.5** - Docs, examples, refactoring, coverage
* API changes
    * [`d3d::Compiler`] construction tweaked to discourage dll preloading attacks
    * [`d3d::Compiler::strip_shader`] now demands [Bytecode](d3d::Bytecode)
    * [`d3d::Caps`] rusty types
    * MSRV 1.58
* API additions
    * [`d3d`] constants, ext fn accessors
    * [`xinput::Gamepad`] constants
* External changes
    * [`examples::d3d9_02_xinput`]
    * Misc. docs improvements
* Internal changes
    * `xtask`: `coverage`, `publish`
    * `// SAFETY:` soundness doc spam
    * Clippy spam
    * Finish merging `thin3d9`

### **0.0.0-unsound.n** - Earlier
* **0.0.0-unsound.4** - Fixed [`xinput`] delay loading, test coverage
* **0.0.0-unsound.3** - Initial [`xinput`] support
* **0.0.0-unsound.2** - Fixed <https://docs.rs/thindx>
* **0.0.0-unsound.1** - First published to <https://crates.io/crates/thindx>
* **0.0.0-reserved** - Placeholder crate
