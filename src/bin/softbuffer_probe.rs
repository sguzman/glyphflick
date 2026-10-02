use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::platform::wayland::WindowAttributesExtWayland as _;
use winit::window::{Window, WindowId};

const WINDOW_WIDTH: f64 = 560.0;
const WINDOW_HEIGHT: f64 = 440.0;
const CLEAR_PIXEL: u32 = 0x0008_0808;

fn main() -> Result<(), winit::error::EventLoopError> {
    let process_start = Instant::now();

    let event_loop_start = Instant::now();
    let event_loop = EventLoop::new()?;
    eprintln!(
        "glyphflick softbuffer timing event_loop_init_us={}",
        event_loop_start.elapsed().as_micros()
    );

    let context_start = Instant::now();
    let context = Context::new(event_loop.owned_display_handle())
        .expect("failed to create softbuffer Wayland context");
    eprintln!(
        "glyphflick softbuffer timing context_init_us={}",
        context_start.elapsed().as_micros()
    );

    let mut app = ProbeApp {
        process_start,
        context,
        window: None,
        surface: None,
    };
    event_loop.run_app(&mut app)
}

struct ProbeApp {
    process_start: Instant,
    context: Context<OwnedDisplayHandle>,
    window: Option<Rc<Window>>,
    surface: Option<Surface<OwnedDisplayHandle, Rc<Window>>>,
}

impl ApplicationHandler for ProbeApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        event_loop.set_control_flow(ControlFlow::Wait);
        eprintln!(
            "glyphflick softbuffer timing startup_to_resumed_us={}",
            self.process_start.elapsed().as_micros()
        );

        let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let window_start = Instant::now();
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Glyphflick software presenter probe")
                        .with_inner_size(size)
                        .with_min_inner_size(size)
                        .with_max_inner_size(size)
                        .with_resizable(false)
                        .with_decorations(false)
                        .with_name("glyphflick-softbuffer-probe", "glyphflick-softbuffer-probe"),
                )
                .expect("failed to create Wayland probe window"),
        );
        eprintln!(
            "glyphflick softbuffer timing window_create_us={}",
            window_start.elapsed().as_micros()
        );

        let surface_start = Instant::now();
        let surface = Surface::new(&self.context, Rc::clone(&window))
            .expect("failed to create softbuffer surface");
        eprintln!(
            "glyphflick softbuffer timing surface_create_us={}",
            surface_start.elapsed().as_micros()
        );

        window.request_redraw();
        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let surface = self.surface.as_mut().expect("softbuffer surface missing");
                let size = window.inner_size();
                let width = NonZeroU32::new(size.width.max(1)).expect("width is nonzero");
                let height = NonZeroU32::new(size.height.max(1)).expect("height is nonzero");

                let resize_start = Instant::now();
                surface
                    .resize(width, height)
                    .expect("failed to resize softbuffer surface");
                eprintln!(
                    "glyphflick softbuffer timing resize_us={}",
                    resize_start.elapsed().as_micros()
                );

                let acquire_start = Instant::now();
                let mut buffer = surface
                    .buffer_mut()
                    .expect("failed to acquire softbuffer back buffer");
                eprintln!(
                    "glyphflick softbuffer timing buffer_acquire_us={}",
                    acquire_start.elapsed().as_micros()
                );

                let fill_start = Instant::now();
                buffer.fill(CLEAR_PIXEL);
                eprintln!(
                    "glyphflick softbuffer timing buffer_fill_us={}",
                    fill_start.elapsed().as_micros()
                );

                let present_start = Instant::now();
                buffer.present().expect("failed to present softbuffer frame");
                eprintln!(
                    "glyphflick softbuffer timing present_call_us={}",
                    present_start.elapsed().as_micros()
                );
                eprintln!(
                    "glyphflick softbuffer timing startup_to_first_present_us={}",
                    self.process_start.elapsed().as_micros()
                );
                event_loop.exit();
            }
            _ => {}
        }
    }
}
