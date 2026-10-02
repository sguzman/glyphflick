use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily, FontTweak};
use epaint_default_fonts::{EMOJI_ICON, NOTO_EMOJI_REGULAR, UBUNTU_LIGHT};

const UBUNTU: &str = "Ubuntu-Light";
const NOTO_EMOJI: &str = "NotoEmoji-Regular";
const EMOJI_ICON_FONT: &str = "emoji-icon-font";

pub fn install(ctx: &egui::Context) {
    ctx.set_fonts(definitions());
}

fn definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::empty();

    fonts.font_data.insert(
        UBUNTU.to_owned(),
        Arc::new(FontData::from_static(UBUNTU_LIGHT)),
    );
    fonts.font_data.insert(
        NOTO_EMOJI.to_owned(),
        Arc::new(FontData::from_static(NOTO_EMOJI_REGULAR).tweak(FontTweak {
            scale: 0.81,
            ..Default::default()
        })),
    );
    fonts.font_data.insert(
        EMOJI_ICON_FONT.to_owned(),
        Arc::new(FontData::from_static(EMOJI_ICON).tweak(FontTweak {
            scale: 0.90,
            ..Default::default()
        })),
    );

    let family = vec![
        UBUNTU.to_owned(),
        NOTO_EMOJI.to_owned(),
        EMOJI_ICON_FONT.to_owned(),
    ];

    fonts
        .families
        .insert(FontFamily::Proportional, family.clone());
    fonts.families.insert(FontFamily::Monospace, family);

    fonts
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
    fn bundled_font_scalar_coverage_report() {
        let ctx = egui::Context::default();
        install(&ctx);

        let corpus = Corpus::emoji();
        let font_id = egui::FontId::proportional(27.0);
        let mut scalars = BTreeSet::new();

        for (_, glyph) in corpus.iter() {
            scalars.extend(glyph.text().chars().filter(|&c| !is_sequence_control(c)));
        }

        let mut prime = ctx.run_ui(Default::default(), |_| {});
        prime.textures_delta.clear();

        let mut missing = Vec::new();
        let renderable_entries = ctx.fonts_mut(|fonts| {
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

        let covered = scalars.len() - missing.len();
        println!(
            "glyphflick font coverage: visible_scalars={}/{} missing={} scalar_sufficient_entries={}/{}",
            covered,
            scalars.len(),
            missing.len(),
            renderable_entries,
            corpus.len()
        );

        if !missing.is_empty() {
            let missing = missing
                .iter()
                .map(|c| format!("U+{:04X} {}", *c as u32, c))
                .collect::<Vec<_>>()
                .join(", ");
            println!("missing visible scalars: {missing}");
        }
    }

    #[test]
    #[expect(clippy::print_stdout)]
    fn pinned_outline_font_probe() {
        let Ok(path) = std::env::var("GLYPHFLICK_FONT_PROBE") else {
            println!("glyphflick font probe: skipped (GLYPHFLICK_FONT_PROBE unset)");
            return;
        };

        let bytes = std::fs::read(&path).expect("failed to read pinned font probe");
        let mut definitions = FontDefinitions::empty();
        definitions.font_data.insert(
            UBUNTU.to_owned(),
            Arc::new(FontData::from_static(UBUNTU_LIGHT)),
        );
        definitions.font_data.insert(
            "NotoEmoji-Probe".to_owned(),
            Arc::new(FontData::from_owned(bytes).tweak(FontTweak {
                scale: 0.81,
                ..Default::default()
            })),
        );

        let family = vec![UBUNTU.to_owned(), "NotoEmoji-Probe".to_owned()];
        definitions
            .families
            .insert(FontFamily::Proportional, family.clone());
        definitions.families.insert(FontFamily::Monospace, family);

        let ctx = egui::Context::default();
        ctx.set_fonts(definitions);

        let corpus = Corpus::emoji();
        let font_id = egui::FontId::proportional(27.0);
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
            "glyphflick pinned outline probe: visible_scalars={}/{} missing={} scalar_sufficient_entries={}/{}",
            scalars.len() - missing.len(),
            scalars.len(),
            missing.len(),
            scalar_sufficient_entries,
            corpus.len()
        );

        let galley = ctx.fonts_mut(|fonts| {
            fonts.layout_no_wrap(
                "🚀🤖🦀".to_owned(),
                font_id.clone(),
                egui::Color32::WHITE,
            )
        });
        let rasterized = galley
            .rows
            .iter()
            .flat_map(|row| row.glyphs.iter())
            .filter(|glyph| !is_sequence_control(glyph.chr))
            .all(|glyph| !glyph.uv_rect.is_nothing());
        assert!(
            rasterized,
            "pinned outline font has charmap entries that egui 0.36.2 cannot rasterize"
        );
        println!("glyphflick pinned outline probe: representative_rasterization=ok");

        if !missing.is_empty() {
            let sample = missing
                .iter()
                .take(40)
                .map(|c| format!("U+{:04X} {}", *c as u32, c))
                .collect::<Vec<_>>()
                .join(", ");
            println!("pinned outline probe missing sample: {sample}");
        }
    }

    #[test]
    fn glyphflick_font_set_excludes_unused_hack_face() {
        let fonts = definitions();

        assert_eq!(fonts.font_data.len(), 3);
        assert!(!fonts.font_data.contains_key("Hack"));
        assert_eq!(
            fonts.families[&FontFamily::Proportional],
            [UBUNTU, NOTO_EMOJI, EMOJI_ICON_FONT]
        );
    }
}
