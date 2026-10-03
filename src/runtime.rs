use std::collections::HashMap;
use std::num::NonZeroU32;
use std::rc::Rc;

use egui::epaint::Primitive;
use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::platform::wayland::WindowAttributesExtWayland as _;
use winit::window::{Window, WindowId};

use crate::app::GlyphflickApp;
use crate::clipboard::WlCopyClipboard;
use crate::perf::Timing;

const WINDOW_WIDTH: f64 = 560.0;
const WINDOW_HEIGHT: f64 = 440.0;
const CLEAR_PIXEL: u32 = 0x0008_0808;

pub fn run(timing: Timing) -> Result<(), winit::error::EventLoopError> {
    let event_loop_start = timing.stamp();
    let event_loop = EventLoop::new()?;
    timing.report_event_loop_init(event_loop_start);

    let context_start = timing.stamp();
    let context = Context::new(event_loop.owned_display_handle())
        .expect("failed to create softbuffer Wayland context");
    timing.report_context_init(context_start);

    let mut runtime = Runtime::new(timing, context);
    event_loop.run_app(&mut runtime)
}

struct Runtime {
    timing: Timing,
    context: Context<OwnedDisplayHandle>,
    window: Option<Rc<Window>>,
    surface: Option<Surface<OwnedDisplayHandle, Rc<Window>>>,
    egui_ctx: Option<egui::Context>,
    egui_winit: Option<egui_winit::State>,
    app: Option<GlyphflickApp<WlCopyClipboard>>,
    textures: TextureStore,
    first_present: bool,
}

impl Runtime {
    fn new(timing: Timing, context: Context<OwnedDisplayHandle>) -> Self {
        Self {
            timing,
            context,
            window: None,
            surface: None,
            egui_ctx: None,
            egui_winit: None,
            app: None,
            textures: TextureStore::default(),
            first_present: true,
        }
    }
}

impl ApplicationHandler for Runtime {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        event_loop.set_control_flow(ControlFlow::Wait);
        self.timing.mark_resumed();

        let surface_start = self.timing.stamp();
        let size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Glyphflick")
                        .with_inner_size(size)
                        .with_min_inner_size(size)
                        .with_max_inner_size(size)
                        .with_resizable(false)
                        .with_decorations(false)
                        .with_name("glyphflick", "glyphflick"),
                )
                .expect("failed to create Glyphflick Wayland window"),
        );
        let surface = Surface::new(&self.context, Rc::clone(&window))
            .expect("failed to create Glyphflick software surface");
        self.timing.report_window_surface_init(surface_start);

        let ctx = egui::Context::default();

        let egui_winit_start = self.timing.stamp();
        let egui_winit = egui_winit::State::new(
            ctx.clone(),
            egui::ViewportId::ROOT,
            event_loop,
            None,
            event_loop.system_theme(),
            None,
        );
        self.timing.report_egui_winit_init(egui_winit_start);

        let app_start = self.timing.stamp();
        let app = GlyphflickApp::new(&ctx, WlCopyClipboard, self.timing);
        self.timing.report_egui_app_init(app_start);

        window.request_redraw();
        self.window = Some(window);
        self.surface = Some(surface);
        self.egui_ctx = Some(ctx);
        self.egui_winit = Some(egui_winit);
        self.app = Some(app);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref().cloned() else {
            return;
        };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => event_loop.exit(),
            WindowEvent::RedrawRequested => self.draw_frame(event_loop),
            event => {
                let response = self
                    .egui_winit
                    .as_mut()
                    .expect("egui-winit state missing")
                    .on_window_event(&window, &event);

                if response.repaint || matches!(event, WindowEvent::Resized(_)) {
                    window.request_redraw();
                }
            }
        }
    }
}

