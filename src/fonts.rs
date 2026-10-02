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

        let mut missing = Vec::new();
        let mut renderable_entries = 0usize;

        let mut output = ctx.run_ui(Default::default(), |ui| {
            ui.fonts_mut(|fonts| {
                for &scalar in &scalars {
                    if !fonts.has_glyph(&font_id, scalar) {
                        missing.push(scalar);
                    }
                }

                renderable_entries = corpus
                    .iter()
                    .filter(|(_, glyph)| {
                        glyph
                            .text()
                            .chars()
                            .filter(|&c| !is_sequence_control(c))
                            .all(|c| fonts.has_glyph(&font_id, c))
                    })
                    .count();
            });
        });
        output.textures_delta.clear();

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
