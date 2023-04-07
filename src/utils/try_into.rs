// try_into.rs
use proc_macro2::{TokenStream as TokenStream2, Ident};

use quote::quote;
use std::collections::HashMap;

use super::CastTypeData;

pub fn build_type_variants_map(
    cast_type_data: &CastTypeData)
    -> HashMap<String, Vec<proc_macro2::Ident>> {

// Initialize a HashMap to store the relationship between cast types and their corresponding Enum variants
let mut type_variants_map: HashMap<String, Vec<proc_macro2::Ident>> = HashMap::new();

// Handle primitive types
for (cast_type, variant) in cast_type_data.cast_types.iter().zip(cast_type_data.variants.iter()) {
    let cast_type_str = cast_type.to_string();
    type_variants_map.entry(cast_type_str).or_insert_with(Vec::new).push(variant.clone());
}

// Handle string types
for (string_type, string_variant) in cast_type_data.string_types.iter().zip(cast_type_data.string_variants.iter()) {
    let string_type_str = string_type.to_string();
    type_variants_map.entry(string_type_str).or_insert_with(Vec::new).push(string_variant.clone());
}

// Handle complex types
for (complex_cast_type, complex_variant) in cast_type_data.complex_cast_types_group.iter().zip(cast_type_data.complex_variants.iter()) {
    let complex_cast_type_str = complex_cast_type.to_string();
    type_variants_map.entry(complex_cast_type_str).or_insert_with(Vec::new).push(complex_variant.clone());
}
return type_variants_map
}

pub fn generate_try_into_impls(type_variants_map: &HashMap<String, Vec<Ident>>, data_kind_name: &Ident) -> TokenStream2 {    
    
    // Initialize a vector to store the generated TryInto trait implementations
    let mut try_into_impls = Vec::new();
    // Iterate through the type_variants_map to generate trait implementations for each cast type
    for (cast_type_str, variants) in type_variants_map {
        let cast_type: TokenStream2 = cast_type_str.parse().unwrap();
        // Generate match arms for each variant associated with the cast type
        let match_arms: TokenStream2 = variants.into_iter().map(|variant| {
            quote! {
                #data_kind_name::#variant(val) => Ok(val),
            }
        }).collect();
        // Generate the TryInto trait implementation block for the current cast type
        let impl_block = quote! {
            impl std::convert::TryInto<#cast_type> for #data_kind_name {
                type Error = String;

                fn try_into(self) -> Result<#cast_type, Self::Error> {
                    match self {
                        #match_arms
                        _ => Err(format!("Cannot convert non-compatible DataTypesCast into {}", #cast_type_str)),
                    }
                }
            }
        };
        // Add the generated implementation block to the vector
        try_into_impls.push(impl_block);
    }
    // Combine all the generated implementation blocks into a single TokenStream
    let generated_impls = quote! {
        #(#try_into_impls)*
    };
    generated_impls.into()
}
