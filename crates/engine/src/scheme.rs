//! Transliteration schemes and the linguistic units they emit.
//!
//! A [`PhoneticScheme`] is essentially an ordered rule table mapping Latin key
//! sequences (the *keys* the user types) to typed [`Sound`]s (what they mean in
//! the target script). The contextual assembly of those sounds into correct
//! Bengali — inherent vowels, dependent vowel signs (কার), and conjuncts
//! (যুক্তাক্ষর) joined by the virama (হসন্ত) — is handled by
//! [`crate::Transliterator`].

use std::collections::HashMap;

use crate::Scheme;
use crate::transliterate::Transliterator;

/// A typed linguistic unit produced by matching a romanization rule.
///
/// Making the *kind* of sound explicit (rather than emitting raw glyphs) is what
/// lets the transliterator apply Bengali orthography rules correctly and
/// generically.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sound {
    /// A consonant (ব্যঞ্জনবর্ণ). Carries an inherent vowel *ô* on its own; when
    /// it directly follows another consonant a virama (্) is inserted to form a
    /// conjunct.
    Consonant(&'static str),

    /// A vowel (স্বরবর্ণ). Rendered as its `independent` form at the start of a
    /// syllable, or as its `dependent` sign (কার) after a consonant. The inherent
    /// vowel *ô* uses an empty `dependent`, since a bare consonant already carries it.
    Vowel {
        /// Independent form, used in isolation / word-initially (e.g. `আ`).
        independent: &'static str,
        /// Dependent sign (কার), used after a consonant (e.g. `া`).
        dependent: &'static str,
    },

    /// A standalone sign that is simply appended and does not combine with a
    /// preceding consonant as a vowel does — e.g. anusvara (ং), visarga (ঃ),
    /// or chandrabindu (ঁ).
    Sign(&'static str),

    /// An explicit virama / হসন্ত (্), used to force a conjunct or a
    /// consonant-final form.
    Halant,
}

/// A phonetic transliteration scheme backed by a longest-match rule table.
///
/// Construct the Bengali scheme with [`PhoneticScheme::bengali`].
#[derive(Clone, Debug)]
pub struct PhoneticScheme {
    id: &'static str,
    display_name: &'static str,
    rules: HashMap<&'static str, Sound>,
    /// Longest rule key length in `char`s — the window size for longest-match.
    max_key_len: usize,
}

impl PhoneticScheme {
    /// Build a scheme from an explicit rule list. Later duplicate keys override
    /// earlier ones.
    #[must_use]
    pub fn new(
        id: &'static str,
        display_name: &'static str,
        rules: Vec<(&'static str, Sound)>,
    ) -> Self {
        let max_key_len = rules
            .iter()
            .map(|(k, _)| k.chars().count())
            .max()
            .unwrap_or(0);
        Self {
            id,
            display_name,
            rules: rules.into_iter().collect(),
            max_key_len,
        }
    }

    /// The longest key length (in `char`s) in this scheme's table.
    #[must_use]
    pub fn max_key_len(&self) -> usize {
        self.max_key_len
    }

    /// Look up an exact rule key.
    #[must_use]
    pub fn lookup(&self, key: &str) -> Option<&Sound> {
        self.rules.get(key)
    }

    /// The canonical Bengali phonetic scheme (clean-room; see crate docs).
    ///
    /// The romanization below is an independent, conventional mapping. It is
    /// intentionally readable and data-like so it can later be externalised into
    /// a data file without changing the engine.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn bengali() -> Self {
        use Sound::{Consonant as C, Halant, Sign as S};

        // Helper for vowels: (independent, dependent-sign).
        const fn v(independent: &'static str, dependent: &'static str) -> Sound {
            Sound::Vowel {
                independent,
                dependent,
            }
        }

        let rules: Vec<(&'static str, Sound)> = vec![
            // ---- Vowels (স্বরবর্ণ) --------------------------------------------
            // Inherent vowel ô has no key: a bare consonant already carries it.
            ("a", v("আ", "া")), // aa / আ-কার
            ("i", v("ই", "ি")),
            ("I", v("ঈ", "ী")),
            ("ii", v("ঈ", "ী")),
            ("u", v("উ", "ু")),
            ("U", v("ঊ", "ূ")),
            ("uu", v("ঊ", "ূ")),
            ("e", v("এ", "ে")),
            ("OI", v("ঐ", "ৈ")),
            ("oi", v("ঐ", "ৈ")),
            ("o", v("ও", "ো")),
            ("OU", v("ঔ", "ৌ")),
            ("ou", v("ঔ", "ৌ")),
            ("rri", v("ঋ", "ৃ")),
            // ---- Consonants (ব্যঞ্জনবর্ণ) -------------------------------------
            ("k", C("ক")),
            ("kh", C("খ")),
            ("g", C("গ")),
            ("gh", C("ঘ")),
            ("Ng", C("ঙ")),
            ("ch", C("চ")),
            ("c", C("চ")),
            ("chh", C("ছ")),
            ("j", C("জ")),
            ("jh", C("ঝ")),
            ("NG", C("ঞ")),
            ("T", C("ট")),
            ("Th", C("ঠ")),
            ("D", C("ড")),
            ("Dh", C("ঢ")),
            ("N", C("ণ")),
            ("t", C("ত")),
            ("th", C("থ")),
            ("d", C("দ")),
            ("dh", C("ধ")),
            ("n", C("ন")),
            ("p", C("প")),
            ("ph", C("ফ")),
            ("f", C("ফ")),
            ("b", C("ব")),
            ("bh", C("ভ")),
            ("v", C("ভ")),
            ("m", C("ম")),
            ("z", C("য")),
            ("r", C("র")),
            ("l", C("ল")),
            ("sh", C("শ")),
            ("Sh", C("ষ")),
            ("s", C("স")),
            ("h", C("হ")),
            ("R", C("ড়")),
            ("Rh", C("ঢ়")),
            ("y", C("য়")),
            // ---- Signs (চিহ্ন) ------------------------------------------------
            ("ng", S("ং")),  // anusvara
            (":", S("ঃ")),   // visarga
            ("^", S("ঁ")),    // chandrabindu
            ("t``", S("ৎ")), // khanda ta
            // ---- Explicit virama / হসন্ত -------------------------------------
            (",,", Halant),
            // ---- Bengali digits (সংখ্যা) -------------------------------------
            ("0", S("০")),
            ("1", S("১")),
            ("2", S("২")),
            ("3", S("৩")),
            ("4", S("৪")),
            ("5", S("৫")),
            ("6", S("৬")),
            ("7", S("৭")),
            ("8", S("৮")),
            ("9", S("৯")),
        ];

        Self::new("bengali-phonetic", "Bengali (Phonetic)", rules)
    }
}

impl Scheme for PhoneticScheme {
    fn id(&self) -> &str {
        self.id
    }

    fn display_name(&self) -> &str {
        self.display_name
    }

    fn transliterate(&self, input: &str) -> String {
        Transliterator::new(self).convert(input)
    }
}
