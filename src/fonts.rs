use std::fs::File;
use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily};
use epaint_default_fonts::UBUNTU_LIGHT;
use memmap2::{Mmap, MmapOptions};

const UBUNTU: &str = "Ubuntu-Light";
const SYSTEM_EMOJI: &str = "NotoColorEmoji";
const EMOJI_FAMILY: &str = "Glyphflick Emoji";

const SYSTEM_EMOJI_PATHS: &[&str] = &[
    // Arch / EndeavourOS: extra/noto-fonts-emoji
    "/usr/share/fonts/noto/NotoColorEmoji.ttf",
    // Debian / Ubuntu:
    "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
    // Common manually installed / alternate package layouts:
    "/usr/share/fonts/TTF/NotoColorEmoji.ttf",
    "/usr/local/share/fonts/NotoColorEmoji.ttf",
];

struct MappedFont(Mmap);

impl AsRef<[u8]> for MappedFont {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmojiFontStatus {
    Mapped(&'static str),
    Missing,
}

pub fn install(ctx: &egui::Context) -> EmojiFontStatus {
    let (definitions, status) = definitions();
    ctx.set_fonts(definitions);
    status
}

#[inline]
pub fn emoji_font_id(size: f32) -> egui::FontId {
    egui::FontId::new(size, FontFamily::Name(Arc::from(EMOJI_FAMILY)))
}

fn definitions() -> (FontDefinitions, EmojiFontStatus) {
    let mut fonts = FontDefinitions::empty();

    fonts.font_data.insert(
        UBUNTU.to_owned(),
        Arc::new(FontData::from_static(UBUNTU_LIGHT)),
    );

    let status = if let Some((path, data)) = map_system_emoji_font() {
        fonts
            .font_data
            .insert(SYSTEM_EMOJI.to_owned(), Arc::new(data));
        EmojiFontStatus::Mapped(path)
    } else {
        EmojiFontStatus::Missing
    };

    let ui_family = vec![UBUNTU.to_owned()];
    fonts
        .families
        .insert(FontFamily::Proportional, ui_family.clone());
    fonts.families.insert(FontFamily::Monospace, ui_family);

    let emoji_family = if status == EmojiFontStatus::Missing {
        vec![UBUNTU.to_owned()]
    } else {
        vec![SYSTEM_EMOJI.to_owned()]
    };
    fonts
        .families
        .insert(FontFamily::Name(Arc::from(EMOJI_FAMILY)), emoji_family);

    (fonts, status)
}

fn map_system_emoji_font() -> Option<(&'static str, FontData)> {
    for &path in SYSTEM_EMOJI_PATHS {
        let Ok(file) = File::open(path) else {
            continue;
        };

        // SAFETY: these are read-only system font files. We retain the immutable
        // mapping for as long as egui can reference its bytes.
        let Ok(mapping) = (unsafe { MmapOptions::new().map(&file) }) else {
            continue;
        };

        let blob: epaint::text::Blob = Arc::new(MappedFont(mapping));
        return Some((path, FontData::from_blob(blob, 0)));
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::corpus::Corpus;

    fn is_sequence_control(c: char) -> bool {
        matches!(c, '\u{200D}' | '\u{20E3}' | '\u{FE0E}' | '\u{FE0F}')
            || ('\u{E0020}'..='\u{E007F}').contains(&c)
    }

    #[test]
    #[expect(clippy::print_stdout)]
    fn system_color_font_coverage_report() {
        let ctx = egui::Context::default();
        let status = install(&ctx);

        let EmojiFontStatus::Mapped(path) = status else {
            println!("glyphflick system emoji coverage: skipped (Noto Color Emoji not found)");
            return;
        };

        let corpus = Corpus::emoji();
        let font_id = emoji_font_id(27.0);
        let mut scalars = BTreeSet::new();
        for (_, glyph) in corpus.iter() {
            scalars.extend(glyph.text().chars().filter(|&c| !is_sequence_control(c)));
        }

        let mut prime = ctx.run_ui(Default::default(), |_| {});
        prime.textures_delta.clear();

        let mut missing = Vec::new();
        let scalar_sufficient_entries = ctx.fonts_mut(|fonts| {
            for &scalar in &scalars {
                if !fonts.has_glyph(&font_id, scalar) {
                    missing.push(scalar);
                }
            }

            corpus
                .iter()
                .filter(|(_, glyph)| {
                    glyph
                        .text()
                        .chars()
                        .filter(|&c| !is_sequence_control(c))
                        .all(|c| fonts.has_glyph(&font_id, c))
                })
                .count()
        });

        println!(
            "glyphflick system emoji coverage: path={path} visible_scalars={}/{} missing={} scalar_sufficient_entries={}/{}",
            scalars.len() - missing.len(),
            scalars.len(),
            missing.len(),
            scalar_sufficient_entries,
            corpus.len()
        );

        for sequence in ["🚀", "👍🏽", "🇲🇽", "👨‍👩‍👧‍👦"] {
            let galley = ctx.fonts_mut(|fonts| {
                fonts.layout_no_wrap(sequence.to_owned(), font_id.clone(), egui::Color32::WHITE)
            });
            assert!(
                galley.num_vertices > 0,
                "system color font failed to rasterize representative sequence {sequence:?}"
            );
        }
        println!("glyphflick system emoji coverage: representative_rasterization=ok");
    }

    #[test]
    fn font_set_keeps_ui_and_emoji_families_separate() {
        let (fonts, status) = definitions();

        assert_eq!(fonts.families[&FontFamily::Proportional], [UBUNTU]);
        assert_eq!(fonts.families[&FontFamily::Monospace], [UBUNTU]);

        let emoji = &fonts.families[&FontFamily::Name(Arc::from(EMOJI_FAMILY))];
        match status {
            EmojiFontStatus::Mapped(_) => assert_eq!(emoji, [SYSTEM_EMOJI]),
            EmojiFontStatus::Missing => assert_eq!(emoji, [UBUNTU]),
        }
    }
}
