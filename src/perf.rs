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
}

impl Default for Timing {
    #[inline(always)]
    fn default() -> Self {
        Self {
            #[cfg(feature = "timing")]
            process_start: Instant::now(),
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
