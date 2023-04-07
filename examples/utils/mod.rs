pub mod generate_mock_data;
use binary_type_cast::TypeCast;
use nom::{bytes::complete::take, IResult};
use serde::{Deserialize, Serialize};

// DataTypesCast is generated at compile time here
// The names of the variants should match the data_types textually described. In the example, its the `data_type` field in the xml which is then deserialized into the RecordDesc struct.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, TypeCast)]
pub enum DataTypes {
    // 4 bytes
    #[cast(from_le_bytes => f32)]
    IEEE754LSBSingle,
    // 8 bytes
    #[cast(from_le_bytes => f64)]
    IEEE754LSBDouble,
    // [4 bytes, 4 bytes]
    #[cast(from_le_bytes => [f32;2])]
    IEEE754LSBSingleArr2,
    // [8 bytes, 8 bytes]
    #[cast(from_le_bytes => [f64;2])]
    IEEE754LSBDoubleArr2,
    // [4 bytes, 4 bytes]
    #[cast(from_be_bytes => [f32;2])]
    IEEE754MSBSingleArr2,
    // [8 bytes, 8 bytes]
    #[cast(from_be_bytes => [f64;2])]
    IEEE754MSBDoubleArr2,
    // [4 bytes, 4 bytes, 4 bytes]
    #[cast(from_le_bytes => [f32;3])]
    IEEE754LSBSingleArr3,
    // [8 bytes, 8 bytes, 8 bytes]
    #[cast(from_le_bytes => [f64;3])]
    IEEE754LSBDoubleArr3,
    #[cast(String)]
    ASCIIString,
}

#[derive(Debug, Deserialize)]
pub struct RecordDescs {
    #[serde(rename = "RecordDesc")]
    pub record_desc: Vec<RecordDesc>,
}

#[derive(Debug, Deserialize)]
pub struct RecordDesc {
    pub name: String,
    pub number: u32,
    pub location: i32,
    pub data_type: DataTypes,
    pub length: u32,
}

pub fn parse_field_bytes<'i>(input: &'i [u8], length: &u32) -> IResult<&'i [u8], &'i [u8]> {
    take(*length)(input)
}
