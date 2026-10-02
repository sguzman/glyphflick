#[cfg(feature = "timing")]
use std::time::Instant;

#[cfg(feature = "timing")]
pub type Stamp = Instant;

#[cfg(not(feature = "timing"))]
#[derive(Clone, Copy)]
pub struct Stamp;

#[derive(Clone, Copy)]
pub struct Timing {
    #[cfg(feature = "timing")]
    process_start: Instant,
    #[cfg(feature = "timing")]
    exit_after_first_swap: bool,
    #[cfg(feature = "timing")]
    force_gles: bool,
    #[cfg(feature = "timing")]
    alpha_zero: bool,
}

impl Default for Timing {
    #[inline(always)]
    fn default() -> Self {
        Self {
            #[cfg(feature = "timing")]
            process_start: Instant::now(),
            #[cfg(feature = "timing")]
            exit_after_first_swap: std::env::var_os("GLYPHFLICK_EXIT_AFTER_FIRST_SWAP").is_some(),
            #[cfg(feature = "timing")]
            force_gles: std::env::var_os("GLYPHFLICK_FORCE_GLES").is_some(),
            #[cfg(feature = "timing")]
            alpha_zero: std::env::var_os("GLYPHFLICK_ALPHA_ZERO").is_some(),
        }
    }
}

impl Timing {
    #[inline(always)]
    pub fn stamp(self) -> Stamp {
        #[cfg(feature = "timing")]
        {
            Instant::now()
        }

        #[cfg(not(feature = "timing"))]
        {
            Stamp
        }
    }

    #[inline(always)]
    pub fn report_event_loop_init(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing event_loop_init_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }

    #[inline(always)]
    pub fn mark_resumed(self) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing startup_to_resumed_us={}",
            self.process_start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = self;
    }

    #[inline(always)]
    pub fn report_runtime_init(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing wayland_egl_gl_init_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }

    #[inline(always)]
    pub fn report_egui_init(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing egui_runtime_init_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }

    #[inline(always)]
    pub fn report_swap_interval(self, disabled: bool) {
        #[cfg(feature = "timing")]
        eprintln!("glyphflick timing swap_interval_dont_wait={disabled}");

        #[cfg(not(feature = "timing"))]
        let _ = (self, disabled);
    }

    #[inline(always)]
    pub fn report_font_init(self, start: Stamp, system_emoji_mapped: bool) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing font_init_us={} system_emoji_mapped={system_emoji_mapped}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start, system_emoji_mapped);
    }

    #[inline(always)]
    pub fn report_corpus(self, start: Stamp, glyph_count: usize) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing corpus_init_us={} glyphs={glyph_count}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start, glyph_count);
    }

    #[inline(always)]
    pub fn mark_first_ui(self) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing startup_to_first_ui_us={}",
            self.process_start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = self;
    }

    #[cfg(feature = "timing")]
    #[inline(always)]
    pub const fn exit_after_first_swap(self) -> bool {
        self.exit_after_first_swap
    }

    #[inline(always)]
    pub const fn force_gles(self) -> bool {
        #[cfg(feature = "timing")]
        {
            self.force_gles
        }

        #[cfg(not(feature = "timing"))]
        {
            false
        }
    }

    #[inline(always)]
    pub const fn alpha_zero(self) -> bool {
        #[cfg(feature = "timing")]
        {
            self.alpha_zero
        }

        #[cfg(not(feature = "timing"))]
        {
            false
        }
    }

    #[inline(always)]
    pub fn report_gl_config(
        self,
        alpha: u8,
        depth: u8,
        stencil: u8,
        samples: u8,
        hardware: bool,
        api: glutin::config::Api,
    ) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing gl_config_alpha={alpha} depth={depth} stencil={stencil} samples={samples} hardware={hardware} api={api:?}"
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, alpha, depth, stencil, samples, hardware, api);
    }

    #[inline(always)]
    pub fn report_display_build(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing egl_display_config_window_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }

    #[inline(always)]
    pub fn report_context_create(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing gl_context_create_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }

    #[inline(always)]
    pub fn report_surface_create(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing egl_surface_create_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }

    #[inline(always)]
    pub fn report_make_current(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing gl_make_current_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }

    #[inline(always)]
    pub fn report_gl_loader(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing glow_loader_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }

    #[inline(always)]
    pub fn report_context_api(self, api: glutin::context::ContextApi) {
        #[cfg(feature = "timing")]
        eprintln!("glyphflick timing context_api={api:?}");

        #[cfg(not(feature = "timing"))]
        let _ = (self, api);
    }

    #[cfg(feature = "timing")]
    #[inline(always)]
    pub fn report_first_egui_run(self, start: Stamp) {
        eprintln!(
            "glyphflick timing first_egui_run_us={}",
            start.elapsed().as_micros()
        );
    }

    #[cfg(feature = "timing")]
    #[inline(always)]
    pub fn report_first_gl_paint(self, start: Stamp) {
        eprintln!(
            "glyphflick timing first_gl_paint_us={}",
            start.elapsed().as_micros()
        );
    }

    #[cfg(feature = "timing")]
    #[inline(always)]
    pub fn report_first_swap_call(self, start: Stamp) {
        eprintln!(
            "glyphflick timing first_swap_call_us={}",
            start.elapsed().as_micros()
        );
    }

    #[cfg(feature = "timing")]
    #[inline(always)]
    pub fn mark_first_swap(self) {
        eprintln!(
            "glyphflick timing startup_to_first_swap_complete_us={}",
            self.process_start.elapsed().as_micros()
        );
    }

    #[inline(always)]
    pub fn report_search(self, start: Stamp, result_count: usize) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing search_us={} results={result_count}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start, result_count);
    }

    #[inline(always)]
    pub fn report_clipboard(self, start: Stamp) {
        #[cfg(feature = "timing")]
        eprintln!(
            "glyphflick timing clipboard_establish_us={}",
            start.elapsed().as_micros()
        );

        #[cfg(not(feature = "timing"))]
        let _ = (self, start);
    }
}
