use crate::d3d9::IDirect3DDevice9Ext;



/// Reference a 2-dimensional array of pixels for use with e.g. [IDirect3DDevice9Ext::create_texture_from].
pub struct TextureMipRef<'p> {
    /// Raw pixel bytes.  Perhaps sourced from [bytemuck::bytes_of]\(your_typed_pixels\)
    pub data:           &'p [u8], // good use case for bytemuck::bytes_of(your_pixels_slice)

    /// The number of **bytes** between rows of pixel blocks.
    ///
    /// For most formats, this is equivalent to rows of pixels, but for DXT formats, this will be for rows of 4x4 pixel blocks.
    pub stride:         usize,
}

#[cfg(not_yet)]
/// Reference a 3-dimensional array of pixels for use with e.g. [IDirect3DDevice9Ext::create_volume_texture_from].
pub struct VolumeTextureMipRef<'p> {
    /// Raw pixel bytes.  Perhaps sourced from [bytemuck::bytes_of]\(your_typed_pixels\)
    pub data:           &'p [u8],

    /// The number of **bytes** between rows of pixel blocks.
    ///
    /// For most formats, this is equivalent to rows of pixels, but for DXT formats, this will be for rows of 4x4 pixel blocks.
    pub stride_row:     usize,

    /// The number of **bytes** between 2d planes / slices of pixels.
    pub stride_slice:   usize,
}

#[cfg(not_yet)]
/// Reference 6x[TextureMipRef]s to form cubemap dara for use with e.g. [IDirect3DDevice9Ext::create_cube_texture_from].
pub struct CubeTextureMipRef<'p> {
    /// The cubemap face covering the +X direction
    pub pos_x: TextureMipRef<'p>,

    /// The cubemap face covering the -X direction
    pub neg_x: TextureMipRef<'p>,

    /// The cubemap face covering the +Y direction
    pub pos_y: TextureMipRef<'p>,

    /// The cubemap face covering the -Y direction
    pub neg_y: TextureMipRef<'p>,

    /// The cubemap face covering the +Z direction
    pub pos_z: TextureMipRef<'p>,

    /// The cubemap face covering the -Z direction
    pub neg_z: TextureMipRef<'p>,
}
