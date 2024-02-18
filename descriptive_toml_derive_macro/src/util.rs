use quote::ToTokens;
use syn::Attribute;

pub fn get_attribute<'a>(attr_name: &'a str, field: &'a syn::Field) -> Option<&'a syn::Attribute> {
    field.attrs.iter().find(|a| a.path().is_ident(attr_name))
}

pub fn attribute_value_as_string(attr: &Attribute) -> String {
    let attr_description_as_string = attr
        .meta
        .require_name_value()
        .unwrap()
        .value
        .to_token_stream()
        .to_string();
    let mut as_char_iter = attr_description_as_string.chars();
    as_char_iter.next();
    as_char_iter.next_back();
    as_char_iter.as_str().to_owned()
}

pub fn field_name_to_key_literal(field_name: &syn::Ident) -> syn::LitStr {
    let name: String = field_name.to_string();
    syn::LitStr::new(&name, field_name.span())
}

// Convert a fields type of type Option<InnerType> to InnerType as a string
pub fn field_option_type_to_inner_type_string(field_option_type: &syn::Type) -> String {
    let type_name = field_option_type.to_token_stream().to_string();
    let mut type_as_char = type_name.chars();
    for _ in 0..=7 {
        type_as_char.next();
    }
    type_as_char.next_back();
    type_as_char.as_str().to_string()
}
