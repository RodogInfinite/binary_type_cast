
extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(DataCast,attributes(cast))]
pub fn derive_macro(input: TokenStream) -> TokenStream {

    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    //eprintln!("Input {:#?}",ast.data);
    let name = &ast.ident;

    //let variants = match ast.data.clone() {
    //    syn::Data::Enum(enum_item) => {
    //        enum_item.variants.into_iter().map(|v| v.ident)
    //    }
    //    _ => panic!("DataCast only works on enums"),
    //};

    
    //let punctuated = if let syn::Data::Enum(
    //    syn::DataEnum{ ref variants,..}
    //) = ast.data
    //{
    //   // eprintln!("Variants {:#?}",variants);
    //    variants
    //} else {
    //    unimplemented!();
    //};

    // Rewrite to use these
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
    // If using casts to write types in DataKind, then consider defaulting to use type variants that define theirs already
    /*

    #[cast(f32)]      
    IEEE754LSBSingle == IEEE754LSBSingle(f32)
    
    enum DataKind{
            #(#variants(#(#cast_types)*),)*
    }

    current path would be that #cast_types come from `let cast_types = get_cast_types(data_enum)`

    */
    fn cast_attribute(data_enum: syn::DataEnum) -> bool {
        data_enum.variants.into_iter().flat_map( |variant| variant.attrs.into_iter().filter_map(|attr| Some(attr.path.segments[0].ident == "cast"))).next().unwrap()
    
    }
    
    // not even sure this is the right path. Need it to fill with 
    //fn get_cast_types(data_enum: syn::DataEnum) -> Vec<proc_macro2::Ident> {
    //     
    //
    //            //let v: Vec<proc_macro2::Ident> = idents.iter().collect();
    //            //v
    //
    //} 


    


    let punctuated = if let syn::Data::Enum(
        data_enum
    ) = ast.data
    {

        data_enum.variants.into_iter()
        .map(|variant| variant)
            .for_each(|variant| variant.attrs.into_iter().map(|attr|attr)
                .for_each(|attr| attr.tokens.into_iter().map(|token|token)
                    .for_each(|token|{
                        if let proc_macro2::TokenTree::Group(group) = token {
                            group.stream().into_iter().map(|stream| 
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
                        } else {
                            unimplemented!();
                        };
                    }
                    )
                )
            ) ; 
    };
       //let cast =  cast_types(data_enum);
       //eprintln!("CAST {:#?}",cast);


       
       //eprintln!("ATTR IDENT? {:#?}",attr_ident);
    //} else {
    //    unimplemented!();
    //};

    //eprintln!("attrs? {:#?}",punctuated);

    /*
    for variant in punctuated.iter() {
        let x = if let syn::Variant{ref attrs, ..} = variant
        {
            for attr in attrs {
                //eprintln!("SEGMENTS {:#?}",attr.path.segments);
                if attr.path.segments[0].ident == "cast" {
                    let tokens = attr.tokens.clone().into_iter();
                    for token in tokens {
                        if let proc_macro2::TokenTree::Group(group) = token {
                            //for g in group.unwrap() {
                            let mut stream = group.stream().into_iter();
                            for s in stream {
                                eprintln!("STREAM {:#?}",s);
                                match s {
                                    proc_macro2::TokenTree::Ident(ref ident) => {
                                        //eprintln!("IDENT {:#?}",ident);
                                        //ident
                                        
                                    },
                                    proc_macro2::TokenTree::Group(array_group) => {
                                        //eprintln!("INNER GROUP? {:#?}",inner_group);
                                        let mut array_stream = array_group.stream().into_iter();
                                        let j = match array_stream.next().unwrap().clone() {
                                            proc_macro2::TokenTree::Ident(ref ident) => {
                                            eprintln!("INNER IDENT {:#?}",ident);
                                            //ident
                                            },
                                            tt => panic!("Expected '' found {}",tt)
                                        };
                                    },
                                    tt => panic!("Expected '' found {}",tt),
                                }
                            }
                        } else {
                            unimplemented!();
                        }
                    }
                    
                        
    
                    
                }
            }
        } else {
            unimplemented!();
        };
    
    }
    //eprintln!("YYYYYYYYYYYYY {}",y);
     */
   /* 
    let gen = quote! {
        enum DataKind{
            #(#variants,)*
        }

        impl DataKind  {
            fn parse(&self) -> #name {
                
            } 
        }

    };
    gen.into()
    
*/
    TokenStream::new()
    

}


#[proc_macro_attribute]
pub fn cast(attr:TokenStream, item:TokenStream) -> TokenStream {

    println!("attr: \"{}\"", attr.to_string());
    item
}