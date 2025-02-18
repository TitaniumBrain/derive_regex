pub use derive_regex_proc_macro::FromRegex;
// use regex::{Error, Regex};

pub trait FromRegex {
    fn parse(s: &str) -> Result<Self, String>
    where
        Self: std::marker::Sized;
}

// #[derive(FromRegex)]
// #[regex(pattern = r#"(?<bar>\d+)"#)]
// struct Foo {
//     bar: u32,
// }

// #[derive(FromRegex)]
// #[regex(pattern = r#"\d+"#)]
// struct Foo2 {
//     bar: u32,
// }

// #[derive(FromRegex)]
// #[regex(pattern = r#"(?<bar>\d+)"#)]
// struct Foo3;

// struct Foo4;

// impl FromRegex for Foo4 {
//     fn parse(s: &str) -> std::result::Result<Foo4, std::string::String> {
//         let re = Regex::new("pattern here").expect("Regex validated at compile time");
//         if re.is_match(s) {
//             return Ok(Self);
//         }
//         Err("couldn't parse from haystack".to_string())
//     }
// }
