

//----------------------------------------------------------------------------
// D3DXSaveVolumeToFile:
// ---------------------
// Save a volume to a image file.
//
// Parameters:
//  pDestFile
//      File name of the destination file
//  DestFormat
//      D3DXIMAGE_FILEFORMAT specifying file format to use when saving.
//  pSrcVolume
//      Source volume, containing the image to be saved
//  pSrcPalette
//      Source palette of 256 colors, or NULL
//  pSrcBox
//      Source box, or NULL for the entire volume
//
//----------------------------------------------------------------------------

HRESULT WINAPI
    D3DXSaveVolumeToFileA(
        LPCSTR                    pDestFile,
        D3DXIMAGE_FILEFORMAT      DestFormat,
        LPDIRECT3DVOLUME9         pSrcVolume,
        CONST PALETTEENTRY*       pSrcPalette,
        CONST D3DBOX*             pSrcBox);

HRESULT WINAPI
    D3DXSaveVolumeToFileW(
        LPCWSTR                   pDestFile,
        D3DXIMAGE_FILEFORMAT      DestFormat,
        LPDIRECT3DVOLUME9         pSrcVolume,
        CONST PALETTEENTRY*       pSrcPalette,
        CONST D3DBOX*             pSrcBox);

#ifdef UNICODE
#define D3DXSaveVolumeToFile D3DXSaveVolumeToFileW
#else
#define D3DXSaveVolumeToFile D3DXSaveVolumeToFileA
#endif


//----------------------------------------------------------------------------
// D3DXSaveVolumeToFileInMemory:
// ---------------------
// Save a volume to a image file.
//
// Parameters:
//  pDestFile
//      File name of the destination file
//  DestFormat
//      D3DXIMAGE_FILEFORMAT specifying file format to use when saving.
//  pSrcVolume
//      Source volume, containing the image to be saved
//  pSrcPalette
//      Source palette of 256 colors, or NULL
//  pSrcBox
//      Source box, or NULL for the entire volume
//
//----------------------------------------------------------------------------

HRESULT WINAPI
    D3DXSaveVolumeToFileInMemory(
        LPD3DXBUFFER*             ppDestBuf,
        D3DXIMAGE_FILEFORMAT      DestFormat,
        LPDIRECT3DVOLUME9         pSrcVolume,
        CONST PALETTEENTRY*       pSrcPalette,
        CONST D3DBOX*             pSrcBox);

