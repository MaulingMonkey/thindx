use crate::*;
use crate::d3d::*;

use std::convert::TryInto;
use std::ptr::*;



/// <h1 id="archive" class="section-header"><a href="#archive">Manipulate Bytecode Archives</a></h1>
impl Compiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompressshaders)\]
    /// D3DCompressShaders
    ///
    /// Compresses a set of shaders into a more compact form.
    ///
    /// ### Arguments
    /// *   `shaders`       - The compiled shader(s) to compress.
    /// *   `flags`         - [CompressShader] flags controlling compression.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_42.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// let basic_hlsl  = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// let plain_txt   = std::fs::read(r"test\data\plain.txt").unwrap();
    /// let tocompress = [
    ///     ShaderData::from(&basic_hlsl[..]),
    ///     ShaderData::from(&plain_txt[..]),
    /// ];
    /// println!("tocompress: [{} bytes, {} bytes]", basic_hlsl.len(), plain_txt.len());
    ///
    /// let compress = d3dc.compress_shaders(&tocompress, CompressShader::default()).unwrap();
    /// println!("compressed:  {} bytes", compress.len());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// to_compress: [493 bytes, 58 bytes]
    /// compressed:   432 bytes
    /// ```
    ///
    /// ### Remarks
    /// *   You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    /// *   This was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.
    pub fn compress_shaders(
        &self,
        shaders:                &[ShaderData],
        flags:                  impl Into<CompressShader>,
    ) -> Result<BytesBlob, MethodError> {
        // Early outs
        let f           = self.D3DCompressShaders.ok_or(MethodError("D3DCompressShaders", THINERR::MISSING_DLL_EXPORT))?;
        let num_shaders = shaders.len().try_into().map_err(|_| MethodError::new("D3DCompressShaders", THINERR::MISSING_DLL_EXPORT))?;

        // SAFETY: ⚠️ This sketchy mut cast *should* be sane (famous last words.)
        // We're only casting away the immutability of the D3D_SHADER_DATA elements, not the bytecode those elements point to.
        // The array is far too small to reuse as an in-place compression buffer or anything like that.
        // Additionally, in manual testing, I see no mutation of the underlying array.
        // Finally, `pShaderData` is marked `_In_reads_(uNumShaders)` - if this was actually written to, it should be `_Inout_count_` instead.
        let shader_data = shaders.as_ptr() as *mut _;

        let flags       = flags.into().into();

        // SAFETY: ❌ needs fuzz testing for alloc overflow bugs
        //  * `f`               ✔️ should be valid/sound like all `self.*`
        //  * `num_shaders`     ⚠️ could truncate within `D3DCompressShaders` (harmless?)
        //  * `shader_data`     ❌ could overflow an alloc within `D3DCompressShaders`
        //  * `shader_data`     ✔️ may contain arbitrary non-shader data
        //  * `flags`           ⚠️ could be invalid
        //  * `compressed_data` ✔️ is a trivial out-param
        let mut compressed_data = null_mut();
        let hr = unsafe { f(num_shaders, shader_data, flags, &mut compressed_data) };
        MethodError::check("D3DCompressShaders", hr)?;

        // SAFETY: ✔️
        //  * `compressed_data` ✔️ is either null (from_raw panics) or a valid, owned, non-dangling ID3DBlob (ownership transfers)
        //  * `ReadOnlyBlob`    ✔️ imposes no restrictions on `compressed_data`'s contents.
        Ok(BytesBlob::new(unsafe { ReadOnlyBlob::from_raw(compressed_data) }))
    }

    // D3DDecompressShaders behavior:
    //  * if `ppShaders` is null, E_FAIL, even if you're requesting 0 shaders
    //  * if `indices` is specified:
    //      * `start_index` is ignored
    //      * `indices[0..num_shaders]` is read for archive indexes to read
    //      * if `indices` has duplicates, those are null
    //  * if `indices` is null:
    //      * treated as if indicies == (start_index .. start_index + num_shaders).collect()
    //
    // https://github.com/MaulingMonkey/thindx/issues/5

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddecompressshaders)\]
    /// D3DDecompressShaders
    ///
    /// Get the number of compressed shaders in the `src_data` archive.
    ///
    /// ### Arguments
    /// *   `src_data`      - The compressed shaders.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_42.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let basic_hlsl  = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let plain_txt   = std::fs::read(r"test\data\plain.txt").unwrap();
    /// # let tocompress = [
    /// #     ShaderData::from(&basic_hlsl[..]),
    /// #     ShaderData::from(&plain_txt[..]),
    /// # ];
    /// #
    /// # let compress = d3dc.compress_shaders(&tocompress, CompressShader::default()).unwrap();
    /// assert_eq!(2, d3dc.decompress_shaders_count(&compress).unwrap());
    /// ```
    ///
    /// ### Remarks
    /// *   You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    /// *   This was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.
    pub fn decompress_shaders_count(&self, src_data: &[u8]) -> Result<u32, MethodError> {
        let f = self.D3DDecompressShaders.ok_or(MethodError("D3DDecompressShaders", THINERR::MISSING_DLL_EXPORT))?;
        let mut shader = null_mut(); // D3DDecompressShaders will fail with E_FAIL if ppShaders is null, even if it doesn't use it
        let mut total_shaders = 0;

        // SAFETY: ❌ needs fuzz testing for alloc overflow and corruption bugs
        //  * `f`               ✔️ should be valid/sound like all `self.*`
        //  * `src_data`        ⚠️ probably won't overflow an alloc within `D3DDecompressShaders` with these params?
        //  * `src_data`        ❌ could contain corrupt decompression data
        //  * `shader`          ✔️ is unused except for a null ptr check
        //  * `total_shaders`   ⚠️ could be truncated at the 4 GB mark
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), 0, 0, null_mut(), 0, &mut shader, &mut total_shaders) };
        debug_assert!(shader.is_null(), "BUG: shader shouldn't have been touched!?!?");
        MethodError::check("D3DDecompressShaders", hr)?;
        Ok(total_shaders)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddecompressshaders)\]
    /// D3DDecompressShaders
    ///
    /// Decompresses one or more shaders from a compressed set.
    ///
    /// ### Arguments
    /// *   `src_data`      - The compressed shaders to decompress.
    /// *   `flags`         - Reserved (pass [None]).
    /// *   `start_index`   - The first shader to read, by archive index order.
    /// *   `out_shaders`   - An output buffer of shaders to read.  The size of the buffer dictates how many shaders will be read.
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_42.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let basic_hlsl  = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let plain_txt   = std::fs::read(r"test\data\plain.txt").unwrap();
    /// # let tocompress = [
    /// #     ShaderData::from(&basic_hlsl[..]),
    /// #     ShaderData::from(&plain_txt[..]),
    /// # ];
    /// #
    /// # let compress = d3dc.compress_shaders(&tocompress, CompressShader::default()).unwrap();
    /// let mut decompressed = [None, None, None];
    /// let decompressed2 = d3dc.decompress_shaders_inplace(
    ///     &compress, None, 0, &mut decompressed[..]
    /// ).unwrap();
    ///
    /// assert_eq!(2, decompressed2.len());
    /// assert_eq!(basic_hlsl, decompressed2[0].as_ref().unwrap().get_buffer());
    /// assert_eq!(plain_txt,  decompressed2[1].as_ref().unwrap().get_buffer());
    ///
    /// assert_eq!(&basic_hlsl[..], decompressed[0].as_ref().unwrap().get_buffer());
    /// assert_eq!(&plain_txt[..],  decompressed[1].as_ref().unwrap().get_buffer());
    /// assert!(decompressed[2].is_none());
    /// ```
    ///
    /// ### Remarks
    /// *   You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    /// *   This was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.
    pub fn decompress_shaders_inplace<'s>(
        &self,
        src_data:               &[u8],
        flags:                  impl Into<Option<std::convert::Infallible>>,
        start_index:            u32,
        out_shaders:            &'s mut [Option<ReadOnlyBlob>],
    ) -> Result<&'s [Option<ReadOnlyBlob>], MethodError> {
        let f = self.D3DDecompressShaders.ok_or(MethodError("D3DDecompressShaders", THINERR::MISSING_DLL_EXPORT))?;
        let n : u32 = out_shaders.len().try_into().map_err(|_| MethodError::new("D3DDecompressShaders", THINERR::SLICE_TOO_LARGE))?;
        let _ = flags;

        for shader in out_shaders.iter_mut() { *shader = None; } // D3DCompressShaders will overwrite
        let mut total_shaders = 0;

        // SAFETY: ❌ needs fuzz testing for alloc overflow and corruption bugs
        //  * `f`               ✔️ should be valid/sound like all `self.*`
        //  * `src_data`        ❌ could overflow an alloc within `D3DDecompressShaders`
        //  * `src_data`        ❌ could contain corrupt decompression data
        //  * `n`               ⚠️ "shouldn't" (famous last words) cause cause alloc overflow issues inside `D3DDecompressShaders` ?
        //  * `flags`           ✔️ are reserved / 0
        //  * `start_index`     ❌ could be out-of-bounds
        //  * `out_shaders`     ✔️ should be ABI compatible thanks to the #[repr(transparent)] conversion chain: Option<ReadOnlyBlob> -> Option<mcom::Rc<ID3DBlob>> -> Option<NonNull<ID3DBlob>> -> `*mut ID3DBlob`
        //  * `out_shaders`     ✔️ should not leak (explicitly `None`ed out earlier)
        //  * `out_shaders`     ✔️ should contain sufficient elements to avoid overflow (`n` inferred from `.len`)
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), n, start_index, null_mut(), 0, out_shaders.as_mut_ptr().cast(), &mut total_shaders) };
        MethodError::check("D3DDecompressShaders", hr)?;
        let read = n.min(total_shaders) as usize;
        Ok(&out_shaders[..read])
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddecompressshaders)\]
    /// D3DDecompressShaders
    ///
    /// Decompresses one or more shaders from a compressed set.
    ///
    /// ### Arguments
    /// *   `src_data`      - The compressed shaders to decompress.
    /// *   `flags`         - Reserved (pass [None]).
    /// *   `_range`        - The archive index range to read (pass `..` - other options may be supported in the future.)
    ///
    /// ### Errors
    /// *   [THINERR::MISSING_DLL_EXPORT]   - `d3dcompiler_42.dll` and earlier
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::d3d::*; let d3dc = Compiler::load_system(47).unwrap();
    /// # let basic_hlsl  = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let plain_txt   = std::fs::read(r"test\data\plain.txt").unwrap();
    /// # let tocompress = [
    /// #     ShaderData::from(&basic_hlsl[..]),
    /// #     ShaderData::from(&plain_txt[..]),
    /// # ];
    /// #
    /// # let compress = d3dc.compress_shaders(&tocompress, CompressShader::default()).unwrap();
    /// let decompressed = d3dc.decompress_shaders(&compress, None, ..).unwrap();
    ///
    /// assert_eq!(2, decompressed.len());
    /// assert_eq!(basic_hlsl, decompressed[0].as_ref().unwrap().get_buffer());
    /// assert_eq!(plain_txt,  decompressed[1].as_ref().unwrap().get_buffer());
    /// ```
    ///
    /// ### Remarks
    /// *   You can use this API to develop your Windows Store apps, but you can't use it in apps that you submit to the Windows Store.
    /// *   This was introduced by d3dcompiler_43.dll, and is unavailable in earlier versions.
    pub fn decompress_shaders(
        &self,
        src_data:               &[u8],
        flags:                  impl Into<Option<std::convert::Infallible>>,
        _range:                 std::ops::RangeFull,
    ) -> Result<Vec<Option<ReadOnlyBlob>>, MethodError> {
        let n = self.decompress_shaders_count(src_data)? as usize;
        let mut v = Vec::new();
        v.resize_with(n, || None);
        let n2 = self.decompress_shaders_inplace(src_data, flags, 0, &mut v[..])?.len();
        debug_assert_eq!(n, n2);
        Ok(v)
    }
}
