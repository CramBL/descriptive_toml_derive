use descriptive_toml_derive::TomlConfig;
use pretty_assertions::assert_eq;

#[derive(TomlConfig, Default)]
struct TestStruct {
    #[description = "A number"]
    #[example = "42"]
    a: Option<u32>,
    #[description = "A string"]
    #[example = "Hello, World!"]
    b: Option<String>,
}

#[test]
pub fn test_derive() {
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
}
