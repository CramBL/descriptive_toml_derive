//! # Description
//! Convenience crate with a trait definition for use with the procedural derive macro `descriptive_toml_derive_macro`.
//!
//! # Example
//!
//! ```rust
//! use descriptive_toml_derive::TomlConfig;
//!
//! #[derive(TomlConfig, Default)]
//! pub struct CustomChecks {
//!     #[description = "Number of CRU Data Packets expected in the data"]
//!     #[example = "20, 500532"]
//!     cdps: Option<u32>,
//! }
//! ```
//!
//! ```rust,ignore
//! let toml_string = CustomChecks::default().to_string_pretty_toml();
//! println!({}, toml_string);
//! ```
//! Output:
//! ```toml
//! # Number of CRU Data Packets expected in the data
//! # Example: 20, 500532
//! #cdps = None [ u32 ] # (Uncomment and set to enable)
//! ```

// Re-export the derive macro
pub use descriptive_toml_derive_macro::TomlConfig;

/// This trait is derived through the [TomlConfig](descriptive_toml_derive_macro::TomlConfig) derive macro.
pub trait TomlConfig {
    /// Generates a customized pretty [String] representation of the serialized struct as a `TOML` template.
    /// The template includes comments with all possible fields and their types, that is easily edited and deserializes into the struct it was serialized from.
    /// The template also includes comments with descriptions, and examples.
    fn to_string_pretty_toml(&self) -> String;
}
