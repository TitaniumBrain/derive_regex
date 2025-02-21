pub use derive_regex_proc_macro::FromRegex;

/// A trait for parsing types from a string using a regular expression
///
/// Derive this trait on a struct or enum to add the ability to parse it from a string using a regular expression.
///
/// Tuple-like structs or enum variants parse their fields from numbered regex capture groups, i.e. the groups must be defined in the same order as the fields.
/// Named structs or enum variants use named capture groups with the same name as the fields, meaning their oreder doesn't matter.
///
/// The fields must be of a type that implements FromStr.
///
/// # Example
///
/// ```rust
/// use derive_regex::FromRegex;
///
/// #[derive(Debug, FromRegex, PartialEq)]
/// #[regex(pattern = r"(?P<name>\w+),\s*(?P<age>\d+)")]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
///     let input = "Alice, 30";
///     let person = Person::parse(input).unwrap();
///     assert_eq!(person, Person {
///         name: "Alice".to_string(),
///         age: 30,
///     });
/// ```
pub trait FromRegex {
    fn parse(input: &str) -> Result<Self, String>
    where
        Self: std::marker::Sized;
}
