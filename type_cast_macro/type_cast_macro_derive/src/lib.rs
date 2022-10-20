
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(DataCast,attributes(cast))]
pub fn derive_macro(input: TokenStream) -> TokenStream {

    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    //eprintln!("Input {:#?}",ast.data);
    let name = &ast.ident;

    let variants = match ast.data.clone() {
        syn::Data::Enum(enum_item) => {
            enum_item.variants.into_iter().map(|v| v.ident)
        }
        _ => panic!("DataCast only works on enums"),
    };


    
    fn cast_attribute(data_enum: syn::DataEnum) -> bool {
        data_enum.variants.into_iter().flat_map( |variant| variant.attrs.into_iter().filter_map(|attr| Some(attr.path.segments[0].ident == "cast"))).next().unwrap()
    
    }
    

    // This immediately breaks for #[cast(f32 => from_le_bytes)]
    // Walk the Enum and get the attribute types
    fn get_cast_types(data_enum: syn::DataEnum, cast_types: &mut Vec<proc_macro2::Ident>)  {
        data_enum.variants.into_iter()
            .map(|variant| variant)
                .for_each(|variant| variant.attrs.into_iter().map(|attr|attr)
                    .for_each(|attr| attr.tokens.into_iter().map(|token|token)
                        .for_each(|token|{
                            if let proc_macro2::TokenTree::Group(group) = token {
                                let mut type_ident = group.stream().into_iter().map(|stream| 
                                    match stream {
                                        proc_macro2::TokenTree::Ident(ref ident) => {
                                            eprintln!("IDENT {:#?}",ident);
                                            ident.clone()
                                        },
                                        proc_macro2::TokenTree::Group(array_group) => {
                                            array_group.stream().into_iter().map(|array_stream| 
                                                match array_stream {
                                                    proc_macro2::TokenTree::Ident(ref ident) => {
                                                        eprintln!("INNER IDENT {:#?}",ident);
                                                        ident.clone()
                                                        },
                                                        tt => panic!("Expected '' found {}",tt)
                                                }).next().unwrap()
                                        },
                                        tt => panic!("Expected '' found {}",tt),
                                    }).collect::<Vec<proc_macro2::Ident>>(); 

                                    cast_types.push(type_ident.pop().unwrap()); // Seems silly to put it into a vec just to pop it out and push it to another in another scope. Not sure of another solution right now
                            } else {
                                unimplemented!();
                            };
                        })
                        
                    )
                );
                eprintln!("CAST TYPES {:#?}",cast_types);
    } 


    
    // Not sure of a nicer way to achieve getting the values into this scope.
    let mut cast_types: Vec<proc_macro2::Ident> = vec![];

    let _ = if let syn::Data::Enum(
        data_enum
    ) = ast.data
    {   
        get_cast_types(data_enum, &mut cast_types) 

    } else {
        unimplemented!();
    };
   
    let gen = quote! {
        enum DataKind{
            #(#variants(#cast_types),)*
        }

        impl DataKind  {
            fn parse(&self) -> #name {
                
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