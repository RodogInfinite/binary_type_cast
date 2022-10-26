use type_cast_macro::DataCast;
use type_cast_macro_derive::cast;
use type_cast_macro_derive::DataCast;

#[derive(Clone, Copy, Debug, DataCast)]
pub enum DataTypes {
    // 4 bytes
    #[cast(f32 => from_le_bytes)]
    IEEE754LSBSingle,
    // 8 bytes
    #[cast(f64 => from_le_bytes)]
    IEEE754LSBDouble,
    // 4 bytes
    #[cast([f32;2] => from_le_bytes)]
    IEEE754LSBSingleArr,
    // 8 bytes
    #[cast([f64;2] => from_le_bytes)]
    IEEE754MSBDoubleArr,
}

fn main() {
    DataTypes::parse()
}
