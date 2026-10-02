use core::num::NonZeroU32;
use std::ffi::CString;
use std::sync::Arc;

use glutin::context::{GlContext as _, NotCurrentGlContext as _};
use glutin::display::{GetGlDisplay as _, GlDisplay as _};
use glutin::prelude::GlSurface as _;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::wayland::WindowAttributesExtWayland as _;
use winit::raw_window_handle::HasWindowHandle as _;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::app::GlyphflickApp;
use crate::clipboard::WlCopyClipboard;
use crate::perf::Timing;

const WINDOW_WIDTH: f64 = 560.0;
const WINDOW_HEIGHT: f64 = 440.0;
const CLEAR_COLOR: [f32; 4] = [0.03, 0.03, 0.03, 1.0];

pub fn run(timing: Timing) -> Result<(), winit::error::EventLoopError> {
    let event_loop_start = timing.stamp();
    let event_loop = EventLoop::new()?;
    timing.report_event_loop_init(event_loop_start);

    let mut runtime = Runtime::new(timing);
    event_loop.run_app(&mut runtime)
}

struct Runtime {
    timing: Timing,
    #[cfg(feature = "timing")]
    first_swap_pending: bool,
    gl_window: Option<GlutinWindowContext>,
    egui_glow: Option<egui_glow::EguiGlow>,
    app: Option<GlyphflickApp<WlCopyClipboard>>,
}

impl Runtime {
    const fn new(timing: Timing) -> Self {
        Self {
            timing,
            #[cfg(feature = "timing")]
            first_swap_pending: true,
            gl_window: None,
            egui_glow: None,
            app: None,
        }
    }
}

impl ApplicationHandler for Runtime {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gl_window.is_some() {
            return;
        }

        event_loop.set_control_flow(ControlFlow::Wait);
        self.timing.mark_resumed();

        let runtime_start = self.timing.stamp();
        let (gl_window, gl) = create_display(event_loop, self.timing);
        self.timing.report_runtime_init(runtime_start);

        let egui_start = self.timing.stamp();
        let egui_glow = egui_glow::EguiGlow::new(event_loop, Arc::new(gl), None, None, false);
        self.timing.report_egui_init(egui_start);

        let app = GlyphflickApp::new(&egui_glow.egui_ctx, WlCopyClipboard, self.timing);

        gl_window.window().request_redraw();

        self.gl_window = Some(gl_window);
        self.egui_glow = Some(egui_glow);
        self.app = Some(app);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(gl_window) = self.gl_window.as_mut() else {
            return;
        };

        if window_id != gl_window.window().id() {
            return;
        }

        if matches!(&event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
            event_loop.exit();
            return;
        }

        if matches!(&event, WindowEvent::RedrawRequested) {
            let egui_glow = self.egui_glow.as_mut().expect("egui runtime missing");
            let app = self.app.as_mut().expect("glyphflick app missing");

            #[cfg(feature = "timing")]
            let first_egui_start = self.first_swap_pending.then(|| self.timing.stamp());

            egui_glow.run(gl_window.window(), |ui| app.ui(ui));

            #[cfg(feature = "timing")]
            if let Some(start) = first_egui_start {
                self.timing.report_first_egui_run(start);
            }

            if app.exit_requested() {
                event_loop.exit();
                return;
            }

            #[cfg(feature = "timing")]
            let first_paint_start = self.first_swap_pending.then(|| self.timing.stamp());

            let screen_size: [u32; 2] = gl_window.window().inner_size().into();
            egui_glow.painter.clear(screen_size, CLEAR_COLOR);
            egui_glow.paint(gl_window.window());

            #[cfg(feature = "timing")]
            if let Some(start) = first_paint_start {
                self.timing.report_first_gl_paint(start);
            }

            #[cfg(feature = "timing")]
            let first_swap_start = self.first_swap_pending.then(|| self.timing.stamp());

            if let Err(error) = gl_window.swap_buffers() {
                eprintln!("glyphflick: buffer swap failed: {error}");
                event_loop.exit();
                return;
            }

            #[cfg(feature = "timing")]
            if self.first_swap_pending {
                if let Some(start) = first_swap_start {
                    self.timing.report_first_swap_call(start);
                }
                self.first_swap_pending = false;
                self.timing.mark_first_swap();
                if self.timing.exit_after_first_swap() {
                    event_loop.exit();
                    return;
                }
            }
            return;
        }

        if let WindowEvent::Resized(physical_size) = &event {
            gl_window.resize(*physical_size);
        }

        let response = self
            .egui_glow
            .as_mut()
            .expect("egui runtime missing")
            .on_window_event(gl_window.window(), &event);

        if response.repaint {
            gl_window.window().request_redraw();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(egui_glow) = self.egui_glow.as_mut() {
            egui_glow.destroy();
        }
    }
}

