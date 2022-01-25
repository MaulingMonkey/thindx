// missing from winapi:
#[allow(non_camel_case_types)] type D3DXIMAGE_FILEFORMAT = u32;



/// \[docs.microsoft.com\]
/// D3DXIMAGE_FILEFORMAT
/// ([BMP], [JPG], [TGA], [PNG], [DDS], [PPM], [DIB], [HDR], or [PFM])
// TODO: docs link, examples, see also, proper docs
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ImageFileFormat(D3DXIMAGE_FILEFORMAT);
#[doc(hidden)] pub use ImageFileFormat as IFF;
// no winapi equivalent

enumish! { ImageFileFormat => D3DXIMAGE_FILEFORMAT; BMP, JPG, TGA, PNG, DDS, PPM, DIB, HDR, PFM }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl IFF {
    pub const BMP : IFF = IFF(0);
    pub const JPG : IFF = IFF(1);
    pub const TGA : IFF = IFF(2);
    pub const PNG : IFF = IFF(3);
    pub const DDS : IFF = IFF(4);
    pub const PPM : IFF = IFF(5);
    pub const DIB : IFF = IFF(6);
    pub const HDR : IFF = IFF(7);
    pub const PFM : IFF = IFF(8);
}



//#cpp2rust D3DXIMAGE_FILEFORMAT    = d3dx9::ImageFileFormat

//#cpp2rust D3DXIFF_BMP             = d3dx9::IFF::BMP
//#cpp2rust D3DXIFF_JPG             = d3dx9::IFF::JPG
//#cpp2rust D3DXIFF_TGA             = d3dx9::IFF::TGA
//#cpp2rust D3DXIFF_PNG             = d3dx9::IFF::PNG
//#cpp2rust D3DXIFF_DDS             = d3dx9::IFF::DDS
//#cpp2rust D3DXIFF_PPM             = d3dx9::IFF::PPM
//#cpp2rust D3DXIFF_DIB             = d3dx9::IFF::DIB
//#cpp2rust D3DXIFF_HDR             = d3dx9::IFF::HDR
//#cpp2rust D3DXIFF_PFM             = d3dx9::IFF::PFM

//#cpp2ignore D3DXIFF_FORCE_DWORD
