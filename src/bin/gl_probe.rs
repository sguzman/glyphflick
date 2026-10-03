#[path = "../app.rs"]
mod app;
#[path = "../clipboard.rs"]
mod clipboard;
#[path = "../corpus.rs"]
mod corpus;
#[path = "../fonts.rs"]
mod fonts;
#[path = "../navigation.rs"]
mod navigation;
#[allow(dead_code)]
#[path = "../perf.rs"]
mod perf;
#[path = "../runtime_gl.rs"]
mod runtime_gl;
#[path = "../search.rs"]
mod search;

use perf::Timing;

fn main() -> Result<(), winit::error::EventLoopError> {
    runtime_gl::run(Timing::default())
}
