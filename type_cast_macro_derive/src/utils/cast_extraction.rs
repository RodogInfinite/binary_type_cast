use super::CastTypeData;
use proc_macro2::TokenTree;

enum ProcessingState {
    Simple,
    Complex,
}

pub fn get_cast_types(data_enum: syn::DataEnum, cast_type_data: &mut CastTypeData) -> Vec<proc_macro2::TokenTree> {
    let mut errors = Vec::new();
    
    data_enum.variants.into_iter().for_each(|variant| {
        variant.attrs.into_iter().filter(|attr| attr.path.is_ident("cast")).for_each(|attr| {
            if let TokenTree::Group(group) = attr.tokens.into_iter().next().unwrap() {
                let mut processing_state = ProcessingState::Simple;

                group.stream().into_iter().for_each(|stream| {
                    match stream.clone() {
                        TokenTree::Ident(ref ident) => {
                            let ident_str = ident.to_string();
                            match ident_str.as_str() {
                                "from_le_bytes" | "from_be_bytes" => match processing_state {
                                    ProcessingState::Simple => cast_type_data.conversion.push(ident.clone()),
                                    ProcessingState::Complex => cast_type_data.complex_conversion.push(ident.clone()),
                                },
                                "f32" | "f64" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => {
                                    cast_type_data.cast_types.push(ident.clone());
                                    cast_type_data.variants.push(variant.ident.clone());
                                },
                                "String" => {
                                    cast_type_data.string_types.push(ident.clone());
                                    cast_type_data.string_variants.push(variant.ident.clone());
                                },
                                i => errors.extend(syn::Error::new(ident.span(), format!("Expected valid conversion or valid cast type, found {}", i)).to_compile_error()),
                            }
                        },
                        TokenTree::Punct(ref punct) => {
                            match punct.as_char() {
                                '=' | '>' => {}
                                _ => errors.extend(syn::Error::new(punct.span(), format!("Expected '=' or '>', found '{}'", punct.as_char())).to_compile_error()),
                            }
                        },
                        TokenTree::Group(array_group) => {
                            cast_type_data.complex_cast_types_group.push(array_group.clone());
                            processing_state = ProcessingState::Complex;

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
                                    tt => errors.extend(syn::Error::new(array_stream.span(), format!("Expected '' found {}", tt)).to_compile_error()),
                                }
                            });
                        },
                        tt => errors.extend(syn::Error::new(stream.span(), tt.to_string()).to_compile_error()),
                    }
                });
            }
        });
    });
    
    errors
}
