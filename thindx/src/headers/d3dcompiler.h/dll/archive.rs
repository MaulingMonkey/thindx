use crate::*;

use std::convert::TryInto;
use std::ptr::*;



/// <h1 id="archive" class="section-header"><a href="#archive">Manipulate Bytecode Archives</a></h1>
impl D3DCompiler {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompressshaders)\]
    /// D3DCompressShaders
    ///
    /// Compresses a set of shaders into a more compact form.
    ///
    /// ### Arguments
    /// *   `shaders`       - The compiled shader(s) to compress.
    /// *   `flags`         - [CompressShader] flags controlling compression.
    ///
    /// ### Returns
    /// *   Err([THINERR::MISSING_DLL_EXPORT])    - `d3dcompiler_42.dll` and earlier
    /// *   Ok([ReadOnlyBlob])
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
    /// let basic_hlsl  = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// let plain_txt   = std::fs::read(r"test\data\plain.txt").unwrap();
    /// let tocompress = [
    ///     ShaderData::from(&basic_hlsl[..]),
    ///     ShaderData::from(&plain_txt[..]),
    /// ];
    /// println!("tocompress: [{} bytes, {} bytes]", basic_hlsl.len(), plain_txt.len());
    ///
    /// let compress = compiler.compress_shaders(&tocompress, CompressShader::default()).unwrap();
    /// println!("compressed:  {} bytes", compress.get_buffer().len());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// to_compress: [493 bytes, 58 bytes]
    /// compressed:   432 bytes
    /// ```
    #[requires(!store)]
    #[requires(d3dcompiler=43)]
    pub fn compress_shaders(
        &self,
        shaders:                &[ShaderData],
        flags:                  impl Into<CompressShader>,
    ) -> Result<ReadOnlyBlob, Error> {
        // Early outs
        let f           = self.D3DCompressShaders.ok_or(Error::new("D3DCompressShaders", THINERR::MISSING_DLL_EXPORT))?;
        let num_shaders = shaders.len().try_into().map_err(|_| Error::new("D3DCompressShaders", THINERR::MISSING_DLL_EXPORT))?;

        let shader_data = shaders.as_ptr() as *mut _; // TODO: Is the `mut` sane?
        let flags       = flags.into().into();

        let mut compressed_data = null_mut();
        let hr = unsafe { f(num_shaders, shader_data, flags, &mut compressed_data) };
        Error::check("D3DCompressShaders", hr)?;
        Ok(unsafe { ReadOnlyBlob::from_raw(compressed_data) })
        // TODO: Wait, this takes multiple shaders.  Does this also *return* an array of blobs, perhaps?
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
    /// ### Returns
    /// *   Err([THINERR::MISSING_DLL_EXPORT])    - `d3dcompiler_42.dll` and earlier
    /// *   Ok([u32])                               - the number of compressed shaders contained within `src_data`
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let basic_hlsl  = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let plain_txt   = std::fs::read(r"test\data\plain.txt").unwrap();
    /// # let tocompress = [
    /// #     ShaderData::from(&basic_hlsl[..]),
    /// #     ShaderData::from(&plain_txt[..]),
    /// # ];
    /// #
    /// # let compress = compiler.compress_shaders(&tocompress, CompressShader::default()).unwrap();
    /// assert_eq!(2, compiler.decompress_shaders_count(compress.get_buffer()).unwrap());
    /// ```
    #[requires(!store)]
    #[requires(d3dcompiler=43)]
    pub fn decompress_shaders_count(&self, src_data: &[u8]) -> Result<u32, Error> {
        let f = self.D3DDecompressShaders.ok_or(Error::new("D3DDecompressShaders", THINERR::MISSING_DLL_EXPORT))?;
        let mut shader = null_mut(); // D3DDecompressShaders will fail with E_FAIL if ppShaders is null, even if it doesn't use it
        let mut total_shaders = 0;
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), 0, 0, null_mut(), 0, &mut shader, &mut total_shaders) };
        debug_assert!(shader.is_null(), "BUG: shader shouldn't have been touched!?!?");
        Error::check("D3DDecompressShaders", hr)?;
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
    ///
    /// ### Returns
    /// *   Err([THINERR::MISSING_DLL_EXPORT])            - `d3dcompiler_42.dll` and earlier
    /// *   Ok(&amp;'s \[[Option]&lt;[ReadOnlyBlob]&gt;\])  - the shader(s) that were decompressed
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let basic_hlsl  = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let plain_txt   = std::fs::read(r"test\data\plain.txt").unwrap();
    /// # let tocompress = [
    /// #     ShaderData::from(&basic_hlsl[..]),
    /// #     ShaderData::from(&plain_txt[..]),
    /// # ];
    /// #
    /// # let compress = compiler.compress_shaders(&tocompress, CompressShader::default()).unwrap();
    /// let mut decompressed = [None, None, None];
    /// let decompressed2 = compiler.decompress_shaders_inplace(
    ///     compress.get_buffer(), None, 0, &mut decompressed[..]
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
    #[requires(!store)]
    #[requires(d3dcompiler=43)]
    pub fn decompress_shaders_inplace<'s>(
        &self,
        src_data:               &[u8],
        flags:                  impl Into<Option<void::Void>>,
        start_index:            u32,
        out_shaders:            &'s mut [Option<ReadOnlyBlob>],
    ) -> Result<&'s [Option<ReadOnlyBlob>], Error> {
        let f = self.D3DDecompressShaders.ok_or(Error::new("D3DDecompressShaders", THINERR::MISSING_DLL_EXPORT))?;
        let n : u32 = out_shaders.len().try_into().map_err(|_| Error::new("D3DDecompressShaders", THINERR::SLICE_TOO_LARGE))?;
        let _ = flags;

        for shader in out_shaders.iter_mut() { *shader = None; } // D3DCompressShaders will overwrite
        let mut total_shaders = 0;
        let hr = unsafe { f(src_data.as_ptr().cast(), src_data.len(), n, start_index, null_mut(), 0, out_shaders.as_mut_ptr().cast(), &mut total_shaders) };
        Error::check("D3DDecompressShaders", hr)?;
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
    ///
    /// ### Returns
    /// *   Err([THINERR::MISSING_DLL_EXPORT])        - `d3dcompiler_42.dll` and earlier
    /// *   Ok(Vec&lt;[Option]&lt;[ReadOnlyBlob]&gt;)   - the shader(s) that were decompressed
    ///
    /// ### Example
    /// ```rust
    /// # use thindx::*; let compiler = D3DCompiler::new(47).unwrap();
    /// # let basic_hlsl  = std::fs::read(r"test\data\basic.hlsl").unwrap();
    /// # let plain_txt   = std::fs::read(r"test\data\plain.txt").unwrap();
    /// # let tocompress = [
    /// #     ShaderData::from(&basic_hlsl[..]),
    /// #     ShaderData::from(&plain_txt[..]),
    /// # ];
    /// #
    /// # let compress = compiler.compress_shaders(&tocompress, CompressShader::default()).unwrap();
    /// let decompressed = compiler.decompress_shaders(compress.get_buffer(), None, ..).unwrap();
    ///
    /// assert_eq!(2, decompressed.len());
    /// assert_eq!(basic_hlsl, decompressed[0].as_ref().unwrap().get_buffer());
    /// assert_eq!(plain_txt,  decompressed[1].as_ref().unwrap().get_buffer());
    /// ```
    #[requires(!store)]
    #[requires(d3dcompiler=43)]
    pub fn decompress_shaders<'s>(
        &self,
        src_data:               &[u8],
        flags:                  impl Into<Option<void::Void>>,
        _range:                 std::ops::RangeFull,
    ) -> Result<Vec<Option<ReadOnlyBlob>>, Error> {
        let n = self.decompress_shaders_count(src_data)? as usize;
        let mut v = Vec::new();
        v.resize_with(n, || None);
        let n2 = self.decompress_shaders_inplace(src_data, flags, 0, &mut v[..])?.len();
        debug_assert_eq!(n, n2);
        Ok(v)
    }
}