impl Runtime {
    fn draw_frame(&mut self, event_loop: &ActiveEventLoop) {
        let window = self.window.as_ref().cloned().expect("window missing");
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let raw_input = self
            .egui_winit
            .as_mut()
            .expect("egui-winit state missing")
            .take_egui_input(&window);

        let ctx = self.egui_ctx.as_ref().expect("egui context missing");
        let app = self.app.as_mut().expect("glyphflick app missing");

        let egui_start = self.timing.stamp();
        let egui::FullOutput {
            platform_output,
            mut textures_delta,
            shapes,
            pixels_per_point,
            ..
        } = ctx.run_ui(raw_input, |ui| app.ui(ui));
        self.timing.report_egui_run(egui_start);

        self.egui_winit
            .as_mut()
            .expect("egui-winit state missing")
            .handle_platform_output(&window, platform_output);

        let followup_redraw = app.take_followup_redraw();
        if app.exit_requested() {
            event_loop.exit();
            return;
        }

        let tessellate_start = self.timing.stamp();
        let primitives = ctx.tessellate(shapes, pixels_per_point);
        self.timing.report_tessellate(tessellate_start);

        let texture_start = self.timing.stamp();
        self.textures.apply(&mut textures_delta);
        self.timing.report_texture_update(texture_start);

        let surface = self.surface.as_mut().expect("surface missing");
        surface
            .resize(
                NonZeroU32::new(width).expect("nonzero width"),
                NonZeroU32::new(height).expect("nonzero height"),
            )
            .expect("failed to resize Glyphflick software surface");

        let mut buffer = surface
            .buffer_mut()
            .expect("failed to acquire Glyphflick software back buffer");
        buffer.fill(CLEAR_PIXEL);

        let raster_start = self.timing.stamp();
        rasterize(
            &mut buffer,
            width as usize,
            height as usize,
            pixels_per_point,
            &primitives,
            &self.textures,
        );
        self.timing.report_software_raster(raster_start);

        let present_start = self.timing.stamp();
        buffer
            .present()
            .expect("failed to present Glyphflick software frame");
        self.timing.report_present_call(present_start);

        for id in textures_delta.free.drain() {
            self.textures.images.remove(&id);
        }

        if self.first_present {
            self.first_present = false;
            self.timing.mark_first_present();

            #[cfg(feature = "timing")]
            if self.timing.exit_after_first_present() {
                event_loop.exit();
                return;
            }
        }

        if followup_redraw {
            window.request_redraw();
        }
    }
}

#[derive(Default)]
struct TextureStore {
    images: HashMap<egui::TextureId, Texture>,
}

struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<egui::Color32>,
}

impl TextureStore {
    fn apply(&mut self, delta: &mut egui::TexturesDelta) {
        for (id, image_deltas) in delta.set.drain() {
            for image_delta in image_deltas {
                let egui::ImageData::Color(image) = &image_delta.image;
                let [patch_width, patch_height] = image.size;

                if let Some([x, y]) = image_delta.pos {
                    let texture = self
                        .images
                        .get_mut(&id)
                        .expect("partial texture update before full image");
                    assert!(x + patch_width <= texture.width);
                    assert!(y + patch_height <= texture.height);

                    for row in 0..patch_height {
                        let source = &image.pixels[row * patch_width..(row + 1) * patch_width];
                        let start = (y + row) * texture.width + x;
                        texture.pixels[start..start + patch_width].copy_from_slice(source);
                    }
                } else {
                    self.images.insert(
                        id,
                        Texture {
                            width: patch_width,
                            height: patch_height,
                            pixels: image.pixels.clone(),
                        },
                    );
                }
            }
        }
    }
}

fn rasterize(
    target: &mut [u32],
    width: usize,
    height: usize,
    pixels_per_point: f32,
    primitives: &[egui::ClippedPrimitive],
    textures: &TextureStore,
) {
    for clipped in primitives {
        let Primitive::Mesh(mesh) = &clipped.primitive else {
            continue;
        };
        let Some(texture) = textures.images.get(&mesh.texture_id) else {
            continue;
        };

        let clip = pixel_clip(clipped.clip_rect, width, height, pixels_per_point);
        if clip.0 >= clip.2 || clip.1 >= clip.3 {
            continue;
        }

        for triangle in mesh.indices.as_chunks::<3>().0 {
            let a = mesh.vertices[triangle[0] as usize];
            let b = mesh.vertices[triangle[1] as usize];
            let c = mesh.vertices[triangle[2] as usize];
            raster_triangle(target, width, clip, pixels_per_point, texture, a, b, c);
        }
    }
}

fn pixel_clip(
    rect: egui::Rect,
    width: usize,
    height: usize,
    pixels_per_point: f32,
) -> (i32, i32, i32, i32) {
    let min_x = (rect.min.x * pixels_per_point).round() as i32;
    let min_y = (rect.min.y * pixels_per_point).round() as i32;
    let max_x = (rect.max.x * pixels_per_point).round() as i32;
    let max_y = (rect.max.y * pixels_per_point).round() as i32;

    (
        min_x.clamp(0, width as i32),
        min_y.clamp(0, height as i32),
        max_x.clamp(0, width as i32),
        max_y.clamp(0, height as i32),
    )
}

