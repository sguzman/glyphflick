use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub struct Timing {
    enabled: bool,
    process_start: Instant,
}

impl Timing {
    pub fn from_env(process_start: Instant) -> Self {
        Self {
            enabled: std::env::var_os("GLYPHFLICK_TIMING").is_some(),
            process_start,
        }
    }

    #[inline]
    pub fn report_corpus(self, elapsed: Duration, glyph_count: usize) {
        if self.enabled {
            eprintln!(
                "glyphflick timing corpus_init_us={} glyphs={glyph_count}",
                elapsed.as_micros()
            );
        }
    }

    #[inline]
    pub fn mark_first_ui(self) {
        if self.enabled {
            report("startup_to_first_ui", self.process_start.elapsed());
        }
    }

    #[inline]
    pub fn report_search(self, elapsed: Duration, result_count: usize) {
        if self.enabled {
            eprintln!(
                "glyphflick timing search_us={} results={result_count}",
                elapsed.as_micros()
            );
        }
    }

    #[inline]
    pub fn report_clipboard(self, elapsed: Duration) {
        if self.enabled {
            report("clipboard_establish", elapsed);
        }
    }
}

fn report(label: &str, elapsed: Duration) {
    eprintln!(
        "glyphflick timing {label}_us={}",
        elapsed.as_micros()
    );
}
