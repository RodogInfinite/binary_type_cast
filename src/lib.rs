extern crate proc_macro;
mod utils;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use utils::from_str::generated_from_str_impl;

use std::iter::repeat;
use syn;

use crate::utils::{
    cast_extraction::get_cast_types,
    try_into::{build_type_variants_map, generate_try_into_impls},
    CastTypeData,
};

/// # Example
/// ```
/// #[derive(Clone, Copy, Debug, Serialize, Deserialize, TypeCast)]
/// pub enum DataTypes {
///     // 4 bytes
///     #[cast(from_le_bytes => f32)]
///     IEEE754LSBSingle,
///     // 8 bytes
///     #[cast(from_le_bytes => f64)]
///     IEEE754LSBDouble,
///     // [4 bytes, 4 bytes]
///     #[cast(from_le_bytes => [f32;2])]
///     IEEE754LSBSingleArr2,
///     // [8 bytes, 8 bytes]
///     #[cast(from_le_bytes => [f64;2])]
///     IEEE754LSBDoubleArr2,
///     // [4 bytes, 4 bytes]
///     #[cast(from_be_bytes => [f32;2])]
///     IEEE754MSBSingleArr2,
///     // [8 bytes, 8 bytes]
///     #[cast(from_be_bytes => [f64;2])]
///     IEEE754MSBDoubleArr2,
///     // [4 bytes, 4 bytes, 4 bytes]
///     #[cast(from_le_bytes => [f32;3])]
///     IEEE754LSBSingleArr3,
///     // [8 bytes, 8 bytes, 8 bytes]
///     #[cast(from_le_bytes => [f64;3])]
///     IEEE754LSBDoubleArr3,
///     #[cast(String)]
///     ASCIIString,
/// }
/// ```
///
/// Generated at compile time:
///
/// ```
/// #[derive(Clone, Copy, Debug, Serialize, Deserialize, TypeCast)]
/// pub enum DataTypes {
///     IEEE754LSBSingle(f32),
///     IEEE754LSBDouble(f64),
///     IEEE754LSBSingleArr2([f32;2]),
///     IEEE754LSBDoubleArr2([f64;2]),
///     IEEE754MSBSingleArr2([f32;2]),
///     IEEE754MSBDoubleArr2([f64;2]),
///     IEEE754LSBSingleArr3([f32;3]),
///     IEEE754LSBDoubleArr3([f64;3]),
///     ASCIIString(String),
/// }
///
///
/// impl DataTypes {
///     pub fn parse(self, input: &mut &[u8]) -> DataTypesCast {
///         match self {
///             DataTypes::IEEE754LSBSingle => {
///                 DataTypesCast::IEEE754LSBSingle({
///                     let (bytes, _) = input.split_at(std::mem::size_of::<f32>());
///                     <f32>::from_le_bytes(bytes.try_into().unwrap())
///                 })
///             }
///             DataTypes::IEEE754LSBDouble => {
///                 DataTypesCast::IEEE754LSBDouble({
///                     let (bytes, _) = input.split_at(std::mem::size_of::<f64>());
///                     <f64>::from_le_bytes(bytes.try_into().unwrap())
///                 })
///             }
///             DataTypes::IEEE754LSBSingleArr2 => {
///                 DataTypesCast::IEEE754LSBSingleArr2({
///                     let mut tmp_vec = std::vec::Vec::new();
///                     for _ in 0..2 {
///                         let (bytes, rest) = input
///                             .split_at(std::mem::size_of::<f32>());
///                         let converted = <f32>::from_le_bytes(
///                             bytes.try_into().unwrap(),
///                         );
///                         *input = rest;
///                         tmp_vec.push(converted);
///                     }
///                     let out: [f32; 2] = tmp_vec
///                         .into_iter()
///                         .collect::<Vec<f32>>()
///                         .try_into()
///                         .unwrap();
///                     out
///                 })
///             }
///             DataTypes::IEEE754LSBDoubleArr2 => {
///                 DataTypesCast::IEEE754LSBDoubleArr2({
///                     let mut tmp_vec = std::vec::Vec::new();
///                     for _ in 0..2 {
///                         let (bytes, rest) = input
///                             .split_at(std::mem::size_of::<f64>());
///                         let converted = <f64>::from_le_bytes(
///                             bytes.try_into().unwrap(),
///                         );
///                         *input = rest;
///                         tmp_vec.push(converted);
///                     }
///                     let out: [f64; 2] = tmp_vec
///                         .into_iter()
///                         .collect::<Vec<f64>>()
///                         .try_into()
///                         .unwrap();
///                     out
///                 })
///             }
///             DataTypes::IEEE754MSBSingleArr2 => {
///                 DataTypesCast::IEEE754MSBSingleArr2({
///                     let mut tmp_vec = std::vec::Vec::new();
///                     for _ in 0..2 {
///                         let (bytes, rest) = input
///                             .split_at(std::mem::size_of::<f32>());
///                         let converted = <f32>::from_le_bytes(
///                             bytes.try_into().unwrap(),
///                         );
///                         *input = rest;
///                         tmp_vec.push(converted);
///                     }
///                     let out: [f32; 2] = tmp_vec
///                         .into_iter()
///                         .collect::<Vec<f32>>()
///                         .try_into()
///                         .unwrap();
///                     out
///                 })
///             }
///             DataTypes::IEEE754MSBDoubleArr2 => {
///                 DataTypesCast::IEEE754MSBDoubleArr2({
///                     let mut tmp_vec = std::vec::Vec::new();
///                     for _ in 0..2 {
///                         let (bytes, rest) = input
///                             .split_at(std::mem::size_of::<f64>());
///                         let converted = <f64>::from_le_bytes(
///                             bytes.try_into().unwrap(),
///                         );
///                         *input = rest;
///                         tmp_vec.push(converted);
///                     }
///                     let out: [f64; 2] = tmp_vec
///                         .into_iter()
///                         .collect::<Vec<f64>>()
///                         .try_into()
///                         .unwrap();
///                     out
///                 })
///             }
///             DataTypes::IEEE754LSBSingleArr3 => {
///                 DataTypesCast::IEEE754LSBSingleArr3({
///                     let mut tmp_vec = std::vec::Vec::new();
///                     for _ in 0..3 {
///                         let (bytes, rest) = input
///                             .split_at(std::mem::size_of::<f32>());
///                         let converted = <f32>::from_le_bytes(
///                             bytes.try_into().unwrap(),
///                         );
///                         *input = rest;
///                         tmp_vec.push(converted);
///                     }
///                     let out: [f32; 3] = tmp_vec
///                         .into_iter()
///                         .collect::<Vec<f32>>()
///                         .try_into()
///                         .unwrap();
///                     out
///                 })
///             }
///             DataTypes::IEEE754LSBDoubleArr3 => {
///                 DataTypesCast::IEEE754LSBDoubleArr3({
///                     let mut tmp_vec = std::vec::Vec::new();
///                     for _ in 0..3 {
///                         let (bytes, rest) = input
///                             .split_at(std::mem::size_of::<f64>());
///                         let converted = <f64>::from_le_bytes(
///                             bytes.try_into().unwrap(),
///                         );
///                         *input = rest;
///                         tmp_vec.push(converted);
///                     }
///                     let out: [f64; 3] = tmp_vec
///                         .into_iter()
///                         .collect::<Vec<f64>>()
///                         .try_into()
///                         .unwrap();
///                     out
///                 })
///             }
///             DataTypes::ASCIIString => {
///                 DataTypesCast::ASCIIString(
///                     String::from_utf8(input.to_vec()).unwrap(),
///                 )
///             }
///         }
///     }
/// }
/// impl std::convert::TryInto<f32> for DataTypesCast {
///     type Error = String;
///     fn try_into(self) -> Result<f32, Self::Error> {
///         match self {
///             DataTypesCast::IEEE754LSBSingle(val) => Ok(val),
///             _ => {
///                 Err({
///                     let res = ::alloc::fmt::format(
///                         format_args!(
///                             "Cannot convert non-compatible DataTypesCast into {0}",
///                             "f32"
///                         ),
///                     );
///                     res
///                 })
///             }
///         }
///     }
/// }
/// impl std::convert::TryInto<f64> for DataTypesCast {
///     type Error = String;
///     fn try_into(self) -> Result<f64, Self::Error> {
///         match self {
///             DataTypesCast::IEEE754LSBDouble(val) => Ok(val),
///             _ => {
///                 Err({
///                     let res = ::alloc::fmt::format(
///                         format_args!(
///                             "Cannot convert non-compatible DataTypesCast into {0}",
///                             "f64"
///                         ),
///                     );
///                     res
///                 })
///             }
///         }
///     }
/// }
/// impl std::convert::TryInto<[f64; 2]> for DataTypesCast {
///     type Error = String;
///     fn try_into(self) -> Result<[f64; 2], Self::Error> {
///         match self {
///             DataTypesCast::IEEE754LSBDoubleArr2(val) => Ok(val),
///             DataTypesCast::IEEE754MSBDoubleArr2(val) => Ok(val),
///             _ => {
///                 Err({
///                     let res = ::alloc::fmt::format(
///                         format_args!(
///                             "Cannot convert non-compatible DataTypesCast into {0}",
///                             "[f64 ; 2]"
///                         ),
///                     );
///                     res
///                 })
///             }
///         }
///     }
/// }
/// impl std::convert::TryInto<[f32; 3]> for DataTypesCast {
///     type Error = String;
///     fn try_into(self) -> Result<[f32; 3], Self::Error> {
///         match self {
///             DataTypesCast::IEEE754LSBSingleArr3(val) => Ok(val),
///             _ => {
///                 Err({
///                     let res = ::alloc::fmt::format(
///                         format_args!(
///                             "Cannot convert non-compatible DataTypesCast into {0}",
///                             "[f32 ; 3]"
///                         ),
///                     );
///                     res
///                 })
///             }
///         }
///     }
/// }
/// impl std::convert::TryInto<[f64; 3]> for DataTypesCast {
///     type Error = String;
///     fn try_into(self) -> Result<[f64; 3], Self::Error> {
///         match self {
///             DataTypesCast::IEEE754LSBDoubleArr3(val) => Ok(val),
///             _ => {
///                 Err({
///                     let res = ::alloc::fmt::format(
///                         format_args!(
///                             "Cannot convert non-compatible DataTypesCast into {0}",
///                             "[f64 ; 3]"
///                         ),
///                     );
///                     res
///                 })
///             }
///         }
///     }
/// }
/// impl std::convert::TryInto<[f32; 2]> for DataTypesCast {
///     type Error = String;
///     fn try_into(self) -> Result<[f32; 2], Self::Error> {
///         match self {
///             DataTypesCast::IEEE754LSBSingleArr2(val) => Ok(val),
///             DataTypesCast::IEEE754MSBSingleArr2(val) => Ok(val),
///             _ => {
///                 Err({
///                     let res = ::alloc::fmt::format(
///                         format_args!(
///                             "Cannot convert non-compatible DataTypesCast into {0}",
///                             "[f32 ; 2]"
///                         ),
///                     );
///                     res
///                 })
///             }
///         }
///     }
/// }
/// impl std::convert::TryInto<String> for DataTypesCast {
///     type Error = String;
///     fn try_into(self) -> Result<String, Self::Error> {
///         match self {
///             DataTypesCast::ASCIIString(val) => Ok(val),
///             _ => {
///                 Err({
///                     let res = ::alloc::fmt::format(
///                         format_args!(
///                             "Cannot convert non-compatible DataTypesCast into {0}",
///                             "String"
///                         ),
///                     );
///                     res
///                 })
///             }
///         }
///     }
/// }
/// ```
// Define the `TypeCast` custom derive macro
#[proc_macro_derive(TypeCast, attributes(cast))]
pub fn derive_macro(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // Declare mutable variables to store cast types, conversions, and other related information
    let data_type_names = repeat(name.clone());
    let complex_data_type_names = repeat(name.clone());
    let data_kind_name = format_ident!("{}Cast", name.clone());
    let data_kind_names = repeat(data_kind_name.clone());
    let complex_data_kind_names = repeat(data_kind_name.clone());

    let string_data_kind_names = name.clone();

    let cast_type_data = &mut CastTypeData::default();

    let errors = if let syn::Data::Enum(data_enum) = ast.data {
        // Call the `get_cast_types` function to extract the required information from the Enum
        // This should return an empty Vec if successful or return a Vec<TokenTree> of errors otherwise
        get_cast_types(data_enum, cast_type_data)
    } else {
        return syn::Error::new_spanned(&ast, "TypeCast can only be derived for enums")
            .to_compile_error()
            .into();
    };

    if !errors.is_empty() {
        return proc_macro2::TokenStream::from_iter(errors.into_iter()).into();
    }

    let type_variants_map = build_type_variants_map(&cast_type_data);

    let generated_try_into_impls = generate_try_into_impls(&type_variants_map, &data_kind_name);

    let CastTypeData {
        cast_types,
        complex_cast_types,
        complex_cast_types_group,
        conversion,
        number_of_array_elements,
        variants,
        complex_variants,
        string_types,
        string_variants,
        ..
    } = &cast_type_data;

    let gen = quote! {

        // Derive common traits and define the enum with primitive, complex, and string type variants. The enum's name is generated by appending "Cast" to the name of the enum decorated with #[derive(TypeCast)]
        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub enum #data_kind_name {
            #(#variants(#cast_types),)*
            #(#complex_variants(#complex_cast_types_group),)*
            #(#string_variants(#string_types),)*
        }

        // Implement the parse method for the enum decorated with #[derive(TypeCast)]
        impl #name  {
            // The parse method takes a mutable reference to a byte slice and returns an instance of DataKind
            pub fn parse(self, input: &mut &[u8]) -> #data_kind_name {
                // Match the current variant of the enum decorated with #[derive(TypeCast)] and convert the input bytes accordingly
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
                    // Handle complex types
                    #(
                        #complex_data_type_names::#complex_variants => #complex_data_kind_names::#complex_variants ({
                            let mut tmp_vec = std::vec::Vec::new();
                            // Convert the byte information into the defined rust type from the cast. Push them to the temporary vector in this loop. This allows for an output array to be created that has the size of the array with the expected types defined in the cast.
                            for _ in 0..#number_of_array_elements {
                                let (bytes, rest) = input.split_at(
                                    std::mem::size_of::<#complex_cast_types>()
                                );
                                let converted = <#complex_cast_types>::from_le_bytes(bytes.try_into().unwrap());
                                // This allows the input to become the remaining bytes for the next iteration
                                *input = rest;
                                tmp_vec.push(converted);
                            }
                            // Transform the vec into the output array
                            let out: [#complex_cast_types;#number_of_array_elements]  = tmp_vec.into_iter().collect::<Vec<#complex_cast_types>>().try_into().unwrap();
                            out
                        }),
                    )*
                    // Handle string types
                    #(
                        #string_data_kind_names::#string_variants => #data_kind_name::#string_variants(String::from_utf8(input.to_vec()).unwrap())
                    )*

                }

            }

        }
    };

    let generated_from_str_impl = generated_from_str_impl(name, cast_type_data);

    let combined_gen = quote! {
        #gen
        #generated_try_into_impls
        #generated_from_str_impl
    };
    combined_gen.into()
}
