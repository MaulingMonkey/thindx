//! [d3d9] + [xinput] sample
#![windows_subsystem = "windows"]

use thindx::*;
use thindx::d3d9::*;

use bytemuck::*;

use mmrbi::*;

use raw_window_handle::*;

use winapi::shared::d3d9types::*;
use winapi::um::objbase::CoInitialize;

use winit::dpi::*;
use winit::event::{Event::*, WindowEvent::*};
use winit::event_loop::*;
use winit::window::*;

use std::fs::File;
use std::io;
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

    let d3d         = unsafe { Direct3D::create(SdkVersion::default()) }.unwrap();
    let mut device  = unsafe { try_create_device(&d3d, &window) };
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
                    let _ = render(device, assets);
                    dev::d3d9::screenshot_rt0_for_docs_gen(&device);
                    present_err = device.present(.., .., (), None).err();
                } else {
                    present_err = None;
                }

                if let Some(err) = present_err {
                    match err.kind() {
                        D3DERR::DEVICELOST => {
                            assets = None;
                            drop(device.take()); // avoid simultanious devices for window
                            device = unsafe { try_create_device(&d3d, &window) };
                        },
                        _other => {},
                    }
                }
            },
            _ => {},
        }
    });
}

/// ### ⚠️ Safety ⚠️
/// Caller is responsible for ensuring the [`d3d9::Device`] does not outlive the `window`.
unsafe fn try_create_device(d3d: &Direct3D, window: &Window) -> Option<d3d9::Device> {
    let hwnd = match window.raw_window_handle() {
        RawWindowHandle::Win32(Win32Handle { hwnd, .. }) => hwnd.cast(),
        other => panic!("Expected RawWindowHandle::Windows(...), got {:?} instead", other),
    };

    let mut pp = D3DPRESENT_PARAMETERS { // TODO: replace with d3d::PresentParameters
        Windowed:               true.into(),
        hDeviceWindow:          hwnd,
        SwapEffect:             SwapEffect::Discard.into(),
        PresentationInterval:   Present::IntervalOne.into(),
        .. std::mem::zeroed()
    };

    let behavior =
        // Create::DisablePrintScreen | // d3d9ex
        Create::FpuPreserve |
        Create::HardwareVertexProcessing |
        Create::NoWindowChanges;

    Some(d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp).unwrap())
}

