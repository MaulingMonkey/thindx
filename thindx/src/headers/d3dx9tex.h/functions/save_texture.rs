

//----------------------------------------------------------------------------
// D3DXSaveTextureToFile:
// ----------------------
// Save a texture to a file.
//
// Parameters:
//  pDestFile
//      File name of the destination file
//  DestFormat
//      D3DXIMAGE_FILEFORMAT specifying file format to use when saving.
//  pSrcTexture
//      Source texture, containing the image to be saved
//  pSrcPalette
//      Source palette of 256 colors, or NULL
//
//----------------------------------------------------------------------------


HRESULT WINAPI
    D3DXSaveTextureToFileA(
        LPCSTR                    pDestFile,
        D3DXIMAGE_FILEFORMAT      DestFormat,
        LPDIRECT3DBASETEXTURE9    pSrcTexture,
        CONST PALETTEENTRY*       pSrcPalette);

HRESULT WINAPI
    D3DXSaveTextureToFileW(
        LPCWSTR                   pDestFile,
        D3DXIMAGE_FILEFORMAT      DestFormat,
        LPDIRECT3DBASETEXTURE9    pSrcTexture,
        CONST PALETTEENTRY*       pSrcPalette);

#ifdef UNICODE
#define D3DXSaveTextureToFile D3DXSaveTextureToFileW
#else
#define D3DXSaveTextureToFile D3DXSaveTextureToFileA
#endif


//----------------------------------------------------------------------------
// D3DXSaveTextureToFileInMemory:
// ----------------------
// Save a texture to a file.
//
// Parameters:
//  ppDestBuf
//      address of a d3dxbuffer pointer to return the image data
//  DestFormat
//      D3DXIMAGE_FILEFORMAT specifying file format to use when saving.
//  pSrcTexture
//      Source texture, containing the image to be saved
//  pSrcPalette
//      Source palette of 256 colors, or NULL
//
//----------------------------------------------------------------------------

HRESULT WINAPI
    D3DXSaveTextureToFileInMemory(
        LPD3DXBUFFER*             ppDestBuf,
        D3DXIMAGE_FILEFORMAT      DestFormat,
        LPDIRECT3DBASETEXTURE9    pSrcTexture,
        CONST PALETTEENTRY*       pSrcPalette);

