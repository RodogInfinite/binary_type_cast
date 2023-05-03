use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;

use super::CastTypeData;

pub fn generated_from_str_impl(name: &Ident, cast_type_data: &mut CastTypeData) -> TokenStream2 {
    let mut match_arms = quote! {};
    for variant in &cast_type_data.variants {
        match_arms = quote! {
            #match_arms
            stringify!(#variant) => Ok(#name::#variant),
        };
    }
    for complex_variant in &cast_type_data.complex_variants {
        match_arms = quote! {
            #match_arms
            stringify!(#complex_variant) => Ok(#name::#complex_variant),
        };
    }
    for string_variant in &cast_type_data.string_variants {
        match_arms = quote! {
            #match_arms
            stringify!(#string_variant) => Ok(#name::#string_variant),
        };
    }

    quote! {
        impl std::str::FromStr for #name {
            type Err = Box<dyn std::error::Error + Send + Sync>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #match_arms
                    _ => Err("Invalid variant".into())
                }
            }
        }
    }
}
