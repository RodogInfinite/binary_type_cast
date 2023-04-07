use super::CastTypeData;
use proc_macro2::TokenTree;

/// Represents the processing states for cast type attributes.
enum ProcessingState {
    Simple,  // The cast type is a primitive data type.
    Complex, // The cast type is a complex data type (e.g., an array).
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
        variant.attrs.into_iter().filter(|attr| attr.path.is_ident("cast")).for_each(|attr| {
            if let TokenTree::Group(group) = attr.tokens.into_iter().next().unwrap() {
                let mut processing_state = ProcessingState::Simple;
                let mut punctuations = Vec::new();
                // Iterate through the tokens in the attribute group
                group.stream().into_iter().for_each(|stream| {
                    match stream.clone() {
                        TokenTree::Ident(ref ident) => {
                            let ident_str = ident.to_string();
                            match ident_str.as_str() {
                                // Match the conversion methods
                                "from_le_bytes" | "from_be_bytes" => match processing_state {
                                    ProcessingState::Simple => cast_type_data.conversion.push(ident.clone()),
                                    ProcessingState::Complex => cast_type_data.complex_conversion.push(ident.clone()),
                                },
                                // Match the basic data types
                                "f32" | "f64" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => {
                                    cast_type_data.cast_types.push(ident.clone());
                                    cast_type_data.variants.push(variant.ident.clone());
                                },
                                // Match the String data type
                                "String" => {
                                    cast_type_data.string_types.push(ident.clone());
                                    cast_type_data.string_variants.push(variant.ident.clone());
                                },
                                // Handle invalid conversion or cast types
                                i => errors.extend(syn::Error::new(ident.span(), format!("Expected valid conversion or valid cast type, found {}", i)).to_compile_error()),
                            }
                        },
                        // Ensure that the only punctuation between the conversion method and the data type are consistent with `=>`
                        TokenTree::Punct(ref punct) => {
                            if punct.as_char() == '=' || punct.as_char() == '>' {
                                punctuations.push(punct.as_char());
                            } else {
                                errors.extend(syn::Error::new(punct.span(), format!("Expected '=' or '>', found '{}'", punct.as_char())).to_compile_error());
                            }
                        },
                        // Handle complex cast types (arrays)
                        TokenTree::Group(array_group) => {
                            cast_type_data.complex_cast_types_group.push(array_group.clone());
                            processing_state = ProcessingState::Complex;

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
                        },
                        // Handle unexpected tokens in the attribute group
                        tt => errors.extend(syn::Error::new(stream.span(), tt.to_string()).to_compile_error()),
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
