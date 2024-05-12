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
 #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
 pub enum DataTypesCast {
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
        pub fn parse(self, input: &[u8]) -> IResult<&[u8], DataTypesCast> {
            match self {
                DataTypes::IEEE754LSBSingle => {
                    let (tail, bytes) = nom::number::complete::le_f32(input)?;
                    Ok((tail, DataTypesCast::IEEE754LSBSingle(bytes)))
                }
                DataTypes::IEEE754LSBDouble => {
                    let (tail, bytes) = nom::number::complete::le_f64(input)?;
                    Ok((tail, DataTypesCast::IEEE754LSBDouble(bytes)))
                }
                DataTypes::IEEE754LSBSingleArr2 => {
                    let (tail, elements_vec) = nom::multi::count(
                        nom::number::complete::le_f32,
                        2,
                    )(input)?;
                    let out: [f32; 2] = elements_vec
                        .try_into()
                        .map_err(|_| nom::Err::Failure(
                            nom::error::Error::new(input, nom::error::ErrorKind::Fail),
                        ))?;
                    Ok((tail, DataTypesCast::IEEE754LSBSingleArr2(out)))
                }
                DataTypes::IEEE754LSBDoubleArr2 => {
                    let (tail, elements_vec) = nom::multi::count(
                        nom::number::complete::le_f64,
                        2,
                    )(input)?;
                    let out: [f64; 2] = elements_vec
                        .try_into()
                        .map_err(|_| nom::Err::Failure(
                            nom::error::Error::new(input, nom::error::ErrorKind::Fail),
                        ))?;
                    Ok((tail, DataTypesCast::IEEE754LSBDoubleArr2(out)))
                }
                DataTypes::IEEE754MSBSingleArr2 => {
                    let (tail, elements_vec) = nom::multi::count(
                        nom::number::complete::be_f32,
                        2,
                    )(input)?;
                    let out: [f32; 2] = elements_vec
                        .try_into()
                        .map_err(|_| nom::Err::Failure(
                            nom::error::Error::new(input, nom::error::ErrorKind::Fail),
                        ))?;
                    Ok((tail, DataTypesCast::IEEE754MSBSingleArr2(out)))
                }
                DataTypes::IEEE754MSBDoubleArr2 => {
                    let (tail, elements_vec) = nom::multi::count(
                        nom::number::complete::be_f64,
                        2,
                    )(input)?;
                    let out: [f64; 2] = elements_vec
                        .try_into()
                        .map_err(|_| nom::Err::Failure(
                            nom::error::Error::new(input, nom::error::ErrorKind::Fail),
                        ))?;
                    Ok((tail, DataTypesCast::IEEE754MSBDoubleArr2(out)))
                }
                DataTypes::IEEE754LSBSingleArr3 => {
                    let (tail, elements_vec) = nom::multi::count(
                        nom::number::complete::le_f32,
                        3,
                    )(input)?;
                    let out: [f32; 3] = elements_vec
                        .try_into()
                        .map_err(|_| nom::Err::Failure(
                            nom::error::Error::new(input, nom::error::ErrorKind::Fail),
                        ))?;
                    Ok((tail, DataTypesCast::IEEE754LSBSingleArr3(out)))
                }
                DataTypes::IEEE754LSBDoubleArr3 => {
                    let (tail, elements_vec) = nom::multi::count(
                        nom::number::complete::le_f64,
                        3,
                    )(input)?;
                    let out: [f64; 3] = elements_vec
                        .try_into()
                        .map_err(|_| nom::Err::Failure(
                            nom::error::Error::new(input, nom::error::ErrorKind::Fail),
                        ))?;
                    Ok((tail, DataTypesCast::IEEE754LSBDoubleArr3(out)))
                }
                DataTypes::ASCIIString => {
                    let (tail, bytes) = nom::bytes::complete::take_while1(|c: u8| {
                        c.is_ascii()
                    })(input)?;
                    let string_result = String::from_utf8(bytes.to_vec())
                        .map_err(|_| nom::Err::Failure(
                            nom::error::Error::new(input, nom::error::ErrorKind::Fail),
                        ))?;
                    Ok((tail, DataTypesCast::ASCIIString(string_result)))
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
                                "[f32;2]"
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
                                "[f32;3]"
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
                                "[f64;2]"
                            ),
                        );
                        res
                    })
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
                                "[f64;3]"
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
    impl std::str::FromStr for DataTypes {
        type Err = Box<dyn std::error::Error>;
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