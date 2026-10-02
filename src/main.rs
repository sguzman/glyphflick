mod app;
mod clipboard;
mod corpus;
mod navigation;
mod perf;
mod runtime;
mod search;

use perf::Timing;

fn main() -> Result<(), winit::error::EventLoopError> {
    runtime::run(Timing::default())
}
