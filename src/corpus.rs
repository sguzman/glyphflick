#[derive(Clone, Copy)]
pub struct GlyphRecord {
    emoji: &'static emojis::Emoji,
}

impl GlyphRecord {
    #[inline]
    pub const fn text(self) -> &'static str {
        self.emoji.as_str()
    }

    #[inline]
    pub const fn name(self) -> &'static str {
        self.emoji.name()
    }

    #[inline]
    pub fn shortcodes(self) -> impl Iterator<Item = &'static str> + Clone {
        self.emoji.shortcodes()
    }
}

pub struct Corpus {
    glyphs: Box<[GlyphRecord]>,
}

impl Corpus {
    pub fn emoji() -> Self {
        let mut glyphs = Vec::with_capacity(5_000);

        for emoji in emojis::iter() {
            glyphs.push(GlyphRecord { emoji });

            if let Some(tones) = emoji.skin_tones() {
                glyphs.extend(tones.skip(1).map(|emoji| GlyphRecord { emoji }));
            }
        }

        Self {
            glyphs: glyphs.into_boxed_slice(),
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> GlyphRecord {
        self.glyphs[index]
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    #[inline]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (usize, GlyphRecord)> + '_ {
        self.glyphs.iter().copied().enumerate()
    }
}

#[cfg(test)]
mod tests {
    use super::Corpus;

    #[test]
    fn corpus_contains_multi_codepoint_sequences_unchanged() {
        let corpus = Corpus::emoji();
        assert!(corpus.iter().any(|(_, glyph)| glyph.text() == "👨‍👩‍👧‍👦"));
    }

    #[test]
    fn corpus_includes_skin_tone_variants() {
        let corpus = Corpus::emoji();
        assert!(corpus.iter().any(|(_, glyph)| glyph.text() == "🙌🏽"));
    }
}
