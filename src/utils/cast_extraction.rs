use super::CastTypeData;
use proc_macro2::{Span, TokenTree};
use syn::Ident;

/// Represents the processing states for cast type attributes.
enum ProcessingState {
    None,
    AwaitingType,
    AwaitingConversion,
}

/// Parses the custom `cast` attribute for each variant of the given data enum
/// and updates the provided `cast_type_data` with the parsed information.
///
/// This function iterates through the enum variants, filtering for the "cast"
/// attribute. For each "cast" attribute found, it processes its tokens and
/// updates the `cast_type_data` accordingly.
///
/// # Arguments
///
/// * `data_enum` - The data enum to process for the custom `cast` attribute.
/// * `cast_type_data` - A mutable reference to the `CastTypeData` struct, where
///   the parsed information will be stored.
///
/// # Returns
///
/// A vector of `proc_macro2::TokenTree` containing any errors encountered during parsing.
pub fn get_cast_types(
    data_enum: syn::DataEnum,
    cast_type_data: &mut CastTypeData,
) -> Vec<proc_macro2::TokenTree> {
    let mut errors = Vec::new();

    // Iterate over each variant in the enum
    data_enum.variants.into_iter().for_each(|variant| {
        // Iterate over the attributes of the variant, filtering only the ones with the "cast" identifier
        
        let mut current_conversion = String::new();
        variant.attrs.into_iter().filter(|attr| attr.path.is_ident("cast")).for_each(|attr| {
            let mut processing_state = ProcessingState::None;
        
            if let TokenTree::Group(group) = attr.tokens.into_iter().next().unwrap() {
                
                let mut punctuations = Vec::new();
                // Iterate through the tokens in the attribute group
                group.stream().into_iter().for_each(|stream| {
                    match (stream.clone(),&processing_state) {
                        (TokenTree::Ident(ref ident),ProcessingState::None) => {
                            let ident_str = ident.to_string();
                            match ident_str.as_str() {
                                // Match the conversion methods
                                
                                "from_le_bytes" => {
                                    current_conversion = "le_".to_string();
                                    processing_state = ProcessingState::AwaitingType;

                                },
                                "from_be_bytes" => {
                                    current_conversion = "be_".to_string();
                                    processing_state = ProcessingState::AwaitingType;
                                },
                                // Match the String data type
                                "String" => {
                                    cast_type_data.string_types.push(ident.clone());
                                    cast_type_data.string_variants.push(variant.ident.clone());
                                    
                                    processing_state = ProcessingState::None;
                                },
                               
                                // Handle invalid conversion or cast types
                                i => errors.extend(syn::Error::new(ident.span(), format!("Expected valid conversion or valid cast type, found {}", i)).to_compile_error()),
                            }
                        },
                        
                        // Ensure that the only punctuation between the conversion method and the data type are consistent with `=>`
                        (TokenTree::Punct(ref punct),ProcessingState::AwaitingType) => {
                            if punct.as_char() == '=' || punct.as_char() == '>' {
                                punctuations.push(punct.as_char());
                                
                            } else {
                                errors.extend(syn::Error::new(punct.span(), format!("Expected '=' or '>', found '{}'", punct.as_char())).to_compile_error());
                            }
                            if punctuations == ['=','>'] {
                                processing_state = ProcessingState::AwaitingConversion;
                            }
                        },
                        (TokenTree::Ident(ref ident),ProcessingState::AwaitingConversion) => {
                            let ident_str = ident.to_string();
                            match ident_str.as_str() {
                               
                                // Match the basic data types
                                "f32" | "f64" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => {
                                    cast_type_data.cast_types.push(ident.clone());
                                    cast_type_data.variants.push(variant.ident.clone());
                                    cast_type_data.conversion.push(Ident::new(&current_conversion.clone(),Span::call_site()));
                                    current_conversion.clear();
                                    processing_state = ProcessingState::None;

                                },
                                
                                // Handle invalid conversion or cast types
                                i => errors.extend(syn::Error::new(ident.span(), format!("Expected valid conversion or valid cast type, found {}", i)).to_compile_error()),
                            }
                        },
                        // Handle complex cast types (arrays)
                        (TokenTree::Group(array_group),ProcessingState::AwaitingConversion) => {
                            cast_type_data.complex_cast_types_group.push(array_group.clone());
                            

                            // Iterate through the tokens in the array group
                            array_group.stream().into_iter().for_each(|array_stream| {
                                match array_stream.clone() {
                                    TokenTree::Ident(ref ident) => {
                                        cast_type_data.complex_cast_types.push(ident.clone());
                                        cast_type_data.complex_variants.push(variant.ident.clone());
                                    },
                                    TokenTree::Punct(_) => (),
                                    TokenTree::Literal(ref literal) => {
                                        cast_type_data.number_of_array_elements.push(literal.clone());
                                    },
                                    // Handle unexpected tokens in the array group
                                    tt => errors.extend(syn::Error::new(array_stream.span(), format!("Expected '' found {}", tt)).to_compile_error()),
                                }
                            });
                            cast_type_data.complex_conversion.push(Ident::new(&current_conversion.clone(),Span::call_site()));
                                    current_conversion.clear();
                            processing_state = ProcessingState::None;
                        },
                        // Handle unexpected tokens in the attribute group
                        (tt,_) => errors.extend(syn::Error::new(stream.span(), tt.to_string()).to_compile_error()),
                    }
                });
                let valid_arrows = punctuations.windows(2).all(|window| window[0] == '=' && window[1] == '>');
                if !valid_arrows {
                    errors.extend(syn::Error::new(group.span(), "Expected '=>', found different order or extra characters").to_compile_error());
                } else {
                    punctuations.clear();
                }
            }
        });
    });

    // Return the errors encountered during processing
    errors
}
