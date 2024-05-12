use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;

use super::CastTypeData;

/// # Example
/// ```rust
///  #[derive(Clone, Copy, Debug, Serialize, Deserialize, TypeCast)]
///  pub enum DataTypes {
///      // 4 bytes
///      #[cast(from_le_bytes => f32)]
///      IEEE754LSBSingle,
///      // 8 bytes
///      #[cast(from_le_bytes => f64)]
///      IEEE754LSBDouble,
///      // [4 bytes, 4 bytes]
///      #[cast(from_le_bytes => [f32;2])]
///      IEEE754LSBSingleArr2,
///      // [8 bytes, 8 bytes]
///      #[cast(from_le_bytes => [f64;2])]
///      IEEE754LSBDoubleArr2,
///      // [4 bytes, 4 bytes]
///      #[cast(from_be_bytes => [f32;2])]
///      IEEE754MSBSingleArr2,
///      // [8 bytes, 8 bytes]
///      #[cast(from_be_bytes => [f64;2])]
///      IEEE754MSBDoubleArr2,
///      // [4 bytes, 4 bytes, 4 bytes]
///      #[cast(from_le_bytes => [f32;3])]
///      IEEE754LSBSingleArr3,
///      // [8 bytes, 8 bytes, 8 bytes]
///      #[cast(from_le_bytes => [f64;3])]
///      IEEE754LSBDoubleArr3,
///      #[cast(String)]
///      ASCIIString,
///  }
///  ```
///
/// ```rust
/// // Generated at compile time for the FromStr trait:
/// impl std::str::FromStr for DataTypes {
///     type Err = Box<dyn std::error::Error + Send + Sync>;
///
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         match s {
///             "IEEE754LSBSingle" => Ok(DataTypes::IEEE754LSBSingle),
///             "IEEE754LSBDouble" => Ok(DataTypes::IEEE754LSBDouble),
///             "IEEE754LSBSingleArr2" => Ok(DataTypes::IEEE754LSBSingleArr2),
///             "IEEE754LSBDoubleArr2" => Ok(DataTypes::IEEE754LSBDoubleArr2),
///             "IEEE754MSBSingleArr2" => Ok(DataTypes::IEEE754MSBSingleArr2),
///             "IEEE754MSBDoubleArr2" => Ok(DataTypes::IEEE754MSBDoubleArr2),
///             "IEEE754LSBSingleArr3" => Ok(DataTypes::IEEE754LSBSingleArr3),
///             "IEEE754LSBDoubleArr3" => Ok(DataTypes::IEEE754LSBDoubleArr3),
///             "ASCIIString" => Ok(DataTypes::ASCIIString),
///             _ => Err("Invalid variant".into()),
///         }
///     }
/// }
/// ```
///
/// ```rust
/// // Usage
///
/// pub fn usage_example() {
///     let data_type_str = "IEEE754LSBSingle";
///     let data_type = DataTypes::from_str(data_type_str).unwrap();
///     assert_eq!(data_type, DataTypes::IEEE754LSBSingle);
///
///     let data_type_str = "IEEE754LSBDoubleArr3";
///     let data_type = DataTypes::from_str(data_type_str).unwrap();
///     assert_eq!(data_type, DataTypes::IEEE754LSBDoubleArr3);
///
///     let data_type_str = "ASCIIString";
///     let data_type = DataTypes::from_str(data_type_str).unwrap();
///     assert_eq!(data_type, DataTypes::ASCIIString);
///
///     // For an invalid variant
///     let data_type_str = "InvalidVariant";
///     let data_type = DataTypes::from_str(data_type_str);
///     assert!(data_type.is_err());
/// }
///
/// ```

pub fn generated_from_str_impl(name: &Ident, cast_type_data: &mut CastTypeData) -> TokenStream2 {
    let mut match_arms = quote! {};
    for variant in &cast_type_data.variants {
        match_arms = quote! {
            #match_arms
            stringify!(#variant) => Ok(#name::#variant),
        };
    }
    for complex_variant in &cast_type_data.complex_variants {
        match_arms = quote! {
            #match_arms
            stringify!(#complex_variant) => Ok(#name::#complex_variant),
        };
    }
    for string_variant in &cast_type_data.string_variants {
        match_arms = quote! {
            #match_arms
            stringify!(#string_variant) => Ok(#name::#string_variant),
        };
    }

    quote! {
        impl std::str::FromStr for #name {
            type Err = Box<dyn std::error::Error>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #match_arms
                    _ => Err("Invalid variant".into())
                }
            }
        }
    }
}
