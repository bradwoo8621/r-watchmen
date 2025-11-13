/// Converts a string from camel case (e.g., “CamelCase”) to kebab case (e.g., “camel-case”)
/// by inserting hyphens before each uppercase letter except the first one
/// and converting all letters to lowercase.
pub fn camel_to_kebab_and_lower(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push('-');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

/// Converts a string from camel case (e.g., “CamelCase”) to kebab case (e.g., “camel-case”)
/// by inserting hyphens before each uppercase letter except the first one
/// and converting all letters to uppercase.
pub fn camel_to_kebab_and_upper(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push('-');
        }
        result.push(c.to_ascii_uppercase());
    }
    result
}

/// Prepends an ampersand to the input string
/// and converts the first character of the string to its ASCII lowercase form;
/// if the input string is empty, it simply returns a string consisting of only an ampersand.
pub fn first_lowercase_with_ampersand_prefix(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => format!(
            "&{}{}",
            first.to_ascii_lowercase(),
            chars.collect::<String>()
        ),
        None => String::from("&"),
    }
}

fn keep_same(s: &str) -> String {
    String::from(s)
}

fn upper_case(s: &str) -> String {
    s.to_uppercase()
}

fn lower_case(s: &str) -> String {
    s.to_lowercase()
}

pub enum VariantStrPattern {
    CamelCaseToKebabAndLower,
    CamelCaseToKebabAndUpper,
    AmpersandPrefix,
    KeepSame,
    UpperCase,
    LowerCase,
}

pub fn get_pattern_fn(pattern: VariantStrPattern) -> Box<dyn Fn(&str) -> String> {
    match pattern {
        VariantStrPattern::CamelCaseToKebabAndLower => Box::new(camel_to_kebab_and_lower),
        VariantStrPattern::CamelCaseToKebabAndUpper => Box::new(camel_to_kebab_and_upper),
        VariantStrPattern::AmpersandPrefix => Box::new(first_lowercase_with_ampersand_prefix),
        VariantStrPattern::KeepSame => Box::new(keep_same),
        VariantStrPattern::UpperCase => Box::new(upper_case),
        VariantStrPattern::LowerCase => Box::new(lower_case),
    }
}
