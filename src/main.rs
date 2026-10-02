mod app;
mod clipboard;
mod corpus;
mod navigation;
mod perf;
mod search;

use app::GlyphflickApp;
use clipboard::WlCopyClipboard;
use eframe::egui;
use perf::Timing;

const WINDOW_WIDTH: f32 = 560.0;
const WINDOW_HEIGHT: f32 = 440.0;

fn main() -> eframe::Result {
    let timing = Timing::default();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("glyphflick")
            .with_title("Glyphflick")
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_max_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_decorations(false)
            .with_resizable(false)
            .with_icon(egui::IconData::default()),
        renderer: eframe::Renderer::Glow,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        dithering: false,
        persist_window: false,
        ..Default::default()
    };

    eframe::run_native(
        "Glyphflick",
        native_options,
        Box::new(move |cc| {
            Ok(Box::new(GlyphflickApp::new(
                cc,
                WlCopyClipboard,
                timing,
            )))
        }),
    )
}
