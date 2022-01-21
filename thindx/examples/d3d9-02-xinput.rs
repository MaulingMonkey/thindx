//! [d3d9] + [xinput] sample
#![windows_subsystem = "windows"]

use thindx::*;
use thindx::d3d9::*;

use mmrbi::*;

use raw_window_handle::*;

use winapi::shared::d3d9types::*;
use winapi::um::objbase::CoInitialize;

use winit::dpi::*;
use winit::event::{Event::*, WindowEvent::*};
use winit::event_loop::*;
use winit::window::*;

use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::mem::size_of_val;
use std::ptr::null_mut;



fn main() {
    dev::win32::optional_dev_init();
    unsafe { CoInitialize(null_mut()) };
    let event_loop  = EventLoop::new();
    let window      = WindowBuilder::new()
        .with_title("d3d9-02-xinput - thindx example")
        .with_inner_size(Size::Physical(PhysicalSize { width: 800, height: 600 }))
        .with_visible(!dev::d3d9::hide_for_docs_gen())
        .build(&event_loop).unwrap();

    let hwnd = match window.raw_window_handle() {
        RawWindowHandle::Win32(Win32Handle { hwnd, .. }) => hwnd.cast(),
        other => panic!("Expected RawWindowHandle::Windows(...), got {:?} instead", other),
    };

    let mut pp = D3DPRESENT_PARAMETERS { // TODO: replace with d3d::PresentParameters
        Windowed:               true.into(),
        hDeviceWindow:          hwnd,
        SwapEffect:             SwapEffect::Discard.into(),
        PresentationInterval:   Present::IntervalOne.into(),
        .. unsafe { std::mem::zeroed() }
    };

    let behavior =
        // Create::DisablePrintScreen | // d3d9ex
        Create::FpuPreserve |
        Create::HardwareVertexProcessing |
        Create::NoWindowChanges;

    let d3d         = unsafe { Direct3D::create(SdkVersion::default()) }.unwrap();
    let mut device  = Some(unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }.unwrap());
    let mut assets  = None;

    event_loop.run(move |event, _, control_flow|{
        *control_flow = ControlFlow::Poll;

        match event {
            WindowEvent { event: CloseRequested, window_id } if window_id == window.id() => {
                std::process::exit(0); // Ensure Device outlasts closing HWND!
            },
            WindowEvent { event: Resized(size), window_id } if window_id == window.id() => {
                let _ = size; // TODO: resize buffers, set viewport
            },
            WindowEvent { event: Focused(focus), window_id } if window_id == window.id() => {
                xinput::enable(focus);
            },
            MainEventsCleared => {
                let present_err;
                if let Some(device) = device.as_ref() {
                    if assets.is_none() { assets = Some(Assets::new(&device)); }
                    let assets = assets.as_ref().unwrap();

                    device.clear(None, Some(Color::argb(0xFF000000)), None, None).unwrap();

                    let _ = device.begin_scene();
                    let _ = device.set_stream_source(0, &assets.QuadVB, 0, Vertex::STRIDE);
                    let _ = device.set_indices(&assets.QuadIB);
                    let _ = device.set_vertex_declaration(&assets.VertDecl);
                    let _ = device.set_material(Material { ambient: ColorValue { r: 1.0, g: 1.0, b: 1.0, a: 0.0 }, ..Default::default() });
                    unsafe {
                        let device = &*device.as_raw();

                        // TODO: device.set_render_state[_unchecked]
                        device.SetRenderState(D3DRS_LIGHTING,           1                   );
                        device.SetRenderState(D3DRS_ALPHABLENDENABLE,   1                   );
                        device.SetRenderState(D3DRS_DESTBLEND,          D3DBLEND_INVSRCALPHA);
                        device.SetRenderState(D3DRS_AMBIENT,            0xFFFFFFFF          );
                    }
                    unsafe {
                        // TODO: make these safe? can't seem to crash
                        let _ = device.set_sampler_state_unchecked(0, d3d::Samp::MinFilter, d3d::TexF::Linear);
                        let _ = device.set_sampler_state_unchecked(0, d3d::Samp::MagFilter, d3d::TexF::Linear);
                        let _ = device.set_sampler_state_unchecked(0, d3d::Samp::MipFilter, d3d::TexF::Linear);
                    }

                    let sx = 2.0 / 800.0;
                    let sy = 2.0 / 600.0;

                    let user = xinput::User::Zero;
                    let state = xinput::get_state(user).ok().filter(|_| !dev::d3d9::hide_for_docs_gen()).unwrap_or(xinput::State::default());

                    let _ = xinput::set_state(user, xinput::Vibration {
                        left_motor_speed:  0x101 * (state.left_trigger  as u16),
                        right_motor_speed: 0x101 * (state.right_trigger as u16),
                    });

                    use xinput::Buttons;
                    let asset_dpad;
                    if      state.buttons.any_held(Buttons::DPadUp)     { asset_dpad = &assets.Dpad_Up; }
                    else if state.buttons.any_held(Buttons::DPadRight)  { asset_dpad = &assets.Dpad_Right; }
                    else if state.buttons.any_held(Buttons::DPadDown)   { asset_dpad = &assets.Dpad_Down; }
                    else if state.buttons.any_held(Buttons::DPadLeft)   { asset_dpad = &assets.Dpad_Left; }
                    else                                                { asset_dpad = &assets.Dpad; }

                    // I intentionally apply no deadzone to these values
                    let lx = state.left_thumb_x  as f32 * 50.0 / (i16::MAX as f32);
                    let ly = state.left_thumb_y  as f32 * 50.0 / (i16::MAX as f32);
                    let rx = state.right_thumb_x as f32 * 50.0 / (i16::MAX as f32);
                    let ry = state.right_thumb_y as f32 * 50.0 / (i16::MAX as f32);
                    let lt = state.left_trigger  as f32 * 50.0 / (u8::MAX as f32);
                    let rt = state.right_trigger as f32 * 50.0 / (u8::MAX as f32);

                    for (    dx       ,    dy       , texture,           scale, bri) in [
                        (-330.0       , 190.0 - lt  , &assets.LT,          1.0, 128 + state.left_trigger /2),
                        ( 330.0       , 190.0 - rt  , &assets.RT,          1.0, 128 + state.right_trigger/2),
                        (-220.0       , 230.0       , &assets.LB,          1.0, if state.buttons.any_held(Buttons::LeftShoulder ) { 255 } else { 128 }),
                        ( 220.0       , 230.0       , &assets.RB,          1.0, if state.buttons.any_held(Buttons::RightShoulder) { 255 } else { 128 }),

                        ( 300.0       ,  30.0 - 60.0, &assets.A,           0.7, if state.buttons.any_held(Buttons::A) { 255 } else { 128 }),
                        ( 300.0 + 60.0,  30.0       , &assets.B,           0.7, if state.buttons.any_held(Buttons::B) { 255 } else { 128 }),
                        ( 300.0 - 60.0,  30.0       , &assets.X,           0.7, if state.buttons.any_held(Buttons::X) { 255 } else { 128 }),
                        ( 300.0       ,  30.0 + 60.0, &assets.Y,           0.7, if state.buttons.any_held(Buttons::Y) { 255 } else { 128 }),

                        ( 100.0       ,  30.0       , &assets.Start,       0.7, if state.buttons.any_held(Buttons::Start) { 255 } else { 128 }),
                        // Guide Button?
                        (-100.0       ,  30.0       , &assets.Back,        0.7, if state.buttons.any_held(Buttons::Back) { 255 } else { 128 }),

                        (-300.0 + lx  ,  30.0 + ly  , &assets.Left_Stick,  1.5, if state.buttons.any_held(Buttons::LeftThumb) { 255 } else { 128 }),
                        (-150.0       ,-130.0       , asset_dpad,          1.5, if state.buttons.any_held(Buttons::DPadDown | Buttons::DPadRight | Buttons::DPadLeft | Buttons::DPadUp) { 255 } else { 128 }),
                        ( 150.0 + rx  ,-130.0 + ry  , &assets.Right_Stick, 1.5, if state.buttons.any_held(Buttons::RightThumb) { 255 } else { 128 }),
                    ].iter().copied() {
                        // half texel fixups
                        let dx = dx + 0.5;
                        let dy = dy - 0.5;

                        let texture_mip0_desc = texture.get_level_desc(0).unwrap();
                        let texw = texture_mip0_desc.width  as f32 * scale;
                        let texh = texture_mip0_desc.height as f32 * scale;
                        unsafe { (*device.as_raw()).SetRenderState(D3DRS_AMBIENT, (bri as u32) * 0x01010101) };
                        let _ = device.set_transform(d3d::TS::World, d3d::Matrix { m: [
                            [texw * sx,       0.0, 0.0, 0.0],
                            [      0.0, texh * sy, 0.0, 0.0],
                            [      0.0,       0.0, 1.0, 0.0],
                            [  dx * sx,   dy * sy, 0.0, 1.0],
                        ]});
                        let _ = unsafe { device.set_texture(0, texture) };
                        let _ = unsafe { device.draw_indexed_primitive(PT::TriangleList, 0, 0, 4, 0, 2) };
                    }

                    let _ = device.end_scene();

                    dev::d3d9::screenshot_rt0_for_docs_gen(&device);
                    present_err = device.present(.., .., (), None).err();
                } else {
                    present_err = None;
                }

                if let Some(err) = present_err {
                    match err.kind() {
                        D3DERR::DEVICELOST => {
                            assets = None;
                            #[allow(unused_assignments)] { device = None; } // explicitly release COM device before creating a new one
                            device = Some(unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }.unwrap());
                        },
                        _other => {},
                    }
                }
            },
            _ => {},
        }
    });
}

