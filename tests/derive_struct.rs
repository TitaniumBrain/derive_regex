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
    assert_eq!(parsed, Err("couldn't parse from bar".to_string()));
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
