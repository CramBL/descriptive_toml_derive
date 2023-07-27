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
}

#[test]
pub fn test_derive_default() {
    let toml_string = TestStruct::default().to_string_pretty_toml();
    println!("{}", toml_string);

    assert_eq!(
        toml_string,
        r#"# A number
# Example: 42
#a = None [ u32 ] # (Uncomment and set to enable)

# A string
# Example: Hello, World!
#b = None [ String ] # (Uncomment and set to enable)

"#
    );

    // Deserialize the toml string back into a struct
    let deserialized_struct: TestStruct = toml::from_str(&toml_string).unwrap();

    // Check that the deserialized struct is the same as the original
    assert_eq!(TestStruct::default(), deserialized_struct);
}

#[test]
pub fn test_derive_with_values() {
    let test_struct = TestStruct {
        a: Some(42),
        b: Some("Hello, World!".to_string()),
    };

    let toml_string = test_struct.to_string_pretty_toml();
    println!("{toml_string}");

    assert_eq!(
        toml_string,
        r#"# A number
# Example: 42
a = 42 # [ u32 ]

# A string
# Example: Hello, World!
b = "Hello, World!" # [ String ]

"#
    );

    // Deserialize the toml string back into a struct
    let deserialized_struct: TestStruct = toml::from_str(&toml_string).unwrap();

    // Check that the deserialized struct is the same as the original
    assert_eq!(test_struct, deserialized_struct);
}
