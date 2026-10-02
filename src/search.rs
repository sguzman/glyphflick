use crate::corpus::{Corpus, GlyphRecord};

const BUCKET_COUNT: usize = 7;

pub struct SearchResults {
    ordered: Vec<usize>,
    buckets: [Vec<usize>; BUCKET_COUNT],
    total_len: usize,
    all: bool,
}

impl SearchResults {
    pub fn new(corpus: &Corpus) -> Self {
        Self {
            ordered: Vec::new(),
            buckets: std::array::from_fn(|_| Vec::new()),
            total_len: corpus.len(),
            all: true,
        }
    }

    pub fn update(&mut self, corpus: &Corpus, query: &str) {
        let query = query.trim();
        if query.is_empty() {
            self.ordered.clear();
            self.all = true;
            return;
        }

        self.all = false;

        for bucket in &mut self.buckets {
            bucket.clear();
        }

        for (index, glyph) in corpus.iter() {
            if let Some(rank) = rank(glyph, query) {
                self.buckets[rank].push(index);
            }
        }

        self.ordered.clear();
        for bucket in &self.buckets {
            self.ordered.extend_from_slice(bucket);
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        if self.all {
            self.total_len
        } else {
            self.ordered.len()
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn get(&self, position: usize) -> Option<usize> {
        if self.all {
            (position < self.total_len).then_some(position)
        } else {
            self.ordered.get(position).copied()
        }
    }
}

fn rank(glyph: GlyphRecord, query: &str) -> Option<usize> {
    let name = glyph.name();

    if glyph.text() == query {
        return Some(0);
    }
    if glyph.shortcodes().any(|code| eq_ascii(code, query)) {
        return Some(1);
    }
    if eq_ascii(name, query) {
        return Some(2);
    }
    if starts_with_ascii(name, query) {
        return Some(3);
    }
    if glyph.shortcodes().any(|code| starts_with_ascii(code, query)) {
        return Some(4);
    }
    if name
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|word| starts_with_ascii(word, query))
    {
        return Some(5);
    }
    if contains_ascii(name, query)
        || glyph.shortcodes().any(|code| contains_ascii(code, query))
    {
        return Some(6);
    }

    None
}

#[inline]
fn eq_ascii(haystack: &str, needle: &str) -> bool {
    haystack.eq_ignore_ascii_case(needle)
}

#[inline]
fn starts_with_ascii(haystack: &str, needle: &str) -> bool {
    haystack
        .get(..needle.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(needle))
}

fn contains_ascii(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }

    haystack
        .as_bytes()
        .windows(needle.len())
        .any(|window| ascii_bytes_eq_ignore_case(window, needle.as_bytes()))
}

#[inline]
fn ascii_bytes_eq_ignore_case(left: &[u8], right: &[u8]) -> bool {
    left.iter()
        .zip(right)
        .all(|(a, b)| a.eq_ignore_ascii_case(b))
}

#[cfg(test)]
mod tests {
    use super::SearchResults;
    use crate::corpus::Corpus;

    fn first_text(query: &str) -> &'static str {
        let corpus = Corpus::emoji();
        let mut results = SearchResults::new(&corpus);
        results.update(&corpus, query);
        corpus.get(results.get(0).expect("query should match")).text()
    }

    #[test]
    fn empty_query_is_an_implicit_identity_view() {
        let corpus = Corpus::emoji();
        let results = SearchResults::new(&corpus);

        assert_eq!(results.len(), corpus.len());
        assert_eq!(results.get(0), Some(0));
        assert_eq!(results.get(corpus.len() - 1), Some(corpus.len() - 1));
        assert_eq!(results.get(corpus.len()), None);
        assert!(results.ordered.is_empty());
    }

    #[test]
    fn clearing_query_returns_to_identity_view_without_refilling_indices() {
        let corpus = Corpus::emoji();
        let mut results = SearchResults::new(&corpus);

        results.update(&corpus, "rocket");
        assert!(!results.ordered.is_empty());

        results.update(&corpus, "");
        assert_eq!(results.len(), corpus.len());
        assert!(results.ordered.is_empty());
    }

    #[test]
    fn exact_shortcode_wins() {
        assert_eq!(first_text("rocket"), "🚀");
    }

    #[test]
    fn search_is_ascii_case_insensitive() {
        assert_eq!(first_text("RoCkEt"), "🚀");
    }

    #[test]
    fn canonical_name_is_searchable() {
        assert_eq!(first_text("thinking face"), "🤔");
    }

    #[test]
    fn substring_is_searchable() {
        let corpus = Corpus::emoji();
        let mut results = SearchResults::new(&corpus);
        results.update(&corpus, "sunglasses");
        assert!(!results.is_empty());
    }
}
