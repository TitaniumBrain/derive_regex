pub use derive_regex_proc_macro::FromRegex;

pub trait FromRegex {
    fn parse(s: &str) -> Self;
}
