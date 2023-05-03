 # Example
 ```rust
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
 ```

 Generated at compile time:

 ```rust
 #[derive(Clone, Copy, Debug, Serialize, Deserialize, TypeCast)]
 pub enum DataTypes {
     IEEE754LSBSingle(f32),
     IEEE754LSBDouble(f64),
     IEEE754LSBSingleArr2([f32;2]),
     IEEE754LSBDoubleArr2([f64;2]),
     IEEE754MSBSingleArr2([f32;2]),
     IEEE754MSBDoubleArr2([f64;2]),
     IEEE754LSBSingleArr3([f32;3]),
     IEEE754LSBDoubleArr3([f64;3]),
     ASCIIString(String),
 }


 impl DataTypes {
     pub fn parse(self, input: &mut &[u8]) -> DataTypesCast {
         match self {
             DataTypes::IEEE754LSBSingle => {
                 DataTypesCast::IEEE754LSBSingle({
                     let (bytes, _) = input.split_at(std::mem::size_of::<f32>());
                     <f32>::from_le_bytes(bytes.try_into().unwrap())
                 })
             }
             DataTypes::IEEE754LSBDouble => {
                 DataTypesCast::IEEE754LSBDouble({
                     let (bytes, _) = input.split_at(std::mem::size_of::<f64>());
                     <f64>::from_le_bytes(bytes.try_into().unwrap())
                 })
             }
             DataTypes::IEEE754LSBSingleArr2 => {
                 DataTypesCast::IEEE754LSBSingleArr2({
                     let mut tmp_vec = std::vec::Vec::new();
                     for _ in 0..2 {
                         let (bytes, rest) = input
                             .split_at(std::mem::size_of::<f32>());
                         let converted = <f32>::from_le_bytes(
                             bytes.try_into().unwrap(),
                         );
                         *input = rest;
                         tmp_vec.push(converted);
                     }
                     let out: [f32; 2] = tmp_vec
                         .into_iter()
                         .collect::<Vec<f32>>()
                         .try_into()
                         .unwrap();
                     out
                 })
             }
             DataTypes::IEEE754LSBDoubleArr2 => {
                 DataTypesCast::IEEE754LSBDoubleArr2({
                     let mut tmp_vec = std::vec::Vec::new();
                     for _ in 0..2 {
                         let (bytes, rest) = input
                             .split_at(std::mem::size_of::<f64>());
                         let converted = <f64>::from_le_bytes(
                             bytes.try_into().unwrap(),
                         );
                         *input = rest;
                         tmp_vec.push(converted);
                     }
                     let out: [f64; 2] = tmp_vec
                         .into_iter()
                         .collect::<Vec<f64>>()
                         .try_into()
                         .unwrap();
                     out
                 })
             }
             DataTypes::IEEE754MSBSingleArr2 => {
                 DataTypesCast::IEEE754MSBSingleArr2({
                     let mut tmp_vec = std::vec::Vec::new();
                     for _ in 0..2 {
                         let (bytes, rest) = input
                             .split_at(std::mem::size_of::<f32>());
                         let converted = <f32>::from_le_bytes(
                             bytes.try_into().unwrap(),
                         );
                         *input = rest;
                         tmp_vec.push(converted);
                     }
                     let out: [f32; 2] = tmp_vec
                         .into_iter()
                         .collect::<Vec<f32>>()
                         .try_into()
                         .unwrap();
                     out
                 })
             }
             DataTypes::IEEE754MSBDoubleArr2 => {
                 DataTypesCast::IEEE754MSBDoubleArr2({
                     let mut tmp_vec = std::vec::Vec::new();
                     for _ in 0..2 {
                         let (bytes, rest) = input
                             .split_at(std::mem::size_of::<f64>());
                         let converted = <f64>::from_le_bytes(
                             bytes.try_into().unwrap(),
                         );
                         *input = rest;
                         tmp_vec.push(converted);
                     }
                     let out: [f64; 2] = tmp_vec
                         .into_iter()
                         .collect::<Vec<f64>>()
                         .try_into()
                         .unwrap();
                     out
                 })
             }
             DataTypes::IEEE754LSBSingleArr3 => {
                 DataTypesCast::IEEE754LSBSingleArr3({
                     let mut tmp_vec = std::vec::Vec::new();
                     for _ in 0..3 {
                         let (bytes, rest) = input
                             .split_at(std::mem::size_of::<f32>());
                         let converted = <f32>::from_le_bytes(
                             bytes.try_into().unwrap(),
                         );
                         *input = rest;
                         tmp_vec.push(converted);
                     }
                     let out: [f32; 3] = tmp_vec
                         .into_iter()
                         .collect::<Vec<f32>>()
                         .try_into()
                         .unwrap();
                     out
                 })
             }
             DataTypes::IEEE754LSBDoubleArr3 => {
                 DataTypesCast::IEEE754LSBDoubleArr3({
                     let mut tmp_vec = std::vec::Vec::new();
                     for _ in 0..3 {
                         let (bytes, rest) = input
                             .split_at(std::mem::size_of::<f64>());
                         let converted = <f64>::from_le_bytes(
                             bytes.try_into().unwrap(),
                         );
                         *input = rest;
                         tmp_vec.push(converted);
                     }
                     let out: [f64; 3] = tmp_vec
                         .into_iter()
                         .collect::<Vec<f64>>()
                         .try_into()
                         .unwrap();
                     out
                 })
             }
             DataTypes::ASCIIString => {
                 DataTypesCast::ASCIIString(
                     String::from_utf8(input.to_vec()).unwrap(),
                 )
             }
         }
     }
 }
 impl std::convert::TryInto<f32> for DataTypesCast {
     type Error = String;
     fn try_into(self) -> Result<f32, Self::Error> {
         match self {
             DataTypesCast::IEEE754LSBSingle(val) => Ok(val),
             _ => {
                 Err({
                     let res = ::alloc::fmt::format(
                         format_args!(
                             "Cannot convert non-compatible DataTypesCast into {0}",
                             "f32"
                         ),
                     );
                     res
                 })
             }
         }
     }
 }
 impl std::convert::TryInto<f64> for DataTypesCast {
     type Error = String;
     fn try_into(self) -> Result<f64, Self::Error> {
         match self {
             DataTypesCast::IEEE754LSBDouble(val) => Ok(val),
             _ => {
                 Err({
                     let res = ::alloc::fmt::format(
                         format_args!(
                             "Cannot convert non-compatible DataTypesCast into {0}",
                             "f64"
                         ),
                     );
                     res
                 })
             }
         }
     }
 }
 impl std::convert::TryInto<[f64; 2]> for DataTypesCast {
     type Error = String;
     fn try_into(self) -> Result<[f64; 2], Self::Error> {
         match self {
             DataTypesCast::IEEE754LSBDoubleArr2(val) => Ok(val),
             DataTypesCast::IEEE754MSBDoubleArr2(val) => Ok(val),
             _ => {
                 Err({
                     let res = ::alloc::fmt::format(
                         format_args!(
                             "Cannot convert non-compatible DataTypesCast into {0}",
                             "[f64 ; 2]"
                         ),
                     );
                     res
                 })
             }
         }
     }
 }
 impl std::convert::TryInto<[f32; 3]> for DataTypesCast {
     type Error = String;
     fn try_into(self) -> Result<[f32; 3], Self::Error> {
         match self {
             DataTypesCast::IEEE754LSBSingleArr3(val) => Ok(val),
             _ => {
                 Err({
                     let res = ::alloc::fmt::format(
                         format_args!(
                             "Cannot convert non-compatible DataTypesCast into {0}",
                             "[f32 ; 3]"
                         ),
                     );
                     res
                 })
             }
         }
     }
 }
 impl std::convert::TryInto<[f64; 3]> for DataTypesCast {
     type Error = String;
     fn try_into(self) -> Result<[f64; 3], Self::Error> {
         match self {
             DataTypesCast::IEEE754LSBDoubleArr3(val) => Ok(val),
             _ => {
                 Err({
                     let res = ::alloc::fmt::format(
                         format_args!(
                             "Cannot convert non-compatible DataTypesCast into {0}",
                             "[f64 ; 3]"
                         ),
                     );
                     res
                 })
             }
         }
     }
 }
 impl std::convert::TryInto<[f32; 2]> for DataTypesCast {
     type Error = String;
     fn try_into(self) -> Result<[f32; 2], Self::Error> {
         match self {
             DataTypesCast::IEEE754LSBSingleArr2(val) => Ok(val),
             DataTypesCast::IEEE754MSBSingleArr2(val) => Ok(val),
             _ => {
                 Err({
                     let res = ::alloc::fmt::format(
                         format_args!(
                             "Cannot convert non-compatible DataTypesCast into {0}",
                             "[f32 ; 2]"
                         ),
                     );
                     res
                 })
             }
         }
     }
 }
 impl std::convert::TryInto<String> for DataTypesCast {
     type Error = String;
     fn try_into(self) -> Result<String, Self::Error> {
         match self {
             DataTypesCast::ASCIIString(val) => Ok(val),
             _ => {
                 Err({
                     let res = ::alloc::fmt::format(
                         format_args!(
                             "Cannot convert non-compatible DataTypesCast into {0}",
                             "String"
                         ),
                     );
                     res
                 })
             }
         }
     }
 }

// Generated at compile time for FromStr trait:
impl std::str::FromStr for DataTypes {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IEEE754LSBSingle" => Ok(DataTypes::IEEE754LSBSingle),
            "IEEE754LSBDouble" => Ok(DataTypes::IEEE754LSBDouble),
            "IEEE754LSBSingleArr2" => Ok(DataTypes::IEEE754LSBSingleArr2),
            "IEEE754LSBDoubleArr2" => Ok(DataTypes::IEEE754LSBDoubleArr2),
            "IEEE754MSBSingleArr2" => Ok(DataTypes::IEEE754MSBSingleArr2),
            "IEEE754MSBDoubleArr2" => Ok(DataTypes::IEEE754MSBDoubleArr2),
            "IEEE754LSBSingleArr3" => Ok(DataTypes::IEEE754LSBSingleArr3),
            "IEEE754LSBDoubleArr3" => Ok(DataTypes::IEEE754LSBDoubleArr3),
            "ASCIIString" => Ok(DataTypes::ASCIIString),
            _ => Err("Invalid variant".into()),
        }
    }
}
 
 ```