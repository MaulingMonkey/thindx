use crate::d3d9::*;

use winapi::ctypes::c_void;



// #[test] fn draw_indexed_primitive() {}
// #[test] fn draw_indexed_primitive_up() {}
// #[test] fn draw_primitive() {}
// #[test] fn draw_primitive_up() {}
// #[test] fn draw_rect_patch() {}
// #[test] fn draw_tri_patch() {}



/// [IDirect3DDevice9Ext::draw_indexed_primitive_up] index data
///
/// ### ⚠️ Safety ⚠️
/// *   [`indicies`] is the number of **indicies**, not bytes!
/// *   [`indicies`] should be "reasonable" (OOMs/overflows may result in undefined behavior)
/// *   [`ptr`] must point to valid memory, appropriated aligned based on format.
/// *   [`format`] should return a valid index format ([`Format::INDEX16`] or [`Format::INDEX32`])
pub unsafe trait IndexData {
    fn bytes(&self) -> usize;
    fn indicies(&self) -> usize;
    fn as_ptr(&self) -> *const c_void;
    fn format(&self) -> Format;
}

unsafe impl IndexData for &[u16] {
    fn bytes(&self) -> usize { std::mem::size_of_val(*self) }
    fn indicies(&self) -> usize { self.len() }
    fn as_ptr(&self) -> *const c_void { (*self).as_ptr().cast() }
    fn format(&self) -> Format { Format::INDEX16 }
}

unsafe impl IndexData for &[u32] {
    fn bytes(&self) -> usize { std::mem::size_of_val(*self) }
    fn indicies(&self) -> usize { self.len() }
    fn as_ptr(&self) -> *const c_void { (*self).as_ptr().cast() }
    fn format(&self) -> Format { Format::INDEX32 }
}



/// [IDirect3DDevice9Ext::draw_indexed_primitive_up] vertex data
///
/// ### ⚠️ Safety ⚠️
/// *   [`verticies`] is the number of **elements**, not bytes!
/// *   [`verticies`] and [`stride`] should be "reasonable" (OOMs/overflows may result in undefined behavior)
/// *   [`as_ptr`] must point to valid memory, appropriated aligned, of at least [`verticies`] * [`stride`] bytes
pub unsafe trait VertexStreamData {
    fn bytes(&self) -> usize;
    fn verticies(&self) -> usize;
    fn as_ptr(&self) -> *const c_void;
    fn stride(&self) -> u32;
}

unsafe impl<E: Copy> VertexStreamData for &[E] {
    fn bytes(&self) -> usize { std::mem::size_of_val(*self) }
    fn verticies(&self) -> usize { self.len() }
    fn as_ptr(&self) -> *const c_void { (*self).as_ptr().cast() }
    fn stride(&self) -> u32 { std::mem::size_of::<E>() as u32 }
}
