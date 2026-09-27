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

use zbus::zvariant::{Error, OwnedValue, Structure, StructureBuilder, Value};

/// Result alias for the fallible structure builders.
type Result<T> = std::result::Result<T, Error>;

/// An empty `a{sv}` — the `IBusSerializable` attachment map.
///
/// Returned as a concrete `HashMap` (not a `Value`) so `StructureBuilder` encodes it
/// as `a{sv}` rather than wrapping it in a variant.
fn empty_attachments() -> HashMap<String, OwnedValue> {
    HashMap::new()
}

/// Build an `IBusAttribute` structure: `(s a{sv} uuuu)`.
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn attribute(kind: u32, value: u32, start: u32, end: u32) -> Result<Structure<'static>> {
    StructureBuilder::new()
        .add_field("IBusAttribute".to_string())
        .add_field(empty_attachments())
        .add_field(kind)
        .add_field(value)
        .add_field(start)
        .add_field(end)
        .build()
}

/// Build an `IBusAttrList` structure: `(s a{sv} av)`.
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn attr_list(attributes: Vec<Structure<'static>>) -> Result<Structure<'static>> {
    // Each attribute becomes a variant element of the `av` array.
    let variants: Vec<OwnedValue> = attributes
        .into_iter()
        .filter_map(|a| OwnedValue::try_from(Value::from(a)).ok())
        .collect();
    StructureBuilder::new()
        .add_field("IBusAttrList".to_string())
        .add_field(empty_attachments())
        .add_field(variants)
        .build()
}

/// Build an `IBusText` value ready to pass as the `v` argument of `CommitText` or
/// `UpdatePreeditText`. The returned [`Value`] has signature `(s a{sv} s v)`.
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn ibus_text(text: &str) -> Result<Value<'static>> {
    ibus_text_with_attrs(text, Vec::new())
}

/// Like [`ibus_text`] but with explicit attributes (e.g. a preedit underline).
///
/// # Errors
/// Returns an error if the D-Bus structure cannot be assembled.
pub fn ibus_text_with_attrs(
    text: &str,
    attributes: Vec<Structure<'static>>,
) -> Result<Value<'static>> {
    let attrs = attr_list(attributes)?;
    // Passing a `Value` field makes `StructureBuilder` encode it as a variant (`v`),
    // which is exactly the IBusText layout: the attrs field is `v` holding IBusAttrList.
    let structure = StructureBuilder::new()
        .add_field("IBusText".to_string())
        .add_field(empty_attachments())
        .add_field(text.to_string())
        .add_field(Value::from(attrs))
        .build()?;
    Ok(Value::from(structure))
}

#[cfg(test)]
mod tests {
    use super::{attr_list, attribute, ibus_text};
    use zbus::zvariant::Value;

    fn signature_of(value: &Value<'_>) -> String {
        value.value_signature().to_string()
    }

    #[test]
    fn attribute_has_ibus_signature() {
        let a = Value::from(attribute(1, 2, 0, 3).unwrap());
        assert_eq!(signature_of(&a), "(sa{sv}uuuu)");
    }

    #[test]
    fn attr_list_has_ibus_signature() {
        let a = Value::from(attr_list(Vec::new()).unwrap());
        assert_eq!(signature_of(&a), "(sa{sv}av)");
    }

    #[test]
    fn ibus_text_has_ibus_signature() {
        let t = ibus_text("বাংলা").unwrap();
        assert_eq!(signature_of(&t), "(sa{sv}sv)");
    }

    #[test]
    fn ibus_text_carries_the_string() {
        // The 3rd field (index 2) of the IBusText structure is the text.
        let t = ibus_text("আমি").unwrap();
        if let Value::Structure(s) = &t {
            let fields = s.fields();
            match &fields[2] {
                Value::Str(text) => assert_eq!(text.as_str(), "আমি"),
                other => panic!("expected text at field 2, got {other:?}"),
            }
        } else {
            panic!("ibus_text must be a structure");
        }
    }
}
