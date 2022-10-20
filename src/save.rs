/*
//prints everything nicely. Need to move to using iters. See below.

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
                            //eprintln!("STREAM {:#?}",s);
                            match s {
                                proc_macro2::TokenTree::Ident(ref ident) => {
                                    //eprintln!("IDENT {:#?}",ident);
                                    ident
                                    
                                },
                                proc_macro2::TokenTree::Group(array_group) => {
                                    //eprintln!("INNER GROUP? {:#?}",inner_group);
                                    let mut array_stream = array_group.stream().into_iter();
                                    let j = match array_stream.next().unwrap().clone() {
                                        proc_macro2::TokenTree::Ident(ref ident) => {
                                        eprintln!("INNER IDENT {:#?}",ident);
                                        },
                                        tt => panic!("Expected '' found {}",tt)
                                    };
                                },
                                tt => panic!("Expected '' found {}",tt),
                            }
                        }
                    }
                } 
            }
        }
    } else {
        unimplemented!();
    };

}



// ==================================================================
// ==================================================================



// First step in the right direction. It gives one value. Need all of them. 

fn cast_types(data_enum: syn::DataEnum) -> proc_macro2::Ident {
    data_enum.variants.into_iter()
        .flat_map(|variant| variant.attrs.into_iter()
            .flat_map(|attr| attr.tokens.into_iter()
                .flat_map(|token| 
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
                            }
                        )
                    } else {
                        unimplemented!();
                    }
                )
            )
        ).next().unwrap()
} 

*/