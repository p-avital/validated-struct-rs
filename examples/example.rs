fn string_validator(new: &str) -> bool {
    !new.is_empty()
}
fn hi_validator(new: &Hi) -> bool {
    new.c().len() == *new.d()
}
validated_struct::validator! {
    /// Struct documentation works as expected, just make sure they're in the right spot
    #[recursive_attrs] // attributes bellow are added to each substructure, such as Hi
    /// Documentation is an attribute, so it WILL be passed around by #[recursive_attrs]
    #[repr(C)]
    #[derive(Clone, Debug, serde::Deserialize)]
    Hello {
        /// field documentation is given to both the getter an setter for said field
        a: String where (string_validator),
        /// `b` is valid iff `d == c.len()`
        b:
        Hi {
            c: Vec<f64>,
            d: usize
        } where (hi_validator)
    }
}

#[cfg(feature = "serde_json")]
fn main() {
    use validated_struct::ValidatedMap;
    let from_str = serde_json::Deserializer::from_str;
    let mut hello =
        Hello::from_deserializer(&mut from_str(r#"{"a": "hi", "b": {"c": [0.1], "d":1}}"#))
            .unwrap();
    hello.insert("a", &mut from_str("\"\"")).unwrap_err();
    hello.insert("a", &mut from_str("\"hello\"")).unwrap();
    hello
        .insert("b", &mut from_str(r#"{"c": [0.2, 0.1], "d":3}"#))
        .unwrap_err();
    hello
        .insert("b", &mut from_str(r#"{"c": [0.2, 0.1], "d":2}"#))
        .unwrap();
    hello.insert("b/c", &mut from_str("[0.1, 0.3]")).unwrap();
}
#[cfg(not(feature = "serde_json"))]
fn main() {
    panic!("You must build this example with --features=serde_json for it to work")
}
