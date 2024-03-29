//! Basic [d3d9::Device] setup with [winit](https://docs.rs/winit/)
#![windows_subsystem = "windows"]

use thindx::SafeHWND;
use thindx::d3d9::*;

use raw_window_handle::*;

use winit::dpi::*;
use winit::event::{Event::*, WindowEvent::*};
use winit::event_loop::*;
use winit::window::*;



fn main() {
    dev::win32::optional_dev_init();
    let event_loop  = EventLoop::new();
    let window      = WindowBuilder::new()
        .with_title("00-clear-winit - thindx example")
        .with_inner_size(Size::Physical(PhysicalSize { width: 800, height: 600 }))
        .with_visible(!dev::d3d9::hide_for_docs_gen())
        .build(&event_loop).unwrap();

    let hwnd = match window.raw_window_handle() {
        RawWindowHandle::Win32(Win32WindowHandle { hwnd, .. }) => hwnd.cast(),
        other => panic!("Expected RawWindowHandle::Win32(...), got {:?} instead", other),
    };
    let hwnd = unsafe { SafeHWND::assert_unbounded(hwnd).unwrap() };

    let mut pp = PresentParameters {
        windowed:               true.into(),
        device_window:          Some(hwnd),
        swap_effect:            SwapEffect::Discard,
        presentation_interval:  Present::IntervalOne,
        .. PresentParameters::zeroed()
    };

    let behavior =
        // Create::DisablePrintScreen | // d3d9ex
        Create::FpuPreserve |
        Create::HardwareVertexProcessing |
        Create::NoWindowChanges;

    let d3d     = unsafe { Direct3D::create(SdkVersion::default()) }.unwrap();
    let device  = unsafe { d3d.create_device(0, DevType::HAL, None, behavior, &mut pp) }.unwrap();

    event_loop.run(move |event, _, control_flow|{
        *control_flow = ControlFlow::Poll;

        match event {
            WindowEvent { event: CloseRequested, window_id } if window_id == window.id() => {
                std::process::exit(0); // Ensure Device outlasts closing HWND!
            },
            MainEventsCleared => {
                device.clear(None, Some(Color::argb(0xFF224466)), None, None).unwrap();
                device.present(.., .., (), None).unwrap(); // TODO: Handle D3DERR::DEVICELOST
                dev::d3d9::screenshot_rt0_for_docs_gen(&device);
            },
            _ => {},
        }
    });
}
