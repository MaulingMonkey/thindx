Revision history

# 0.0.0-unsound.x

### **0.0.0-unsound.6** - Natvis
* API additions
    * `d3d::*` constants, ext fn accessors
    * `make_four_cc`
* External changes
    * `*.natvis` files for improved debugging
    * Misc. docs improvements

### **0.0.0-unsound.5** - Docs, examples, refactoring, coverage
* API changes
    * `d3d::Compiler` construction tweaked to discourage dll preloading attacks
    * `d3d::Compiler::strip_shader` now demands Bytecode
    * `d3d::Caps` rusty types
    * MSRV 1.58
* API additions
    * `d3d::*` constants, ext fn accessors
    * `xinput::Gamepad::*` constants
    * `make_four_cc`
* External changes
    * `examples/d3d9-02-xinput`
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
