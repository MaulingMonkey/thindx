
//----------------------------------------------------------------------------
// D3DXComputeNormalMap:
// ---------------------
// Converts a height map into a normal map.  The (x,y,z) components of each
// normal are mapped to the (r,g,b) channels of the output texture.
//
// Parameters
//  pTexture
//      Pointer to the destination texture
//  pSrcTexture
//      Pointer to the source heightmap texture
//  pSrcPalette
//      Source palette of 256 colors, or NULL
//  Flags
//      D3DX_NORMALMAP flags
//  Channel
//      D3DX_CHANNEL specifying source of height information
//  Amplitude
//      The constant value which the height information is multiplied by.
//---------------------------------------------------------------------------

HRESULT WINAPI
    D3DXComputeNormalMap(
        LPDIRECT3DTEXTURE9        pTexture,
        LPDIRECT3DTEXTURE9        pSrcTexture,
        CONST PALETTEENTRY*       pSrcPalette,
        DWORD                     Flags,
        DWORD                     Channel,
        FLOAT                     Amplitude);

