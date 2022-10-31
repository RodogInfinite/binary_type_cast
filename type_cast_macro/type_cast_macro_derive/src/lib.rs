#![feature(assert_matches)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::{assert_matches::assert_matches,iter::repeat,string::String};

use syn;

#[proc_macro_derive(TypeCast, attributes(cast))]
pub fn derive_macro(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;

    // Walk the Enum and get the attribute types
    fn get_cast_types(
        data_enum: syn::DataEnum,
        cast_types: &mut Vec<proc_macro2::Ident>,
        complex_cast_types: &mut Vec<proc_macro2::Group>,
        conversion: &mut Vec<proc_macro2::Ident>,
        complex_conversion: &mut Vec<proc_macro2::Ident>,
        number_of_array_elements: &mut Vec<proc_macro2::Literal>,
        variants: &mut Vec<proc_macro2::Ident>,
        complex_variants: &mut Vec<proc_macro2::Ident>,
        string_types: &mut Vec<proc_macro2::Ident>,
        string_variants: &mut Vec<proc_macro2::Ident>,
    ) {
        // `group_bool` is needed for tracking the array types' inner group defined in the casts.
        let mut group_bool = false;
        data_enum.clone().variants.into_iter().map(|variant| variant)
                .for_each(|variant| variant.attrs.into_iter().filter(|attr|attr.path.is_ident("cast")).map(|attr|attr)
                    .for_each(|attr| attr.tokens.into_iter().map(|token|token)
                        .for_each(|token|{
                            if let proc_macro2::TokenTree::Group(group) = token {
                                // For complex types i.e [f32;2], there's an inner group that matches first and provides the type, 
                                // then the next loop through the outer group (here) provides the idents for the conversion. 
                                // group_bool tracks this so that the cast conversions remain correct
                                group_bool = false;
                                group.stream().into_iter().map(|stream| stream)
                                    .for_each(|stream|
                                        match stream {
                                            proc_macro2::TokenTree::Ident(ref ident) => {
                                                match ident.clone().to_string().as_str() {
                                                    // From little endian bytes | From big endian bytes
                                                    "from_le_bytes" | "from_be_bytes" => {
                                                        if group_bool == true {
                                                            complex_conversion.push(ident.clone());
                                                        } else {conversion.push(ident.clone())}
                                                    },
                                                    "f32"|"f64"|"i16"|"i32"|"i64"=> {
                                                        cast_types.push(ident.clone());
                                                        variants.push(variant.ident.clone());
                                                    },
                                                    "String" => {
                                                        string_types.push(ident.clone());
                                                        string_variants.push(variant.ident.clone());
                                                    }
                                                    

                                                    i => panic!("Expected valid conversion or valid cast type, found {}",i),
                                                }
                                            },
                                            proc_macro2::TokenTree::Punct(ref punct) => {
                                                // Can add verification here that they're in the order `=>`. Push them to a vec and then when it has two items, assert_match it with vec!['=','>']
                                                assert_matches!(punct.as_char(), '='| '>');
                                            },
                                            proc_macro2::TokenTree::Group(array_group) => {
                                                complex_cast_types.push(array_group.clone());
                                                eprintln!("ARRAYGROUP{:#?}",array_group);
                                                array_group.stream().into_iter().map(|array_stream| array_stream)
                                                    .for_each(|array_stream|
                                                        match array_stream {
                                                            proc_macro2::TokenTree::Ident(ref ident) => {
                                                                eprintln!("IDENT {:#?}",ident);
                                                                cast_types.push(ident.clone());
                                                                complex_variants.push(variant.ident.clone());
                                                            },
                                                            proc_macro2::TokenTree::Punct(ref punct) => {
                                                                //place holder, no use right now
                                                                ()
                                                            },
                                                            proc_macro2::TokenTree::Literal(ref literal) => {
                                                                //eprintln!("LITERAL {:#?}",literal.to_string().parse::<u16>().unwrap());
                                                                number_of_array_elements.push(literal.clone())
                                                            },
                                                            tt => panic!("Expected '' found {}",tt)
                                                        }
                                                    );
                                                group_bool = true;
                                            },
                                            tt => panic!("{}",tt),
                                        }
                                    );
                                }
                            }
                        )
                    )
                )
    }

    let mut cast_types = std::vec::Vec::new();
    let mut complex_cast_types = std::vec::Vec::new();
    let mut conversion = std::vec::Vec::new();
    let mut complex_conversion = std::vec::Vec::new();
    let mut number_of_array_elements = std::vec::Vec::new();
    let mut complex_variants: Vec<proc_macro2::Ident> = std::vec::Vec::new();
    let mut variants: Vec<proc_macro2::Ident> = std::vec::Vec::new();
    let data_type_names = repeat(name.clone());
    let complex_data_type_names = repeat(name.clone());
    let data_kind_name = format_ident!("{}Cast", name.clone());
    let data_kind_names = repeat(data_kind_name.clone());
    let complex_data_kind_names = repeat(data_kind_name.clone());
    let string_data_kind_names = name.clone();
    let mut string_types: Vec<proc_macro2::Ident> =std::vec::Vec::new(); 
    let mut string_variants: Vec<proc_macro2::Ident> = std::vec::Vec::new();
    let _ = if let syn::Data::Enum(data_enum) = ast.data {
        get_cast_types(
            data_enum,
            &mut cast_types,
            &mut complex_cast_types,
            &mut conversion,
            &mut complex_conversion,
            &mut number_of_array_elements,
            &mut variants,
            &mut complex_variants,
            &mut string_types,
            &mut string_variants,
        );
    } else {
        unimplemented!();
    };

    let gen = quote! {


        #[derive(Debug)]
        enum #data_kind_name {
            #(#variants(#cast_types),)*
            #(#complex_variants(#complex_cast_types),)*
            #(#string_variants(#string_types),)*
        }


        impl #name  {
            fn parse(self, input: &mut &[u8]) -> #data_kind_name {
                match self {
                    #(
                        #data_type_names::#variants => #data_kind_names::#variants ({
                            // `bytes` should be the size of the entire data read in; therefore, the `rest` from the `split_at` can be ignored
                            let (bytes, _) = input.split_at(
                                std::mem::size_of::<#cast_types>()
                            );
                            <#cast_types>::#conversion(bytes.try_into().unwrap())
                        }),
                    )*
                    #(
                        #complex_data_type_names::#complex_variants => #complex_data_kind_names::#complex_variants ({
                            let mut tmp_vec = std::vec::Vec::new();
                            // Convert the byte information into the defined rust type from the cast. Push them to the temporary vector in this loop. This allows for an output array to be created that has the size of the array with the expected types defined in the cast.
                            for _ in 0..#number_of_array_elements {
                                let (bytes, rest) = input.split_at(
                                    std::mem::size_of::<#cast_types>()
                                );
                                let converted = <#cast_types>::from_le_bytes(bytes.try_into().unwrap());
                                // This allows the input to become the remaining bytes for the next iteration
                                *input = rest;
                                tmp_vec.push(converted);
                            }
                            // Transform the vec into the output array
                            let out: [#cast_types;#number_of_array_elements]  = tmp_vec.into_iter().collect::<Vec<#cast_types>>().try_into().unwrap();
                            out
                        }),
                    )*
                    #(
                        #string_data_kind_names::#string_variants => #data_kind_name::#string_variants(String::from_utf8(input.to_vec()).unwrap())
                    )*
                    
                }
                
            }
        } 
    };
    gen.into()
}
