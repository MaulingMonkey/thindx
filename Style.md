### Doc Comments

Function doc comments should generally follow the following style, skipping irrelevant sections
(e.g. if the fn takes no arguments, remove the entire "### Arguments" header and section.)

```rust
/// \[[docs.microsoft.com](https://docs.microsoft.com/...)\]
/// FunctionName
///
/// Short description of the function and it's behavior
///
/// ### Safety
/// Any safety invariants to be aware of for this fn or structure.  Required on all unsafe fns.
/// Occasionally appropriate on "safe" fns or structures.
///
/// ### Usage
/// ```text
/// ...psuedo-syntax to make arguments clear for macros only...
/// ```
///
/// ### Arguments
/// *   `arg1`          - Description of the first argument.
/// *   `arg2`          - Description of the second argument.
///                       Multi-line comments continued indented.
/// *   `arg3`          - Description of the third argument.
///
/// ### Errors
/// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_4?.dll` and earlier
/// *   [D3DERR::INVALIDCALL]           - ...
/// *   [E::FAIL]                       - ...
///
/// ### Returns
/// *   [Some]([Thing]) - only bother with this when the description and/or type alone isn't informative enough
/// *   [None]          - should describe the fn-specific conditions for the variations
///
/// ### Examples
/// ```rust
/// # use thindx::{*, d3d::*, d3d11::*};
/// # let d3dc = Compiler::new(47).unwrap();
/// println!("Hi!");
/// ```
///
/// ### Output
/// ```text
/// Hi!
/// ```
///
/// ### See Also
/// *   [d3d::Structure]
/// *   [d3d11::Structure]
/// *   [examples::some_example]
///
/// ### Remarks
/// Rarely, some more detailed, longer rambling is appropriate.
///
/// [cite1]:            link1
/// [cite2]:            link2
/// [cite3]:            link3
```

### Naming Things

| Win32                     | thindx                    |
| ------------------------- | ------------------------- |
| **interfaces**            |
| `ID3DSomeInterface`       | `d3d::SomeInterface`
| `IDirect3DSomeInterface9` | `d3d9::SomeInterface`
| `ID3D11SomeInterface`     | `d3d11::SomeInterface`
| **structures**            |
| `D3D_SOME_STRUCT`         | `d3d::SomeStruct`
| `D3D11_SOME_STRUCT`       | `d3d11::SomeStruct`
| **flags/enums**           |
| `D3D_FLAG_VALUE`          | `d3d::Flag::Value`
| `D3D11_FLAG_VALUE`        | `d3d11::Flag::Value`
| **identifiers**           |
| `Type`                    | `ty` (not `r#type` nor `type_`)
