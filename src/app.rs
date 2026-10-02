use std::ops::Range;

use crate::clipboard::ClipboardBackend;
use crate::corpus::Corpus;
use crate::fonts;
use crate::navigation::Selection;
use crate::perf::Timing;
use crate::search::SearchResults;

const SEARCH_HEIGHT: f32 = 38.0;
const CELL_SIZE: f32 = 44.0;
const GLYPH_SIZE: f32 = 27.0;

pub struct GlyphflickApp<B> {
    clipboard: B,
    corpus: Corpus,
    results: SearchResults,
    selection: Selection,
    query: String,
    error: Option<String>,
    focus_search: bool,
    first_ui: bool,
    exit_requested: bool,
    columns: usize,
    visible_rows: Range<usize>,
    scroll_row: Option<usize>,
    timing: Timing,
    emoji_font: egui::FontId,
}

impl<B: ClipboardBackend> GlyphflickApp<B> {
    pub fn new(ctx: &egui::Context, clipboard: B, timing: Timing) -> Self {
        fonts::install(ctx);
        let emoji_font = fonts::emoji_font_id(GLYPH_SIZE);
        ctx.set_visuals(egui::Visuals::dark());
        ctx.all_styles_mut(|style| {
            style.animation_time = 0.0;
            style.interaction.tooltip_delay = 0.0;
            style.interaction.show_tooltips_only_when_still = false;
        });

        let corpus_start = timing.stamp();
        let corpus = Corpus::emoji();
        timing.report_corpus(corpus_start, corpus.len());
        let results = SearchResults::new(&corpus);

        Self {
            clipboard,
            corpus,
            results,
            selection: Selection::default(),
            query: String::new(),
            error: None,
            focus_search: true,
            first_ui: true,
            exit_requested: false,
            columns: 1,
            visible_rows: 0..0,
            scroll_row: None,
            timing,
            emoji_font,
        }
    }

    #[inline]
    pub const fn exit_requested(&self) -> bool {
        self.exit_requested
    }

    fn refresh_results(&mut self) {
        let start = self.timing.stamp();
        self.results.update(&self.corpus, &self.query);
        self.selection.reset();
        self.scroll_row = Some(0);
        self.timing.report_search(start, self.results.len());
    }

    fn handle_navigation(&mut self, ctx: &egui::Context) {
        let has_active_selection = self.selection.active().is_some();
        let modifiers = egui::Modifiers::default();

        let (up, down, left, right) = ctx.input_mut(|input| {
            let up = input.consume_key(modifiers, egui::Key::ArrowUp);
            let down = input.consume_key(modifiers, egui::Key::ArrowDown);

            let (left, right) = if has_active_selection {
                (
                    input.consume_key(modifiers, egui::Key::ArrowLeft),
                    input.consume_key(modifiers, egui::Key::ArrowRight),
                )
            } else {
                (false, false)
            };

            (up, down, left, right)
        });

        let len = self.results.len();
        let moved = if up {
            self.selection.move_up(len, self.columns)
        } else if down {
            self.selection.move_down(len, self.columns)
        } else if left {
            self.selection.move_left(len)
        } else if right {
            self.selection.move_right(len)
        } else {
            false
        };

        if moved {
            self.ensure_selection_visible();
        }
    }

    fn ensure_selection_visible(&mut self) {
        let Some(position) = self.selection.active() else {
            return;
        };

        let row = position / self.columns.max(1);
        if !self.visible_rows.contains(&row) {
            self.scroll_row = Some(row);
        }
    }

    fn commit(&mut self, text: &'static str) {
        let start = self.timing.stamp();
        match self.clipboard.copy(text) {
            Ok(()) => {
                self.timing.report_clipboard(start);
                self.exit_requested = true;
            }
            Err(error) => {
                self.error = Some(error.to_string());
            }
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if self.first_ui {
            self.first_ui = false;
            self.timing.mark_first_ui();
        }

        if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.exit_requested = true;
            return;
        }

        self.handle_navigation(ui.ctx());

        let mut picked = None;

        egui::CentralPanel::default().show(ui, |ui| {
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
            self.columns = columns;

            let row_height = CELL_SIZE + ui.spacing().item_spacing.y;
            let rows = self.results.len().div_ceil(columns);
            let active = self.selection.active();

            let mut scroll = egui::ScrollArea::vertical().auto_shrink([false, false]);
            if let Some(row) = self.scroll_row.take() {
                scroll = scroll.vertical_scroll_offset(row as f32 * row_height);
            }

            let mut visible_rows = self.visible_rows.clone();
            scroll.show_rows(ui, row_height, rows, |ui, row_range| {
                visible_rows = row_range.clone();

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
                                    egui::Button::selectable(
                                        active == Some(position),
                                        egui::RichText::new(glyph.text())
                                            .font(self.emoji_font.clone()),
                                    ),
                                )
                                .on_hover_text(glyph.name());

                            if response.clicked() {
                                picked = Some(glyph.text());
                            }
                        }
                    });
                }
            });
            self.visible_rows = visible_rows;

            if self.results.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.weak("No matching glyphs");
                });
            }
        });

        if picked.is_none() && ui.input(|input| input.key_pressed(egui::Key::Enter)) {
            let position = self.selection.active().unwrap_or(0);
            picked = self
                .results
                .get(position)
                .map(|index| self.corpus.get(index).text());
        }

        if let Some(text) = picked {
            self.commit(text);
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
