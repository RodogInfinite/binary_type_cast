pub mod cast_extraction;
pub mod from_str;
pub mod try_into;

// Define a struct to hold the vectors
#[derive(Clone, Default)]
pub struct CastTypeData {
    pub cast_types: Vec<proc_macro2::Ident>,
    pub complex_cast_types: Vec<proc_macro2::Ident>,
    pub complex_cast_types_group: Vec<proc_macro2::Group>,
    pub conversion: Vec<proc_macro2::Ident>,
    pub complex_conversion: Vec<proc_macro2::Ident>,
    pub number_of_array_elements: Vec<proc_macro2::Literal>,
    pub variants: Vec<proc_macro2::Ident>,
    pub complex_variants: Vec<proc_macro2::Ident>,
    pub string_types: Vec<proc_macro2::Ident>,
    pub string_variants: Vec<proc_macro2::Ident>,
}