#[allow(dead_code)]
#[allow(non_snake_case)]
struct Assets {
    A:                  Texture,
    B:                  Texture,
    X:                  Texture,
    Y:                  Texture,
    Back:               Texture,
    Start:              Texture,
    Dpad_Down:          Texture,
    Dpad_Left:          Texture,
    Dpad_Right:         Texture,
    Dpad_Up:            Texture,
    Dpad:               Texture,
    LB:                 Texture,
    RB:                 Texture,
    LT:                 Texture,
    RT:                 Texture,
    Left_Stick_Click:   Texture,
    Left_Stick:         Texture,
    Right_Stick_Click:  Texture,
    Right_Stick:        Texture,
    QuadIB:             IndexBuffer,
    QuadVB:             VertexBuffer,
    VertDecl:           VertexDeclaration,
}

impl Assets {
    pub fn new(device: &Device) -> Self {
        Self {
            A:                  png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_A.png"),
            B:                  png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_B.png"),
            X:                  png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_X.png"),
            Y:                  png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Y.png"),
            Back:               png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Back.png"),
            Start:              png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Start.png"),
            Dpad_Down:          png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Dpad_Down.png"),
            Dpad_Left:          png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Dpad_Left.png"),
            Dpad_Right:         png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Dpad_Right.png"),
            Dpad_Up:            png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Dpad_Up.png"),
            Dpad:               png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Dpad.png"),
            LB:                 png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_LB.png"),
            RB:                 png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_RB.png"),
            LT:                 png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_LT.png"),
            RT:                 png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_RT.png"),
            Left_Stick_Click:   png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Left_Stick_Click.png"),
            Left_Stick:         png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Left_Stick.png"),
            Right_Stick_Click:  png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Right_Stick_Click.png"),
            Right_Stick:        png2tex(device, r"thindx\examples\assets\xelu\Others\Xbox 360\360_Right_Stick.png"),
            QuadIB:             index16(device, &[0, 1, 2, 0, 2, 3]).expect("QuadIB"),
            QuadVB:             vert2vb(device, &[
                Vertex { position: [ 0.5, -0.5, 0.5, 0.0], texcoord: [1.0, 1.0] },
                Vertex { position: [-0.5, -0.5, 0.5, 0.0], texcoord: [0.0, 1.0] },
                Vertex { position: [-0.5,  0.5, 0.5, 0.0], texcoord: [0.0, 0.0] },
                Vertex { position: [ 0.5,  0.5, 0.5, 0.0], texcoord: [1.0, 0.0] },
            ]).expect("QuadVB"),

            VertDecl: device.create_vertex_declaration(&[
                VertexElement { offset:  0, ty: DeclType8::Float4, usage: DeclUsage8::Position, usage_index: 0, .. Default::default() },
                VertexElement { offset: 16, ty: DeclType8::Float2, usage: DeclUsage8::TexCoord, usage_index: 0, .. Default::default() },
                VertexElement::END
            ]).unwrap()
        }
    }
}

