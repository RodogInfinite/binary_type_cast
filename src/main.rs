use type_cast_macro::DataCast;
use type_cast_macro_derive::DataCast;

#[derive(Clone, Copy, Debug, DataCast)]
pub enum DataTypes {
    // 4 bytes
    #[cast(f32 => from_le_bytes)]
    IEEE754LSBSingle,
    // 8 bytes
    #[cast(f64 => from_le_bytes)]
    IEEE754LSBDouble,
    // [4 bytes, 4 bytes]
    #[cast([f32;2] => from_le_bytes)]
    IEEE754LSBSingleArr,
    // [8 bytes, 8 bytes]
    #[cast([f64;2] => from_le_bytes)]
    IEEE754MSBDoubleArr,
}


fn main() {
    let mut data:   &[u8] = &[172, 152, 111, 195];
    let mut data2:  &[u8] = &[172, 152, 111, 195, 117, 93, 133, 192];
    let mut data3:  &[u8] = &[172, 152, 111, 195, 117, 93, 133, 192, 172, 152, 111, 195, 117, 93, 133, 192];
    println!("{:?}\n{:?}\n{:?}\n{:?}\n",
        DataTypes::IEEE754LSBSingle.parse(&mut data),
        DataTypes::IEEE754LSBDouble.parse(&mut data2),
        DataTypes::IEEE754LSBSingleArr.parse(&mut data2),
        DataTypes::IEEE754MSBDoubleArr.parse(&mut data3),
)
    
}
