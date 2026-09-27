//! The contextual transliterator.
//!
//! [`Transliterator`] walks the input left to right, greedily matching the
//! longest rule key at each position, and assembles the resulting [`Sound`]s into
//! correct target-script text. For Bengali this means:
//!
//! * a bare consonant carries its inherent vowel *ô*;
//! * a vowel after a consonant becomes its dependent sign (কার);
//! * a consonant directly after another consonant is joined with a virama
//!   (হসন্ত, U+09CD) to form a conjunct (যুক্তাক্ষর);
//! * unknown characters (spaces, most punctuation) pass through unchanged and
//!   reset the syllable state.

use crate::scheme::{PhoneticScheme, Sound};

/// Bengali virama / হসন্ত (U+09CD), the conjunct-forming joiner.
const VIRAMA: char = '\u{09CD}';

/// Applies a [`PhoneticScheme`] to input text, tracking syllable context.
///
/// The transliterator is cheap to create and holds only a borrow of its scheme,
/// so callers may create one per conversion.
#[derive(Clone, Copy, Debug)]
pub struct Transliterator<'a> {
    scheme: &'a PhoneticScheme,
}

impl<'a> Transliterator<'a> {
    /// Create a transliterator bound to `scheme`.
    #[must_use]
    pub fn new(scheme: &'a PhoneticScheme) -> Self {
        Self { scheme }
    }

    /// Convert a whole input buffer into target-script text.
    #[must_use]
    pub fn convert(&self, input: &str) -> String {
        let chars: Vec<char> = input.chars().collect();
        let mut out = String::with_capacity(input.len() * 3);
        // True when the previously emitted glyph was a consonant base that can
        // still take a dependent vowel or requires a virama before a consonant.
        let mut pending_consonant = false;
        let mut i = 0;

        while i < chars.len() {
            let remaining = chars.len() - i;
            let mut window = self.scheme.max_key_len().min(remaining);
            let mut matched = false;

            // Longest-match: try the widest window first, shrink to 1.
            while window >= 1 {
                let candidate: String = chars[i..i + window].iter().collect();
                if let Some(sound) = self.scheme.lookup(&candidate) {
                    Self::apply(sound, &mut out, &mut pending_consonant);
                    i += window;
                    matched = true;
                    break;
                }
                window -= 1;
            }

            if !matched {
                // Unknown character: pass through and end the current syllable.
                out.push(chars[i]);
                pending_consonant = false;
                i += 1;
            }
        }

        out
    }

    /// Emit one [`Sound`], updating the syllable state.
    fn apply(sound: &Sound, out: &mut String, pending_consonant: &mut bool) {
        match sound {
            Sound::Consonant(glyph) => {
                if *pending_consonant {
                    out.push(VIRAMA);
                }
                out.push_str(glyph);
                *pending_consonant = true;
            }
            Sound::Vowel {
                independent,
                dependent,
            } => {
                if *pending_consonant {
                    out.push_str(dependent);
                } else {
                    out.push_str(independent);
                }
                *pending_consonant = false;
            }
            Sound::Sign(glyph) => {
                out.push_str(glyph);
                *pending_consonant = false;
            }
            Sound::Halant => {
                out.push(VIRAMA);
                *pending_consonant = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{PhoneticScheme, Scheme};

    fn bn(input: &str) -> String {
        PhoneticScheme::bengali().transliterate(input)
    }

    #[test]
    fn empty_input_yields_empty() {
        assert_eq!(bn(""), "");
    }

    #[test]
    fn word_initial_vowel_is_independent() {
        assert_eq!(bn("ami"), "আমি");
        assert_eq!(bn("amar"), "আমার");
    }

    #[test]
    fn vowel_after_consonant_is_dependent_sign() {
        assert_eq!(bn("sonar"), "সোনার");
        assert_eq!(bn("bhalo"), "ভালো");
    }

    #[test]
    fn anusvara_sign() {
        assert_eq!(bn("bang"), "বাং");
        assert_eq!(bn("bangla"), "বাংলা");
    }

    #[test]
    fn full_phrase_with_spaces() {
        assert_eq!(bn("amar sonar bangla"), "আমার সোনার বাংলা");
    }

    #[test]
    fn adjacent_consonants_form_conjuncts() {
        assert_eq!(bn("kk"), "ক্ক");
        assert_eq!(bn("kt"), "ক্ত");
    }

    #[test]
    fn explicit_halant_matches_implicit_conjunct() {
        assert_eq!(bn("k,,k"), bn("kk"));
        assert_eq!(bn("k,,k"), "ক্ক");
    }

    #[test]
    fn digits_map_to_bengali_numerals() {
        assert_eq!(bn("2026"), "২০২৬");
    }

    #[test]
    fn unknown_characters_pass_through_and_reset_state() {
        assert_eq!(bn("a.b"), "আ.ব");
    }

    #[test]
    fn longest_match_prefers_longer_keys() {
        // "chh" (ছ) must win over "ch" (চ) / "c" (চ).
        assert_eq!(bn("chha"), "ছা");
        assert_eq!(bn("cha"), "চা");
    }
}