fn png2tex(device: &Device, path: &str) -> Texture { // TODO: replace with dev/utility function
    return imp(device, path).unwrap_or_else(|err| fatal!("png2tex(device, {:?}): {}", path, err));

    fn imp(device: &Device, path: &str) -> Result<Texture, io::Error> {
        let cwd = std::env::current_dir()?;
        let exe_path = cwd.join(std::env::args().next().expect("exe_path"));
        let root_dir = exe_path.ancestors().nth(4).ok_or(io::ErrorKind::NotFound)?;
        let mut decoder = png::Decoder::new(File::open(root_dir.join(path))?);
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info()?;
        let mut pngbuf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut pngbuf)?;
        let levels = 1;
        assert_eq!(info.bit_depth, png::BitDepth::Eight);

        let bpp;
        let format;
        match info.color_type {
            png::ColorType::Grayscale => {
                bpp     = 1;
                format  = Format::L8;
            },
            png::ColorType::GrayscaleAlpha => {
                bpp     = 2;
                format  = Format::A8L8;
            },
            png::ColorType::Rgb => {
                bpp    = 3;
                format = Format::R8G8B8; // little endian - B,G,R byte order
                pngbuf.chunks_exact_mut(3).for_each(|w| w.reverse()); // fix RGB => BGR
            },
            png::ColorType::Rgba => {
                bpp    = 4;
                format = Format::A8R8G8B8; // little endian - B,G,R,A byte order
                let mut pending = &mut pngbuf[..];
                while let [r, g, b, a, rest @ ..] = pending {
                    // fix RGBA => BGRA
                    std::mem::swap(r, b);

                    // premultiply alpha
                    *r = ((*r as usize) * (*a as usize) / 255) as u8;
                    *g = ((*g as usize) * (*a as usize) / 255) as u8;
                    *b = ((*b as usize) * (*a as usize) / 255) as u8;

                    pending = rest;
                }
            },
            other => fatal!("unexpected png::ColorType::{:?} for `{}`", other, path),
        };

        let texture = device.create_texture(info.width, info.height, levels, Usage::AutoGenMipMap, format, Pool::Managed, ())?;
        let w = info.width as usize;
        let h = info.height as usize;
        let src_pitch = bpp * w;
        unsafe {
            let lock = texture.lock_rect_unchecked(0, .., Lock::NoOverwrite)?;
            for y in 0..h {
                let src = pngbuf[y * src_pitch as usize..].as_ptr();
                let dst = (lock.pBits as *mut u8).add(y * lock.Pitch as usize);
                std::ptr::copy_nonoverlapping(src, dst.cast(), 4 * w);
            }
            texture.unlock_rect(0)?;
        }
        Ok(texture)
    }
}

