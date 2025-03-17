use proc_macro::{self, TokenStream};
use quote::quote;
use regex::Regex;
use std::collections::HashSet;
use syn::{
    self, parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, Attribute,
    DataEnum, DataStruct, DeriveInput, ExprLit, Lit, LitStr, Meta, Path, Token,
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

/// The configuration options for the #[regex(...)] attribute
struct FromRegexAttr {
    /// The pattern to match for the struct/variant
    pattern_literal: LitStr,
}

fn impl_derive_from_regex_for_struct(
    derive_input: &DeriveInput,
    data: &DataStruct,
) -> proc_macro2::TokenStream {
    let ident = &derive_input.ident;

    let attr_args = match find_regex_attr(&derive_input.attrs) {
        Some(attr) => match get_regex_attr(derive_input, attr) {
            Ok(attr_args) => attr_args,
            Err(err) => return err.into_compile_error(),
        },

        None => {
            return syn::Error::new(derive_input.ident.span(), "missing regex attribute")
                .into_compile_error()
        }
    };

    // needed to prevent the String from being dropped too soon
    let pattern_string = attr_args.pattern_literal.value();
    let pattern = pattern_string.as_str();

    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => {
            return syn::Error::new_spanned(attr_args.pattern_literal, format!("{}", e))
                .into_compile_error()
        }
    };

    let return_type: Path = derive_input.ident.clone().into();

    let impl_block: proc_macro2::TokenStream = match &data.fields {
        syn::Fields::Named(fields_named) => {
            impl_for_named_struct(fields_named, &re, pattern, return_type)
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            impl_for_tuple_struct(fields_unnamed, &re, pattern, return_type)
        }
        syn::Fields::Unit => impl_for_unit_struct(pattern, return_type),
    };

    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();
    quote! {
        impl #impl_generics FromRegex for #ident #ty_generics #where_clause {
            fn parse(input: &str) -> std::result::Result<#ident, std::string::String> {
                #impl_block
                Err(format!{"couldn't parse from \"{}\"", input}.to_string())
            }
        }
    }
}

/// Find the `#[regex(...)]` attribite in the item's attributes
fn find_regex_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("regex"))
}

/// Return the parameters of the `#[regex(...)]` attribute as a `FromRegexAttr `instance
fn get_regex_attr(
    derive_input: &DeriveInput,
    attr: &Attribute,
) -> Result<FromRegexAttr, syn::Error> {
    let mut pattern_literal: Option<LitStr> = None;

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
                                return Err(syn::Error::new(
                                    meta_span,
                                    "expcted `pattern = \"...\"` argument",
                                ));
                            }
                        }
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "unsupported attribute argument",
                        ))
                    }
                }
            }
        }
        Err(err) => return Err(err),
    }

    let pattern_literal = match pattern_literal {
        Some(p) => p,
        None => {
            return Err(syn::Error::new(
                derive_input.ident.span(),
                "expcted `pattern = \"...\"` argument",
            ));
        }
    };

    Ok(FromRegexAttr { pattern_literal })
}

