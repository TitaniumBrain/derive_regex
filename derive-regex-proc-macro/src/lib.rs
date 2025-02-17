use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(FromRegex, attributes(regex))]
pub fn derive_from_regex(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input as DeriveInput);

    impl_derive_from_regex(&derive_input).into()
}

fn impl_derive_from_regex(derive_input: &DeriveInput) -> TokenStream {
    todo!()
}
