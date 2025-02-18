use proc_macro::{self, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, spanned::Spanned, Attribute, DataEnum,
    DataStruct, DeriveInput, ExprLit, Lit, LitStr, Meta, Token,
};

#[proc_macro_derive(FromRegex, attributes(regex))]
pub fn derive_from_regex(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input as DeriveInput);

    impl_derive_from_regex(&derive_input).into()
}

fn impl_derive_from_regex(derive_input: &DeriveInput) -> proc_macro2::TokenStream {
    match &derive_input.data {
        syn::Data::Struct(data_struct) => {
            impl_derive_from_regex_for_struct(derive_input, data_struct)
        }
        syn::Data::Enum(data_enum) => impl_derive_from_regex_for_enum(derive_input, data_enum),
        syn::Data::Union(_) => syn::Error::new(
            derive_input.ident.span(),
            "FromRegex cannot be derived for unions",
        )
        .to_compile_error(),
    }
}

fn impl_derive_from_regex_for_struct(
    derive_input: &DeriveInput,
    data: &DataStruct,
) -> proc_macro2::TokenStream {
    let ident = &derive_input.ident;
    let mut pattern_literal: Option<LitStr> = None;

    match get_regex_attr(&derive_input.attrs) {
        Some(attr) => {
            match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_separated_nonempty) {
                Ok(nested) => {
                    for meta in nested {
                        let meta_span = meta.span();
                        match meta {
                            // #[regex(pattern = "...")]
                            Meta::NameValue(name_value) if name_value.path.is_ident("pattern") => {
                                match name_value.value {
                                    syn::Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit_value),
                                        ..
                                    }) => pattern_literal = Some(lit_value),
                                    _ => {
                                        // TODO: make span cover the whole meta item, not just the name
                                        return syn::Error::new(
                                            meta_span,
                                            "expcted `pattern = \"...\"` argument",
                                        )
                                        .into_compile_error();
                                    }
                                }
                            }
                            _ => {
                                return syn::Error::new_spanned(
                                    meta,
                                    "unsupported attribute argument",
                                )
                                .into_compile_error()
                            }
                        }
                    }
                }
                Err(err) => return err.into_compile_error(),
            }
        }

        None => {
            return syn::Error::new(derive_input.ident.span(), "missing regex attribute")
                .into_compile_error()
        }
    };

    let pattern_literal = match pattern_literal {
        Some(p) => p,
        None => {
            return syn::Error::new(
                derive_input.ident.span(),
                "expcted `pattern = \"...\"` argument",
            )
            .into_compile_error();
        }
    };

    // needed to prevent the STring from being dropped too soon
    let pattern_string = pattern_literal.value();
    let pattern = pattern_string.as_str();

    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => {
            return syn::Error::new_spanned(pattern_literal, format!("{}", e)).into_compile_error()
        }
    };

    let impl_block: proc_macro2::TokenStream = match &data.fields {
        syn::Fields::Named(fields_named) => impl_for_named_struct(fields_named, &re, pattern),
        syn::Fields::Unnamed(fields_unnamed) => impl_for_tuple_struct(fields_unnamed, &re, pattern),
        syn::Fields::Unit => impl_for_unit_struct(pattern),
    };

    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();
    quote! {
        impl #impl_generics FromRegex for #ident #ty_generics #where_clause {
            fn parse(s: &str) -> std::result::Result<#ident, std::string::String> {
                #impl_block
            }
        }
    }
}

fn impl_for_named_struct(
    fields_named: &syn::FieldsNamed,
    re: &Regex,
    pattern: &str,
) -> proc_macro2::TokenStream {
    todo!()
}

fn impl_for_tuple_struct(
    fields_unnamed: &syn::FieldsUnnamed,
    re: &Regex,
    pattern: &str,
) -> proc_macro2::TokenStream {
    let actual_groups = re.captures_len() - 1;
    let expected_groups = fields_unnamed.unnamed.len();

    if actual_groups > expected_groups {
        return syn::Error::new_spanned(
            fields_unnamed,
            format!("too many capturing groups: expected {expected_groups}, got {actual_groups}"),
        )
        .into_compile_error();
    } else if expected_groups > actual_groups {
        return syn::Error::new_spanned(
            fields_unnamed,
            format!("missing capturing groups: expected {expected_groups}, got {actual_groups}"),
        )
        .into_compile_error();
    }

    let field_exprs = fields_unnamed.unnamed.iter().enumerate().map(|(i, field)| {
        let index = i + 1;
        let field_ty = &field.ty;
        quote! {
            caps[#index].parse::<#field_ty>().map_err(|err| err.to_string())?

        }
    });

    quote! {
        let re = ::regex::Regex::new(#pattern).expect("Regex validated at compile time");
        let caps = re.captures(s).ok_or("pattern did not match")?;

        return Ok(Self( #(#field_exprs),* ))
    }
}

fn impl_for_unit_struct(pattern: &str) -> proc_macro2::TokenStream {
    quote! {
        let re = ::regex::Regex::new(#pattern).expect("Regex validated at compile time");
        if re.is_match(s) {
            return Ok(Self);
        }
        Err(format!{"couldn't parse from {}", s}.to_string())
    }
}

fn get_regex_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("regex"))
}

fn impl_derive_from_regex_for_enum(
    derive_input: &DeriveInput,
    data: &DataEnum,
) -> proc_macro2::TokenStream {
    todo!()
}
