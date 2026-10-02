use std::time::Instant;

use eframe::egui;

use crate::clipboard::ClipboardBackend;
use crate::corpus::Corpus;
use crate::perf::Timing;
use crate::search::SearchResults;

const SEARCH_HEIGHT: f32 = 38.0;
const CELL_SIZE: f32 = 44.0;
const GLYPH_SIZE: f32 = 27.0;

pub struct GlyphflickApp<B> {
    clipboard: B,
    corpus: Corpus,
    results: SearchResults,
    query: String,
    error: Option<String>,
    focus_search: bool,
    first_ui: bool,
    timing: Timing,
}

impl<B: ClipboardBackend> GlyphflickApp<B> {
    pub fn new(cc: &eframe::CreationContext<'_>, clipboard: B, timing: Timing) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let corpus_start = Instant::now();
        let corpus = Corpus::emoji();
        timing.report_corpus(corpus_start.elapsed(), corpus.len());
        let results = SearchResults::new(&corpus);

        Self {
            clipboard,
            corpus,
            results,
            query: String::new(),
            error: None,
            focus_search: true,
            first_ui: true,
            timing,
        }
    }

    fn refresh_results(&mut self) {
        let start = Instant::now();
        self.results.update(&self.corpus, &self.query);
        self.timing
            .report_search(start.elapsed(), self.results.len());
    }

    fn commit(&mut self, ctx: &egui::Context, text: &'static str) {
        let start = Instant::now();
        match self.clipboard.copy(text) {
            Ok(()) => {
                self.timing.report_clipboard(start.elapsed());
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            Err(error) => {
                self.error = Some(error.to_string());
            }
        }
    }
}

impl<B: ClipboardBackend> eframe::App for GlyphflickApp<B> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.first_ui {
            self.first_ui = false;
            self.timing.mark_first_ui();
        }

        let ctx = ui.ctx().clone();
        if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        let mut picked = None;

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let response = ui.add_sized(
                [ui.available_width(), SEARCH_HEIGHT],
                egui::TextEdit::singleline(&mut self.query)
                    .hint_text("Search glyphs…")
                    .desired_width(f32::INFINITY),
            );

            if self.focus_search {
                response.request_focus();
                self.focus_search = false;
            }

            if response.changed() {
                self.error = None;
                self.refresh_results();
            }

            if let Some(error) = &self.error {
                ui.colored_label(ui.visuals().error_fg_color, error);
            }

            let spacing = ui.spacing().item_spacing.x;
            let columns = ((ui.available_width() + spacing) / (CELL_SIZE + spacing))
                .floor()
                .max(1.0) as usize;
            let row_height = CELL_SIZE + ui.spacing().item_spacing.y;
            let rows = self.results.len().div_ceil(columns);

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_rows(ui, row_height, rows, |ui, row_range| {
                    for row in row_range {
                        ui.horizontal(|ui| {
                            for column in 0..columns {
                                let position = row * columns + column;
                                let Some(index) = self.results.get(position) else {
                                    break;
                                };
                                let glyph = self.corpus.get(index);

                                let response = ui
                                    .add_sized(
                                        [CELL_SIZE, CELL_SIZE],
                                        egui::Button::new(
                                            egui::RichText::new(glyph.text()).size(GLYPH_SIZE),
                                        )
                                        .frame(false),
                                    )
                                    .on_hover_text(glyph.name());

                                if response.clicked() {
                                    picked = Some(glyph.text());
                                }
                            }
                        });
                    }
                });

            if self.results.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.weak("No matching glyphs");
                });
            }
        });

        if picked.is_none() && ctx.input(|input| input.key_pressed(egui::Key::Enter)) {
            picked = self.results.get(0).map(|index| self.corpus.get(index).text());
        }

        if let Some(text) = picked {
            self.commit(&ctx, text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clipboard::ClipboardError;

    #[derive(Default)]
    struct FakeClipboard {
        copied: Vec<String>,
        fail: bool,
    }

    impl ClipboardBackend for FakeClipboard {
        fn copy(&mut self, text: &str) -> Result<(), ClipboardError> {
            if self.fail {
                Err(ClipboardError::test("no clipboard"))
            } else {
                self.copied.push(text.to_owned());
                Ok(())
            }
        }
    }

    #[test]
    fn fake_clipboard_preserves_exact_unicode_sequence() {
        let mut clipboard = FakeClipboard::default();
        clipboard.copy("👨‍👩‍👧‍👦").unwrap();
        assert_eq!(clipboard.copied, ["👨‍👩‍👧‍👦"]);
    }

    #[test]
    fn fake_clipboard_can_exercise_failure_path() {
        let mut clipboard = FakeClipboard {
            copied: Vec::new(),
            fail: true,
        };
        assert!(clipboard.copy("🚀").is_err());
        assert!(clipboard.copied.is_empty());
    }
}