fn impl_for_named_struct(
    fields_named: &syn::FieldsNamed,
    re: &Regex,
    pattern: &str,
    return_type: Path,
) -> proc_macro2::TokenStream {
    let expected_cap_groups: HashSet<String> = fields_named
        .named
        .iter()
        .filter_map(|field| field.ident.clone().map(|name| name.to_string()))
        .collect();
    let actual_cap_groups: HashSet<String> = re
        .capture_names()
        .skip(1)
        .filter_map(|name| name.map(|name| name.to_string()))
        .collect();

    // struct fields not captured in a group
    let missing_groups: HashSet<String> = expected_cap_groups
        .difference(&actual_cap_groups)
        .cloned()
        .collect();

    // capturing groups not matching any struct field
    let extra_groups: HashSet<String> = actual_cap_groups
        .difference(&expected_cap_groups)
        .cloned()
        .collect();

    let mut group_errors = Vec::new();

    if !missing_groups.is_empty() {
        group_errors.push(
            syn::Error::new_spanned(
                fields_named,
                format!(
                    "missing capture groups for struct fields: {}",
                    missing_groups
                        .into_iter()
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            )
            .into_compile_error(),
        );
    }
    if !extra_groups.is_empty() {
        group_errors.push(
            syn::Error::new_spanned(
                fields_named,
                format!(
                    "these capture groups don't match any struct fields: {}",
                    extra_groups.into_iter().collect::<Vec<String>>().join(", ")
                ),
            )
            .into_compile_error(),
        );
    }

    if !group_errors.is_empty() {
        return quote! {#(#group_errors)*};
    }

    let field_exprs = fields_named.named.iter().map(|field| {
        let field_ident = field.ident.clone().expect("field of named struct");
        let field_name = format!("{field_ident}");
        let field_ty = &field.ty;

        quote! {
            #field_ident: caps[#field_name].parse::<#field_ty>().map_err(|err| err.to_string())?
        }
    });

    quote! {
        {
            use once_cell::sync::Lazy;
            static RE: Lazy<::regex::Regex> = Lazy::new(|| ::regex::Regex::new(#pattern).expect("Regex validated at compile time"));
            if let Some(caps) = RE.captures(input) {
                return Ok(#return_type{ #(#field_exprs),* })
            }
        }
    }
}

fn impl_for_tuple_struct(
    fields_unnamed: &syn::FieldsUnnamed,
    re: &Regex,
    pattern: &str,
    return_type: Path,
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
        {
            use once_cell::sync::Lazy;
            static RE: Lazy<::regex::Regex> = Lazy::new(|| ::regex::Regex::new(#pattern).expect("Regex validated at compile time"));
            if let Some(caps) = RE.captures(input) {
                return Ok(#return_type( #(#field_exprs),* ))
            }
       }
    }
}

fn impl_for_unit_struct(pattern: &str, return_type: Path) -> proc_macro2::TokenStream {
    quote! {
        {
            use once_cell::sync::Lazy;
            static RE: Lazy<::regex::Regex> = Lazy::new(|| ::regex::Regex::new(#pattern).expect("Regex validated at compile time"));
            if RE.is_match(input) {
                return Ok(#return_type);
            }
        }
    }
}

fn impl_derive_from_regex_for_enum(
    derive_input: &DeriveInput,
    data: &DataEnum,
) -> proc_macro2::TokenStream {
    let enum_ident = &derive_input.ident;

    let impls = data
        .variants
        .iter()
        .map(|variant| -> proc_macro2::TokenStream {
            let attr_args = match find_regex_attr(&variant.attrs) {
                Some(attr) => match get_regex_attr(derive_input, attr) {
                    Ok(attr_args) => attr_args,
                    Err(err) => return err.into_compile_error(),
                },

                None => {
                    return syn::Error::new(variant.ident.span(), "missing regex attribute")
                        .into_compile_error()
                }
            };

            // needed to prevent the String from being dropped too soon
            let pattern_string = attr_args.pattern_literal.value();
            let pattern = pattern_string.as_str();

            let re = match Regex::new(pattern) {
                Ok(re) => re,
                Err(e) => {
                    return syn::Error::new_spanned(attr_args.pattern_literal, format!("{}", e))
                        .into_compile_error()
                }
            };

            let variant_ident = &variant.ident;
            let return_type = parse_quote!(#enum_ident::#variant_ident);

            match &variant.fields {
                syn::Fields::Named(fields_named) => {
                    impl_for_named_struct(fields_named, &re, pattern, return_type)
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    impl_for_tuple_struct(fields_unnamed, &re, pattern, return_type)
                }
                syn::Fields::Unit => impl_for_unit_struct(pattern, return_type),
            }
        });

    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();
    quote! {
        impl #impl_generics FromRegex for #enum_ident #ty_generics #where_clause {
            fn parse(input: &str) -> std::result::Result<#enum_ident, std::string::String> {
                #(#impls)*
                Err(format!{"couldn't parse from \"{}\"", input}.to_string())
            }
        }
    }
}
