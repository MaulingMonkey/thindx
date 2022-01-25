

//----------------------------------------------------------------------------
// D3DXFilterTexture:
// ------------------
// Filters mipmaps levels of a texture.
//
// Parameters:
//  pBaseTexture
//      The texture object to be filtered
//  pPalette
//      256 color palette to be used, or NULL for non-palettized formats
//  SrcLevel
//      The level whose image is used to generate the subsequent levels.
//  Filter
//      D3DX_FILTER flags controlling how each miplevel is filtered.
//      Or D3DX_DEFAULT for D3DX_FILTER_BOX,
//
//----------------------------------------------------------------------------

HRESULT WINAPI
    D3DXFilterTexture(
        LPDIRECT3DBASETEXTURE9    pBaseTexture,
        CONST PALETTEENTRY*       pPalette,
        UINT                      SrcLevel,
        DWORD                     Filter);

#define D3DXFilterCubeTexture D3DXFilterTexture
#define D3DXFilterVolumeTexture D3DXFilterTexture

