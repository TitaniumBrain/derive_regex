use derive_regex::*;
use regex;

#[test]
fn test_struct_unit_like() {
    #[derive(FromRegex, Debug, PartialEq, Eq)]
    #[regex(pattern = "foo")]
    struct Foo;

    let haystack = "foo";
    let parsed = Foo::parse(haystack);
    assert_eq!(parsed, Ok(Foo));

    let haystack = "bar";
    let parsed = Foo::parse(haystack);
    assert!(parsed.is_err());
}

#[test]
fn test_struct_tuple_like() {
    #[derive(Debug, FromRegex, PartialEq, Eq)]
    #[regex(pattern = r"(\d+) - (foo)")]
    struct Foo(i32, String);

    let haystack = "123 - foo";
    let parsed = Foo::parse(haystack);
    assert_eq!(parsed, Ok(Foo(123, "foo".to_string())));

    let haystack = "456 - bar";
    let parsed = Foo::parse(haystack);
    assert!(parsed.is_err())
}

#[test]
fn test_struct_named() {
    #[derive(Debug, FromRegex, PartialEq, Eq)]
    #[regex(pattern = r#"(?<first_name>\w+)\s+(?:\w+\.?\s+)*(?<last_name>\w+), (?<num>-?\d+)"#)]
    struct Foo {
        num: i32,
        first_name: String,
        last_name: String,
    }

    let haystack = "John M. Doe, 42";
    let parsed = Foo::parse(haystack);
    assert_eq!(
        parsed,
        Ok(Foo {
            num: 42,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        })
    )
}