fn render(device: &Device, assets: &Assets) -> Result<(), BugRenderErrors> {
    device.clear(None, Some(Color::argb(0xFF000000)), None, None)?;

    device.begin_scene()?;
    device.set_stream_source(0, &assets.QuadVB, 0, Vertex::STRIDE)?;
    device.set_indices(&assets.QuadIB)?;
    device.set_vertex_declaration(&assets.VertDecl)?;
    device.set_material(Material {
        ambient: ColorValue { r: 1.0, g: 1.0, b: 1.0, a: 0.0 },
        ..Default::default()
    })?;
    device.set_render_state_untyped(d3d::RS::Lighting,          true as u32             )?;
    device.set_render_state_untyped(d3d::RS::AlphaBlendEnable,  true as u32             )?;
    device.set_render_state_untyped(d3d::RS::DestBlend,         d3d::Blend::InvSrcAlpha )?;
    device.set_render_state_untyped(d3d::RS::Ambient,           0xFFFFFFFFu32           )?;
    device.set_sampler_state(0, d3d::SampV::MinFilter(d3d::TexF::Linear))?;
    device.set_sampler_state(0, d3d::SampV::MagFilter(d3d::TexF::Linear))?;
    device.set_sampler_state(0, d3d::SampV::MipFilter(d3d::TexF::Linear))?;

    let sx = 2.0 / 800.0;
    let sy = 2.0 / 600.0;

    let user = xinput::User::Zero;
    let state = xinput::get_state(user).ok()
        .filter(|_| !dev::d3d9::hide_for_docs_gen())
        .unwrap_or(xinput::State::default());

    let _ = xinput::set_state(user, xinput::Vibration {
        left_motor_speed:  0x101 * (state.left_trigger  as u16),
        right_motor_speed: 0x101 * (state.right_trigger as u16),
    });

    use xinput::Buttons;
    let asset_dpad;
    if state.buttons.all_held(Buttons::DPadUp | Buttons::DPadLeft) { asset_dpad = &assets.Dpad_Left; }
    else if state.buttons.any_held(Buttons::DPadUp)     { asset_dpad = &assets.Dpad_Up; }
    else if state.buttons.any_held(Buttons::DPadRight)  { asset_dpad = &assets.Dpad_Right; }
    else if state.buttons.any_held(Buttons::DPadDown)   { asset_dpad = &assets.Dpad_Down; }
    else if state.buttons.any_held(Buttons::DPadLeft)   { asset_dpad = &assets.Dpad_Left; }
    else                                                { asset_dpad = &assets.Dpad; }

    // I intentionally apply no deadzone to these values
    let lx = state.left_thumb_x  as f32 * 50.0 / (i16::MAX as f32);
    let ly = state.left_thumb_y  as f32 * 50.0 / (i16::MAX as f32);
    let rx = state.right_thumb_x as f32 * 50.0 / (i16::MAX as f32);
    let ry = state.right_thumb_y as f32 * 50.0 / (i16::MAX as f32);
    let lty = state.left_trigger  as f32 * 50.0 / (u8::MAX as f32);
    let rty = state.right_trigger as f32 * 50.0 / (u8::MAX as f32);

    let dpad = Buttons::DPadDown | Buttons::DPadRight | Buttons::DPadLeft | Buttons::DPadUp;
    let lb      = if state.buttons.any_held(Buttons::LeftShoulder)  { 255 } else { 128 };
    let rb      = if state.buttons.any_held(Buttons::RightShoulder) { 255 } else { 128 };
    let a       = if state.buttons.any_held(Buttons::A)             { 255 } else { 128 };
    let b       = if state.buttons.any_held(Buttons::B)             { 255 } else { 128 };
    let x       = if state.buttons.any_held(Buttons::X)             { 255 } else { 128 };
    let y       = if state.buttons.any_held(Buttons::Y)             { 255 } else { 128 };
    let start   = if state.buttons.any_held(Buttons::Start)         { 255 } else { 128 };
    let back    = if state.buttons.any_held(Buttons::Back)          { 255 } else { 128 };
    let lthumb  = if state.buttons.any_held(Buttons::LeftThumb)     { 255 } else { 128 };
    let rthumb  = if state.buttons.any_held(Buttons::RightThumb)    { 255 } else { 128 };
    let dpad    = if state.buttons.any_held(dpad)                   { 255 } else { 128 };
    let lt      = 128 + state.left_trigger  / 2;
    let rt      = 128 + state.right_trigger / 2;

    for (    dx       ,    dy       , texture,           scale, bri) in [
        (-330.0       , 190.0 - lty , &assets.LT,          1.0, lt),
        ( 330.0       , 190.0 - rty , &assets.RT,          1.0, rt),
        (-220.0       , 230.0       , &assets.LB,          1.0, lb),
        ( 220.0       , 230.0       , &assets.RB,          1.0, rb),

        ( 300.0       ,  30.0 - 60.0, &assets.A,           0.7, a),
        ( 300.0 + 60.0,  30.0       , &assets.B,           0.7, b),
        ( 300.0 - 60.0,  30.0       , &assets.X,           0.7, x),
        ( 300.0       ,  30.0 + 60.0, &assets.Y,           0.7, y),

        ( 100.0       ,  30.0       , &assets.Start,       0.7, start),
        // Guide Button?
        (-100.0       ,  30.0       , &assets.Back,        0.7, back),

        (-300.0 + lx  ,  30.0 + ly  , &assets.Left_Stick,  1.5, lthumb),
        (-150.0       ,-130.0       , asset_dpad,          1.5, dpad),
        ( 150.0 + rx  ,-130.0 + ry  , &assets.Right_Stick, 1.5, rthumb),
    ].iter().copied() {
        // half texel fixups
        let dx = dx + 0.5;
        let dy = dy - 0.5;

        let texture_mip0_desc = texture.get_level_desc(0).unwrap();
        let texw = texture_mip0_desc.width  as f32 * scale;
        let texh = texture_mip0_desc.height as f32 * scale;
        device.set_render_state_untyped(d3d::RS::Ambient, (bri as u32) * 0x01010101)?;
        device.set_transform(d3d::TS::World, d3d::Matrix { m: [
            [texw * sx,       0.0, 0.0, 0.0],
            [      0.0, texh * sy, 0.0, 0.0],
            [      0.0,       0.0, 1.0, 0.0],
            [  dx * sx,   dy * sy, 0.0, 1.0],
        ]})?;
        unsafe { device.set_texture(0, texture) }?;
        unsafe { device.draw_indexed_primitive(PT::TriangleList, 0, 0, 4, 0, 2) }?;
    }

    device.end_scene()?;

    Ok(())
}

