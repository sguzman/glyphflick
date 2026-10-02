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
    use super::*;

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
