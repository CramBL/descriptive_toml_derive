use descriptive_toml_derive::TomlConfig;
use pretty_assertions::assert_eq;
use serde_derive::{Deserialize, Serialize};

#[derive(TomlConfig, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct TestStruct {
    #[description = "A number"]
    #[example = "42"]
    a: Option<u32>,
    #[description = "A string"]
    #[example = "Hello, World!"]
    b: Option<String>,

    #[description = "A nested struct"]
    #[example = "NestedTestStruct { a: Some(42) }"]
    c: Option<NestedTestStruct>,
}

#[derive(TomlConfig, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct NestedTestStruct {
    #[description = "A number"]
    #[example = "42"]
    a: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ExpectStruct {
    a: Option<u32>,
    b: Option<String>,
    c: Option<ExpectNestedStruct>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ExpectNestedStruct {
    a: Option<u32>,
}

#[test]
pub fn test_derive_expect_some() {
    // Some values
    let some_expect = ExpectStruct {
        a: Some(42),
        b: Some("Hello, World!".to_string()),
        c: Some(ExpectNestedStruct { a: Some(42) }),
    };

    let toml = toml::to_string(&some_expect).unwrap();
    println!("{toml}");
}

#[test]
pub fn test_derive_nested_some() {
    // Some values
    let some_test = TestStruct {
        a: Some(42),
        b: Some("Hello, World!".to_string()),
        c: Some(NestedTestStruct { a: Some(42) }),
    };

    let toml = some_test.to_string_pretty_toml();
    println!("{toml}");
}
