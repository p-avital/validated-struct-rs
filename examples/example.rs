fn string_validator(new: &str) -> bool {
    !new.is_empty()
}
fn hi_validator(new: &Hi) -> bool {
    new.c().len() == *new.d()
}
validated_struct::validator! {
    #[recursive_attrs] // attributes bellow are added to each substructure, such as Hi
    #[repr(C)]
    #[derive(Clone, Debug, serde::Deserialize)]
    Hello {
        a: String where (string_validator),
        b: Hi {
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
}
#[cfg(not(feature = "serde_json"))]
fn main() {
    panic!("You must build this example with --feature=serde_json for it to work")
}
