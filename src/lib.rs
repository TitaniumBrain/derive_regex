pub use derive_regex_proc_macro::FromRegex;

pub trait FromRegex {
    fn parse(s: &str) -> Result<Self, String>
    where
        Self: std::marker::Sized;
}