#[allow(clippy::too_many_arguments)]
fn raster_triangle(
    target: &mut [u32],
    target_width: usize,
    clip: (i32, i32, i32, i32),
    pixels_per_point: f32,
    texture: &Texture,
    a: egui::epaint::Vertex,
    b: egui::epaint::Vertex,
    c: egui::epaint::Vertex,
) {
    let p0 = [a.pos.x * pixels_per_point, a.pos.y * pixels_per_point];
    let p1 = [b.pos.x * pixels_per_point, b.pos.y * pixels_per_point];
    let p2 = [c.pos.x * pixels_per_point, c.pos.y * pixels_per_point];
    let area = edge(p0, p1, p2);
    if area.abs() < f32::EPSILON {
        return;
    }

    let min_x = p0[0].min(p1[0]).min(p2[0]).floor() as i32;
    let min_y = p0[1].min(p1[1]).min(p2[1]).floor() as i32;
    let max_x = p0[0].max(p1[0]).max(p2[0]).ceil() as i32;
    let max_y = p0[1].max(p1[1]).max(p2[1]).ceil() as i32;

    let min_x = min_x.max(clip.0);
    let min_y = min_y.max(clip.1);
    let max_x = max_x.min(clip.2);
    let max_y = max_y.min(clip.3);
    if min_x >= max_x || min_y >= max_y {
        return;
    }

    let inv_area = area.recip();
    let ca = a.color.to_array();
    let cb = b.color.to_array();
    let cc = c.color.to_array();

    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = [x as f32 + 0.5, y as f32 + 0.5];
            let w0 = edge(p1, p2, p) * inv_area;
            let w1 = edge(p2, p0, p) * inv_area;
            let w2 = 1.0 - w0 - w1;
            if w0 < -0.0001 || w1 < -0.0001 || w2 < -0.0001 {
                continue;
            }

            let uv_x = w0 * a.uv.x + w1 * b.uv.x + w2 * c.uv.x;
            let uv_y = w0 * a.uv.y + w1 * b.uv.y + w2 * c.uv.y;
            let texel = sample_nearest(texture, uv_x, uv_y).to_array();

            let vertex = [
                lerp_channel(ca[0], cb[0], cc[0], w0, w1, w2),
                lerp_channel(ca[1], cb[1], cc[1], w0, w1, w2),
                lerp_channel(ca[2], cb[2], cc[2], w0, w1, w2),
                lerp_channel(ca[3], cb[3], cc[3], w0, w1, w2),
            ];

            let src = [
                mul_u8(texel[0], vertex[0]),
                mul_u8(texel[1], vertex[1]),
                mul_u8(texel[2], vertex[2]),
                mul_u8(texel[3], vertex[3]),
            ];
            let index = y as usize * target_width + x as usize;
            target[index] = blend_over_rgb(target[index], src);
        }
    }
}

#[inline(always)]
fn edge(a: [f32; 2], b: [f32; 2], p: [f32; 2]) -> f32 {
    (p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
}

#[inline(always)]
fn lerp_channel(a: u8, b: u8, c: u8, w0: f32, w1: f32, w2: f32) -> u8 {
    (w0 * a as f32 + w1 * b as f32 + w2 * c as f32)
        .round()
        .clamp(0.0, 255.0) as u8
}

#[inline(always)]
fn mul_u8(a: u8, b: u8) -> u8 {
    ((a as u16 * b as u16 + 127) / 255) as u8
}

#[inline(always)]
fn sample_nearest(texture: &Texture, u: f32, v: f32) -> egui::Color32 {
    let x = (u.clamp(0.0, 1.0) * texture.width as f32)
        .floor()
        .min((texture.width - 1) as f32) as usize;
    let y = (v.clamp(0.0, 1.0) * texture.height as f32)
        .floor()
        .min((texture.height - 1) as f32) as usize;
    texture.pixels[y * texture.width + x]
}

#[inline(always)]
fn blend_over_rgb(dst: u32, src: [u8; 4]) -> u32 {
    let inv_alpha = 255_u32 - src[3] as u32;
    let dst_r = (dst >> 16) & 0xff;
    let dst_g = (dst >> 8) & 0xff;
    let dst_b = dst & 0xff;

    let out_r = (src[0] as u32 + (dst_r * inv_alpha + 127) / 255).min(255);
    let out_g = (src[1] as u32 + (dst_g * inv_alpha + 127) / 255).min(255);
    let out_b = (src[2] as u32 + (dst_b * inv_alpha + 127) / 255).min(255);

    (out_r << 16) | (out_g << 8) | out_b
}
