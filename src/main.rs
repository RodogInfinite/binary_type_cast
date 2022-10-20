use type_cast_macro::DataCast;
use type_cast_macro_derive::DataCast;
use type_cast_macro_derive::cast;




#[derive(DataCast)]
#[derive(Clone, Copy, Debug)]
pub enum DataTypes {
    // 4 bytes
    #[cast(f32)]
    IEEE754LSBSingle,
    // 8 bytes
    #[cast(f64)]
    IEEE754LSBDouble,
    // 4 bytes
    #[cast([f32;2])]
    IEEE754LSBSingleArr,
    // 8 bytes
    #[cast([f64;2])]
    IEEE754MSBDoubleArr,

}

fn main() {
    DataTypes::parse()
}