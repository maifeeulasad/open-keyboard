# Bengali phonetic scheme (v0)

This is the human-readable specification of the clean-room English→Bengali
romanization implemented in `PhoneticScheme::bengali`
(`crates/engine/src/scheme.rs`). It is authored independently from the Unicode
Bengali block and common transliteration conventions — **no rule tables were copied
from Avro, OpenBangla, or any other project** ([ADR-0006](../decisions/ADR-0006-clean-room-phonetic-scheme.md)).

> Status: **v0** — a deliberately small, correct, well-tested subset. It will grow;
> every addition gets a test and an update here.

## How to read the tables

- **Key** = what you type (case-sensitive: `t`≠`T`, `s`≠`Sh`).
- Longer keys win over shorter prefixes (`chh` before `ch` before `c`).
- **Consonants** carry an inherent *ô*, so `k` alone is `ক` (kô).
- **Vowels** have two forms: *independent* (word-initial / standalone) and
  *dependent* (the কার sign used after a consonant).

## Vowels (স্বরবর্ণ)

| Key | Independent | Dependent (কার) | Example |
| --- | ----------- | ---------------- | ------- |
| *(none)* | — | inherent *ô* | `k` → `ক` |
| `a` | আ | া | `ka` → `কা` |
| `i` | ই | ি | `ki` → `কি` |
| `I`, `ii` | ঈ | ী | `kI` → `কী` |
| `u` | উ | ু | `ku` → `কু` |
| `U`, `uu` | ঊ | ূ | `kU` → `কূ` |
| `e` | এ | ে | `ke` → `কে` |
| `oi`, `OI` | ঐ | ৈ | `koi` → `কৈ` |
| `o` | ও | ো | `ko` → `কো` |
| `ou`, `OU` | ঔ | ৌ | `kou` → `কৌ` |
| `rri` | ঋ | ৃ | `krri` → `কৃ` |

## Consonants (ব্যঞ্জনবর্ণ)

| Key | Glyph | Key | Glyph | Key | Glyph |
| --- | ----- | --- | ----- | --- | ----- |
| `k` | ক | `kh` | খ | `g` | গ |
| `gh` | ঘ | `Ng` | ঙ | `ch`, `c` | চ |
| `chh` | ছ | `j` | জ | `jh` | ঝ |
| `NG` | ঞ | `T` | ট | `Th` | ঠ |
| `D` | ড | `Dh` | ঢ | `N` | ণ |
| `t` | ত | `th` | থ | `d` | দ |
| `dh` | ধ | `n` | ন | `p` | প |
| `ph`, `f` | ফ | `b` | ব | `bh`, `v` | ভ |
| `m` | ম | `z` | য | `r` | র |
| `l` | ল | `sh` | শ | `Sh` | ষ |
| `s` | স | `h` | হ | `R` | ড় |
| `Rh` | ঢ় | `y` | য় | | |

## Signs (চিহ্ন)

| Key | Glyph | Name |
| --- | ----- | ---- |
| `ng` | ং | anusvara (অনুস্বার) |
| `:` | ঃ | visarga (বিসর্গ) |
| `^` | ঁ | chandrabindu (চন্দ্রবিন্দু) |
| `` t`` `` | ৎ | khanda ta (খণ্ড ত) |

## Conjuncts (যুক্তাক্ষর)

Two consonants in a row are joined automatically with the virama (হসন্ত, `্`):

| Type | Result |
| ---- | ------ |
| `kk` | ক্ক |
| `kt` | ক্ত |
| `kSh` | ক্ষ (`k` + `Sh`) |

To force a virama explicitly (e.g. a consonant-final form), type `,,`:

- `k,,k` → `ক্ক` (same as `kk`).

## Digits (সংখ্যা)

`0`–`9` map to Bengali numerals `০`–`৯`. Example: `2026` → `২০২৬`.

## Worked examples

| Type | Get |
| ---- | --- |
| `ami` | আমি |
| `amar` | আমার |
| `sonar` | সোনার |
| `bangla` | বাংলা |
| `amar sonar bangla` | আমার সোনার বাংলা |
| `ami bangla likhi` | আমি বাংলা লিখি |
| `bhalo` | ভালো |

## Known gaps (tracked for later versions)

- No `y`-phala / `r`-phala shortcuts yet (e.g. `্য`, `্র` via a dedicated key).
- No dictionary-backed word suggestions/auto-correct (Phase 5).
- Retroflex/dental and sh/Sh/s choices follow the table strictly; a suggestion
  layer will later offer alternatives.
- Fixed layouts (Probhat/Jatiya-style) are a separate future `Scheme`.