struct BugRenderErrors;
impl From<thindx::MethodError> for BugRenderErrors {
    fn from(err: thindx::MethodError) -> Self {
        // report issue immediately upon error conversion (e.g. at `?` operator)
        bugsalot::bug!("rendering error: {}", err);
        Self
    }
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
        macro_rules! xelu { ( $name:expr ) => {
            png2tex(device, concat!(r"thindx\examples\assets\xelu\", $name))
        }}
        Self {
            A:                  xelu!(r"Others\Xbox 360\360_A.png"),
            B:                  xelu!(r"Others\Xbox 360\360_B.png"),
            X:                  xelu!(r"Others\Xbox 360\360_X.png"),
            Y:                  xelu!(r"Others\Xbox 360\360_Y.png"),
            Back:               xelu!(r"Others\Xbox 360\360_Back.png"),
            Start:              xelu!(r"Others\Xbox 360\360_Start.png"),
            Dpad_Down:          xelu!(r"Others\Xbox 360\360_Dpad_Down.png"),
            Dpad_Left:          xelu!(r"Others\Xbox 360\360_Dpad_Left.png"),
            Dpad_Right:         xelu!(r"Others\Xbox 360\360_Dpad_Right.png"),
            Dpad_Up:            xelu!(r"Others\Xbox 360\360_Dpad_Up.png"),
            Dpad:               xelu!(r"Others\Xbox 360\360_Dpad.png"),
            LB:                 xelu!(r"Others\Xbox 360\360_LB.png"),
            RB:                 xelu!(r"Others\Xbox 360\360_RB.png"),
            LT:                 xelu!(r"Others\Xbox 360\360_LT.png"),
            RT:                 xelu!(r"Others\Xbox 360\360_RT.png"),
            Left_Stick_Click:   xelu!(r"Others\Xbox 360\360_Left_Stick_Click.png"),
            Left_Stick:         xelu!(r"Others\Xbox 360\360_Left_Stick.png"),
            Right_Stick_Click:  xelu!(r"Others\Xbox 360\360_Right_Stick_Click.png"),
            Right_Stick:        xelu!(r"Others\Xbox 360\360_Right_Stick.png"),

            QuadIB: device.create_index_buffer_from(
                &[0u16, 1, 2, 0, 2, 3][..], Usage::None, Pool::Managed, (),
            ).expect("QuadIB"),

            QuadVB: device.create_vertex_buffer_from(&[
                Vertex { position: [ 0.5, -0.5, 0.5, 0.0], texcoord: [1.0, 1.0] },
                Vertex { position: [-0.5, -0.5, 0.5, 0.0], texcoord: [0.0, 1.0] },
                Vertex { position: [-0.5,  0.5, 0.5, 0.0], texcoord: [0.0, 0.0] },
                Vertex { position: [ 0.5,  0.5, 0.5, 0.0], texcoord: [1.0, 0.0] },
            ][..], Usage::None, FVF::None, Pool::Managed, ()).expect("QuadVB"),

            VertDecl: device.create_vertex_declaration(Vertex::ELEMENTS).unwrap()
        }
    }
}

fn png2tex(device: &Device, path: &str) -> Texture { // TODO: replace with utility function
    return imp(device, path).unwrap_or_else(|err| fatal!("png2tex(.., {:?}): {}", path, err));

    fn imp(device: &Device, path: &str) -> Result<Texture, io::Error> {
        let cwd = std::env::current_dir()?;
        let exe_path = cwd.join(std::env::args().next().expect("exe_path"));
        let root_dir = exe_path.ancestors().nth(4).ok_or(io::ErrorKind::NotFound)?;
        let mut decoder = png::Decoder::new(File::open(root_dir.join(path))?);
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info()?;
        let mut pngbuf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut pngbuf)?;
        assert_eq!(info.bit_depth, png::BitDepth::Eight);

        let bpp;
        let format;
        match info.color_type {
            png::ColorType::Grayscale => {
                bpp     = 1;
                format  = FixedTextureFormat::L8;
            },
            png::ColorType::GrayscaleAlpha => {
                bpp     = 2;
                format  = FixedTextureFormat::A8L8;
            },
            png::ColorType::Rgb => {
                bpp    = 3;
                format = FixedTextureFormat::R8G8B8; // little endian - B,G,R byte order
                pngbuf.chunks_exact_mut(3).for_each(|w| w.reverse()); // fix RGB => BGR
            },
            png::ColorType::Rgba => {
                bpp    = 4;
                format = FixedTextureFormat::A8R8G8B8; // little endian - B,G,R,A byte order
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

        let mips = [
            d3d9::TextureMipRef { data: &pngbuf[..], stride: (bpp * info.width) as usize },
        ];

        Ok(device.create_texture_from(
            info.width, info.height, &mips,
            Usage::AutoGenMipMap, format, Pool::Managed, ()
        )?)
    }
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)] struct Vertex {
    pub position:   [f32; 4],
    pub texcoord:   [f32; 2],
}

use VertexElement as VE;
impl Vertex {
    pub const STRIDE : u32 = std::mem::size_of::<Self>() as _;
    pub const ELEMENTS : &'static [VertexElement] = &[
        VE::new(0,  0, DeclType8::Float4, DeclMethod8::Default, DeclUsage8::Position, 0),
        VE::new(0, 16, DeclType8::Float2, DeclMethod8::Default, DeclUsage8::TexCoord, 0),
        VE::END
    ];
}
