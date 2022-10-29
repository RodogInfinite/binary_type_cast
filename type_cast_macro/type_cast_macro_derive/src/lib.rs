#![feature(assert_matches)]
extern crate proc_macro;

use std::{assert_matches::assert_matches};
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn;
use std::iter::repeat;

#[proc_macro_derive(DataCast,attributes(cast))]
pub fn derive_macro(input: TokenStream) -> TokenStream {

    let ast:syn::DeriveInput = syn::parse(input).unwrap();

    let name = ast.ident;

    let variants = match ast.data.clone() {
        syn::Data::Enum(data_enum) => {
            data_enum.variants.into_iter().map(|v| v.ident)
        }
        _ => panic!("DataCast only works on enums"),
    };

    // Separate the variants with arrays as types into their own Vec
    let complex_variants: Vec<proc_macro2::Ident> = variants.clone().filter(|variant| variant.to_string().ends_with("Arr")).collect();
    // Separate the variants with primitive types into their own Vec
    let variants: Vec<proc_macro2::Ident> = variants.clone().filter(|variant| !variant.to_string().ends_with("Arr")).collect();
    
    
    // Walk the Enum and get the attribute types
    fn get_cast_types(
        data_enum: syn::DataEnum, 
        cast_types: &mut Vec<proc_macro2::Ident>, 
        complex_cast_types: &mut Vec<proc_macro2::Group>,
        conversion: &mut Vec<proc_macro2::Ident>,
        complex_conversion: &mut Vec<proc_macro2::Ident>,
        number_of_array_elements: &mut Vec<proc_macro2::Literal>,) 
        {
            let mut group_bool = false;
            data_enum.clone().variants.into_iter().map(|variant| variant)
                .for_each(|variant| variant.attrs.into_iter().map(|attr|attr)
                    .for_each(|attr| attr.tokens.into_iter().map(|token|token)
                        .for_each(|token|{
                            if let proc_macro2::TokenTree::Group(group) = token {
                                // for complex types i.e [f32;2], there's an inner group that matches first and provides the type, 
                                // then the next loop through the outer group (here) provides the idents for the conversion. 
                                // group_bool tracks this so that the cast conversions remain correct
                                group_bool = false; 
                                group.stream().into_iter().map(|stream| stream)
                                    .for_each(|stream|
                                        match stream {
                                            proc_macro2::TokenTree::Ident(ref ident) => {
                                                match ident.clone().to_string().as_str() {
                                                    "f32"=> {
                                                        cast_types.push(ident.clone())
                                                    },
                                                    "f64" => {
                                                        cast_types.push(ident.clone())
                                                    },
                                                    "from_le_bytes" => {
                                                        if group_bool == true {
                                                            complex_conversion.push(ident.clone())
                                                        } else {conversion.push(ident.clone())}
                                                    },
                                                    "from_be_bytes" => {
                                                        if group_bool == true {
                                                            complex_conversion.push(ident.clone())
                                                        } else {conversion.push(ident.clone())}
                                                    },
                                                    i => panic!("Expected valid conversion or valid cast type, found {}",i),
                                                }
                                            },
                                            proc_macro2::TokenTree::Punct(ref punct) => {
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
                                                                cast_types.push(ident.clone())
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

            
    let mut cast_types = vec![];
    let mut complex_cast_types = vec![];
    let mut conversion = vec![];
    let mut complex_conversion = vec![];
    let mut number_of_array_elements = vec![];
    let data_type_names = repeat(name.clone());
    let complex_data_type_names = repeat(name.clone());
    let data_kind_name = format_ident!("{}Cast",name.clone());
    let data_kind_names = repeat(data_kind_name.clone());
    let complex_data_kind_names = repeat(data_kind_name.clone());

    
    let _ = if let syn::Data::Enum(
        data_enum
    ) = ast.data
    {   
        get_cast_types(data_enum, &mut cast_types, &mut complex_cast_types, &mut conversion, &mut complex_conversion, &mut number_of_array_elements);
    } else {
        unimplemented!();
    };
   
    let gen = quote! {
        #[derive(Debug)]
        enum #data_kind_name {
            #(#variants(#cast_types),)*
            #(#complex_variants(#complex_cast_types),)*
        }

        impl #name  {
            fn parse(self, input: &mut &[u8]) -> #data_kind_name {
                match self {
                    #(
                        #data_type_names::#variants => #data_kind_names::#variants ({
                            let (bytes, _) = input.split_at(
                                std::mem::size_of::<#cast_types>()
                            );
                            <#cast_types>::#conversion(bytes.try_into().unwrap())
                        }),
                    )*
                    /*#(
                        #complex_data_type_names::#complex_variants => #complex_data_kind_names::#complex_variants ({
                            let (bytes, rest) = input.split_at(
                                std::mem::size_of::<#cast_types>()
                            );
                            [
                                <#cast_types>::#complex_conversion(bytes.try_into().unwrap()),
                                <#cast_types>::#complex_conversion(rest.try_into().unwrap()),
                            ]
                        }),
                    )* */
                   /* */ #(
                        #complex_data_type_names::#complex_variants => #complex_data_kind_names::#complex_variants ({
                            let mut tmp_vec = std::vec::Vec::new();
                            for _ in 0..#number_of_array_elements {
                                let (bytes, rest) = input.split_at(
                                    std::mem::size_of::<#cast_types>()
                                );
                                let converted = <#cast_types>::from_le_bytes(bytes.try_into().unwrap());
                                *input = rest;
                                tmp_vec.push(converted);  
                            }
                            let out: [#cast_types;#number_of_array_elements]  = tmp_vec.into_iter().collect::<Vec<#cast_types>>().try_into().unwrap();
                            out
                        }),
                    )*
                }
            } 
        } 
    };
    gen.into()
}
