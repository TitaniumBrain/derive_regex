use derive_regex::*;
use regex;

#[test]
fn test_enum_unit_variant() {
    #[derive(FromRegex, Debug, PartialEq, Eq)]
    enum Foo {
        #[regex(pattern = "bar")]
        Bar,
        #[regex(pattern = ".*z.*")]
        Baz,
    }

    let haystack = "bar";
    let parsed = Foo::parse(haystack);
    assert_eq!(parsed, Ok(Foo::Bar));

    let haystack = "fooz";
    let parsed = Foo::parse(haystack);
    assert_eq!(parsed, Ok(Foo::Baz));
    // assert!(parsed.is_err());
}

#[test]
fn test_enum_tuple_variant() {
    #[derive(FromRegex, Debug, PartialEq, Eq)]
    enum Foo {
        #[regex(pattern = r#"(\d{3})\.(\d+)"#)]
        Bar(i32, i32),
        #[regex(pattern = r"(\w+)")]
        Baz(String),
    }

    let haystack = "123.4";
    let parsed = Foo::parse(haystack);
    assert_eq!(parsed, Ok(Foo::Bar(123, 4)));

    let haystack = "foo";
    let parsed = Foo::parse(haystack);
    assert_eq!(parsed, Ok(Foo::Baz("foo".to_string())));
}

#[test]
fn test_enum_named_variant() {
    #[derive(FromRegex, Debug, PartialEq, Eq)]
    enum Foo {
        #[regex(pattern = r"(?<num>\d+)$")]
        Bar { num: i32 },
        #[regex(pattern = r#"name: (?<name>\w+), age: (?<age>\d+)"#)]
        Baz { name: String, age: u32 },
    }

    let haystack = "the number is 1234";
    let parsed = Foo::parse(haystack);
    assert_eq!(parsed, Ok(Foo::Bar { num: 1234 }));

    let haystack = "name: John, age: 20 years";
    let parsed = Foo::parse(haystack);
    assert_eq!(
        parsed,
        Ok(Foo::Baz {
            name: "John".to_string(),
            age: 20
        })
    );
    // assert!(parsed.is_err());
}
