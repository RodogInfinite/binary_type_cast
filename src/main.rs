use type_cast_macro_derive::TypeCast;

#[derive(Clone, Copy, Debug, TypeCast)]
pub enum DataTypes {
    // 4 bytes
    #[cast(from_le_bytes => f32)]
    IEEE754LSBSingle,
    // 8 bytes
    #[cast(from_le_bytes => f64)]
    IEEE754LSBDouble,
    // [4 bytes, 4 bytes]
    #[cast(from_le_bytes => [f32;2])]
    IEEE754LSBSingleArr,
    // [8 bytes, 8 bytes]
    #[cast(from_le_bytes => [f64;2])]
    IEEE754LSBDoubleArr,
    // [4 bytes, 4 bytes]
    #[cast(from_be_bytes => [f32;2])]
    IEEE754MSBSingleArr,
    // [8 bytes, 8 bytes]
    #[cast(from_be_bytes => [f64;2])]
    IEEE754MSBDoubleArr,
    // [4 bytes, 4 bytes]
    #[cast(from_le_bytes => [f32;3])]
    IEEE754LSBSingle2Arr,
    // [8 bytes, 8 bytes]
    #[cast(from_le_bytes => [f64;3])]
    IEEE754LSBDouble2Arr,
}

fn main() {
    let mut data: &[u8] = &[172, 152, 111, 195];
    let mut data2: &[u8] = &[172, 152, 111, 195, 117, 93, 133, 192];
    let mut data3: &[u8] = &[
        172, 152, 111, 195, 117, 93, 133, 192, 172, 152, 111, 195, 117, 93, 133, 192,
    ];
    let mut data4: &[u8] = &[172, 152, 111, 195, 117, 93, 133, 192];
    let mut data5: &[u8] = &[
        172, 152, 111, 195, 117, 93, 133, 192, 172, 152, 111, 195, 117, 93, 133, 192,
    ];
    let mut data6: &[u8] = &[172, 152, 111, 195, 117, 93, 133, 192, 172, 152, 111, 195];
    let mut data7: &[u8] = &[
        172, 152, 111, 195, 117, 93, 133, 192, 172, 152, 111, 195, 117, 93, 133, 192, 172, 152,
        111, 195, 172, 152, 111, 195,
    ];
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
        DataTypes::IEEE754LSBSingle.parse(&mut data),
        DataTypes::IEEE754LSBDouble.parse(&mut data2),
        DataTypes::IEEE754LSBSingleArr.parse(&mut data2),
        DataTypes::IEEE754LSBDoubleArr.parse(&mut data3),
        DataTypes::IEEE754MSBSingleArr.parse(&mut data4),
        DataTypes::IEEE754MSBDoubleArr.parse(&mut data5),
        DataTypes::IEEE754LSBSingle2Arr.parse(&mut data6),
        DataTypes::IEEE754LSBDouble2Arr.parse(&mut data7),
        //DataTypes::IEEE754MSBSingleArr.parse(&mut data4),
        //DataTypes::IEEE754MSBDoubleArr.parse(&mut data5),
    )
}
