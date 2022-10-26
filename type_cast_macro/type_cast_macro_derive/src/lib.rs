#![feature(assert_matches)]
extern crate proc_macro;

use core::num;
use std::{assert_matches::assert_matches};
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn;
use std::iter::repeat;

#[proc_macro_derive(DataCast,attributes(cast))]
pub fn derive_macro(input: TokenStream) -> TokenStream {

    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    //eprintln!("Input {:#?}",ast.data);
    let name = ast.ident;
    //let name: Vec<proc_macro2::Ident> = name.collect();

    let variants = match ast.data.clone() {
        syn::Data::Enum(data_enum) => {
            data_enum.variants.into_iter().map(|v| v.ident)
        }
        _ => panic!("DataCast only works on enums"),
    };

    let complex_variants: Vec<proc_macro2::Ident> = variants.clone().filter(|variant| variant.to_string().ends_with("Arr")).collect();
    // Allow variants to be used multiple times in the quote
    let variants: Vec<proc_macro2::Ident> = variants.clone().filter(|variant| !variant.to_string().ends_with("Arr")).collect();
    
    eprintln!("VARIANTS {:#?}",variants);
    
    fn cast_attribute(data_enum: syn::DataEnum) -> bool {
        data_enum.variants.into_iter().flat_map( |variant| variant.attrs.into_iter().filter_map(|attr| Some(attr.path.segments[0].ident == "cast"))).next().unwrap()
    
    }
    
    

    // Walk the Enum and get the attribute types
    fn get_cast_types(
        data_enum: syn::DataEnum, 
        cast_types: &mut Vec<proc_macro2::Ident>, 
        complex_cast_types: &mut Vec<proc_macro2::Group>,
        conversion: &mut Vec<proc_macro2::Ident>,
        complex_conversion: &mut Vec<proc_macro2::Ident>) {
        let mut group_bool = false;
        data_enum.variants.into_iter()
            .map(|variant| variant)
                .for_each(|variant| variant.attrs.into_iter().map(|attr|attr)
                    .for_each(|attr| attr.tokens.into_iter().map(|token|token)
                        .for_each(|token|{
                            if let proc_macro2::TokenTree::Group(group) = token {
                                eprintln!("GROUP {:#?}",group);
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
                                                //eprintln!("PUNCT {:#?}",punct);
                                                assert_matches!(punct.as_char(), '='| '>');
                                            },
                                            proc_macro2::TokenTree::Group(array_group) => {
                                                eprintln!("Input {:#?}",array_group);
                                                complex_cast_types.push(array_group);
                                                group_bool = true;
                                                
                                            },
                                            tt => panic!("Expected '' found {}",tt),
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
    let data_type_names = repeat(name.clone());
    let complex_data_type_names = data_type_names.clone();
    let data_kind_name = format_ident!("{}Cast",name.clone());
    let data_kind_names = repeat(data_kind_name.clone());
    let complex_data_kind_names = data_kind_names.clone();

    
    let _ = if let syn::Data::Enum(
        data_enum
    ) = ast.data
    {   
        get_cast_types(data_enum, &mut cast_types, &mut complex_cast_types, &mut conversion, &mut complex_conversion); 
        eprintln!("CONVERSION {:#?}",conversion);
        eprintln!("COMPLEX CONVERSION {:#?}",complex_conversion);
    } else {
        unimplemented!();
    };
   
    let gen = quote! {
        enum #data_kind_name {
            #(#variants(#cast_types),)*
            #(#complex_variants(#complex_cast_types),)*
        }

        impl #data_kind_name  {
            fn parse(&self, input: &mut &[u8]) -> #data_kind_name {
                match self{
                    #(#data_type_names::#variants => #data_kind_names::#variants ({
                        let (bytes, _) = input.split_at(
                            std::mem::size_of::<#cast_types>()
                        );
                        <#cast_types>::#conversion(bytes.try_into().unwrap())

                    }),
                    )*
                    
                }
            } 
            fn parse_complex(&self, input: &mut &[u8]) -> #data_kind_name {
                match self{
                    #(#complex_data_type_names::#complex_variants => #complex_data_kind_names::#complex_variants ({
                        let (bytes, rest) = input.split_at(
                            std::mem::size_of::<#cast_types>()
                        );
                        [
                            <#cast_types>::#complex_conversion(bytes.try_into().unwrap()),
                            <#cast_types>::#complex_conversion(rest.try_into().unwrap()),
                        ]

                    }),
                    )*
                }
                }
            } 
    };
    gen.into()
    

    //TokenStream::new()
    

}


#[proc_macro_attribute]
pub fn cast(attr:TokenStream, item:TokenStream) -> TokenStream {

    println!("attr: \"{}\"", attr.to_string());
    item
}