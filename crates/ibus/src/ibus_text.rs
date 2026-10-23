//! Serialize text into the `IBusText` D-Bus format expected by the IBus daemon.
//!
//! IBus does not send plain strings over its `CommitText` / `UpdatePreeditText`
//! signals; it sends an `IBusText`, an `IBusSerializable` object encoded as a nested
//! D-Bus variant. Confirmed against the IBus C source, the wire layout is:
//!
//! ```text
//! IBusText      = (s a{sv} s v)   -> "IBusText",      {}, <text>, <IBusAttrList as v>
//! IBusAttrList  = (s a{sv} av)    -> "IBusAttrList",  {}, [ <IBusAttribute as v> ... ]
//! IBusAttribute = (s a{sv} uuuu)  -> "IBusAttribute", {}, type, value, start, end
//! ```
//!
//! The leading `s` is the GObject type name; the `a{sv}` is the (empty) serializable
//! attachment map. We currently emit an empty attribute list (no styling); a preedit
//! underline attribute can be added later without changing callers.
//!
//! Correctness of this encoding is pinned by signature tests below, which assert the
//! exact D-Bus signatures the daemon requires.

use std::collections::HashMap;

use zbus::{Error, OwnedVariant, Structure, Variant};

/// Result alias for the fallible structure builders.
type Result<T> = std::result::Result<T, Error>;

/// `IBusAttrType::Underline` — the attribute styles the underline of a text range.
const ATTR_TYPE_UNDERLINE: u32 = 1;
/// `IBusAttrUnderline::Single` — a single underline.
const ATTR_UNDERLINE_SINGLE: u32 = 1;

/// An empty `a{sv}` — the `IBusSerializable` attachment map.
///
/// Returned as a concrete `HashMap` (not a `Variant`) so `Structure::builder` encodes it
/// as `a{sv}` rather than wrapping it in a variant.
fn empty_attachments() -> HashMap<String, OwnedVariant> {
    HashMap::new()
}

/// Build an `IBusAttribute` structure: `(s a{sv} uuuu)`.
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn attribute(kind: u32, value: u32, start: u32, end: u32) -> Result<Structure<'static>> {
    Structure::builder()
        .add_field("IBusAttribute".to_string())
        .add_field(empty_attachments())
        .add_field(kind)
        .add_field(value)
        .add_field(start)
        .add_field(end)
        .build()
}

/// A single-underline attribute spanning characters `[start, end)`.
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn underline(start: u32, end: u32) -> Result<Structure<'static>> {
    attribute(ATTR_TYPE_UNDERLINE, ATTR_UNDERLINE_SINGLE, start, end)
}

/// Build an `IBusAttrList` structure: `(s a{sv} av)`.
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn attr_list(attributes: Vec<Structure<'static>>) -> Result<Structure<'static>> {
    // Each attribute becomes a variant element of the `av` array.
    let variants: Vec<OwnedVariant> = attributes
        .into_iter()
        .filter_map(|a| OwnedVariant::try_from(Variant::from(a)).ok())
        .collect();
    Structure::builder()
        .add_field("IBusAttrList".to_string())
        .add_field(empty_attachments())
        .add_field(variants)
        .build()
}

/// Build an `IBusText` value ready to pass as the `v` argument of `CommitText` or
/// `UpdatePreeditText`. The returned [`Variant`] has signature `(s a{sv} s v)`.
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn ibus_text(text: &str) -> Result<Variant<'static>> {
    ibus_text_with_attrs(text, Vec::new())
}

/// Like [`ibus_text`] but with explicit attributes (e.g. a preedit underline).
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn ibus_text_with_attrs(
    text: &str,
    attributes: Vec<Structure<'static>>,
) -> Result<Variant<'static>> {
    let attrs = attr_list(attributes)?;
    // Passing a `Variant` field makes `Structure::builder` encode it as a variant (`v`),
    // which is exactly the IBusText layout: the attrs field is `v` holding IBusAttrList.
    let structure = Structure::builder()
        .add_field("IBusText".to_string())
        .add_field(empty_attachments())
        .add_field(text.to_string())
        .add_field(Variant::from(attrs))
        .build()?;
    Ok(Variant::from(structure))
}

#[cfg(test)]
mod tests {
    use super::{attr_list, attribute, ibus_text};
    use zbus::Variant;

    fn signature_of(value: &Variant<'_>) -> String {
        value.value_signature().to_string()
    }

    #[test]
    fn attribute_has_ibus_signature() {
        let a = Variant::from(attribute(1, 2, 0, 3).unwrap());
        assert_eq!(signature_of(&a), "(sa{sv}uuuu)");
    }

    #[test]
    fn attr_list_has_ibus_signature() {
        let a = Variant::from(attr_list(Vec::new()).unwrap());
        assert_eq!(signature_of(&a), "(sa{sv}av)");
    }

    #[test]
    fn ibus_text_has_ibus_signature() {
        let t = ibus_text("বাংলা").unwrap();
        assert_eq!(signature_of(&t), "(sa{sv}sv)");
    }

    #[test]
    fn underline_has_expected_fields() {
        // (name, {}, type=1 underline, value=1 single, start=0, end=5)
        let u = Variant::from(super::underline(0, 5).unwrap());
        assert_eq!(signature_of(&u), "(sa{sv}uuuu)");
        if let Variant::Structure(s) = &u {
            let f = s.fields();
            assert_eq!(f[2], Variant::U32(1)); // underline type
            assert_eq!(f[3], Variant::U32(1)); // single
            assert_eq!(f[4], Variant::U32(0)); // start
            assert_eq!(f[5], Variant::U32(5)); // end
        } else {
            panic!("underline must be a structure");
        }
    }

    #[test]
    fn ibus_text_carries_the_string() {
        // The 3rd field (index 2) of the IBusText structure is the text.
        let t = ibus_text("আমি").unwrap();
        if let Variant::Structure(s) = &t {
            let fields = s.fields();
            match &fields[2] {
                Variant::Str(text) => assert_eq!(text.as_str(), "আমি"),
                other => panic!("expected text at field 2, got {other:?}"),
            }
        } else {
            panic!("ibus_text must be a structure");
        }
    }
}
