mod app;
mod clipboard;
mod corpus;
mod fonts;
mod navigation;
#[cfg_attr(feature = "legacy-gl", allow(dead_code))]
mod perf;
mod runtime;
mod search;

use perf::Timing;

fn main() -> Result<(), winit::error::EventLoopError> {
    runtime::run(Timing::default())
}