fn index16(device: &Device, src: &[u16]) -> Result<IndexBuffer, MethodError> {
    // TODO: improve safety (explicit `Bytes(...)` tuple?) - previously passed src.len() instead of byte count, resulting in an undersized buffer
    let ib = device.create_index_buffer(size_of_val(src).try_into().unwrap(), Usage::None, Format::Index16, Pool::Managed, ())?;
    unsafe { // TODO: replace with safe(r) logic
        let dst = ib.lock_unchecked(0, 0, Lock::None)?;
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.cast(), src.len());
        ib.unlock()?;
    }
    Ok(ib)
}

fn vert2vb(device: &Device, src: &[Vertex]) -> Result<VertexBuffer, MethodError> {
    let vb = device.create_vertex_buffer(size_of_val(src).try_into().unwrap(), Usage::None, FVF::None, Pool::Managed, ())?;
    unsafe { // TODO: replace with safe(r) logic
        let dst = vb.lock_unchecked(0, 0, Lock::None)?;
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.cast(), src.len());
        vb.unlock()?;
    }
    Ok(vb)
}

#[repr(C)] struct Vertex {
    pub position:   [f32; 4],
    pub texcoord:   [f32; 2],
}

impl Vertex {
    pub const STRIDE : u32 = std::mem::size_of::<Self>() as _;
}