struct GlutinWindowContext {
    window: Window,
    gl_context: glutin::context::PossiblyCurrentContext,
    _gl_display: glutin::display::Display,
    gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
}

impl GlutinWindowContext {
    fn new(event_loop: &ActiveEventLoop, timing: Timing) -> Self {
        let window_attributes = window_attributes();

        let config_template = glutin::config::ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_depth_size(0)
            .with_stencil_size(0)
            .with_transparency(false);

        let display_start = timing.stamp();
        let (mut window, gl_config) = glutin_winit::DisplayBuilder::new()
            .with_preference(glutin_winit::ApiPreference::PreferEgl)
            .with_window_attributes(Some(window_attributes.clone()))
            .build(event_loop, config_template, |mut configs| {
                configs
                    .next()
                    .expect("no EGL configuration available for Glyphflick")
            })
            .expect("failed to create EGL configuration");
        timing.report_display_build(display_start);

        let gl_display = gl_config.display();

        let raw_window_handle = window.as_ref().map(|window| {
            window
                .window_handle()
                .expect("failed to get Wayland window handle")
                .as_raw()
        });

        let context_attributes = if timing.force_gles() {
            glutin::context::ContextAttributesBuilder::new()
                .with_context_api(glutin::context::ContextApi::Gles(None))
                .build(raw_window_handle)
        } else {
            glutin::context::ContextAttributesBuilder::new().build(raw_window_handle)
        };
        let fallback_context_attributes = glutin::context::ContextAttributesBuilder::new()
            .with_context_api(glutin::context::ContextApi::Gles(None))
            .build(raw_window_handle);

        let context_start = timing.stamp();
        let not_current_gl_context = unsafe {
            if timing.force_gles() {
                gl_display
                    .create_context(&gl_config, &context_attributes)
                    .expect("failed to create forced OpenGL ES context")
            } else {
                gl_display
                    .create_context(&gl_config, &context_attributes)
                    .or_else(|_| {
                        gl_display.create_context(&gl_config, &fallback_context_attributes)
                    })
                    .expect("failed to create OpenGL or OpenGL ES context")
            }
        };
        timing.report_context_create(context_start);

        let window = window.take().unwrap_or_else(|| {
            glutin_winit::finalize_window(event_loop, window_attributes, &gl_config)
                .expect("failed to create Wayland window")
        });

        let (width, height): (u32, u32) = window.inner_size().into();
        let width = NonZeroU32::new(width).unwrap_or(NonZeroU32::MIN);
        let height = NonZeroU32::new(height).unwrap_or(NonZeroU32::MIN);

        let surface_attributes =
            glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
                .build(
                    window
                        .window_handle()
                        .expect("failed to get Wayland window handle")
                        .as_raw(),
                    width,
                    height,
                );

        let surface_start = timing.stamp();
        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surface_attributes)
                .expect("failed to create EGL window surface")
        };
        timing.report_surface_create(surface_start);

        let current_start = timing.stamp();
        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("failed to make OpenGL context current");
        timing.report_make_current(current_start);
        timing.report_context_api(gl_context.context_api());

        let swap_interval_disabled = gl_surface
            .set_swap_interval(&gl_context, glutin::surface::SwapInterval::DontWait)
            .is_ok();
        timing.report_swap_interval(swap_interval_disabled);

        Self {
            window,
            gl_context,
            _gl_display: gl_display,
            gl_surface,
        }
    }

    #[inline]
    fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&self, physical_size: PhysicalSize<u32>) {
        let Some(width) = NonZeroU32::new(physical_size.width) else {
            return;
        };
        let Some(height) = NonZeroU32::new(physical_size.height) else {
            return;
        };

        self.gl_surface.resize(&self.gl_context, width, height);
    }

    #[inline]
    fn swap_buffers(&self) -> glutin::error::Result<()> {
        self.gl_surface.swap_buffers(&self.gl_context)
    }

    #[inline]
    fn get_proc_address(&self, address: &std::ffi::CStr) -> *const std::ffi::c_void {
        self._gl_display.get_proc_address(address)
    }
}

fn window_attributes() -> WindowAttributes {
    let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    Window::default_attributes()
        .with_title("Glyphflick")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_max_inner_size(size)
        .with_resizable(false)
        .with_decorations(false)
        .with_name("glyphflick", "glyphflick")
}

fn create_display(
    event_loop: &ActiveEventLoop,
    timing: Timing,
) -> (GlutinWindowContext, egui_glow::glow::Context) {
    let gl_window = GlutinWindowContext::new(event_loop, timing);
    let loader_start = timing.stamp();
    let gl = unsafe {
        egui_glow::glow::Context::from_loader_function(|symbol| {
            let symbol =
                CString::new(symbol).expect("OpenGL procedure name unexpectedly contained NUL");
            gl_window.get_proc_address(&symbol)
        })
    };
    timing.report_gl_loader(loader_start);

    (gl_window, gl)
}
