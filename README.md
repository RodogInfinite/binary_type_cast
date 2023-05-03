# binary_type_cast
A Rust crate for simplifying the process of parsing binary file data into various Rust data types using the TypeCast macro.

## Under heavy development!
TODO: 
- Create Tests
- Better Error Handling

## Overview

 The macro is designed to simplify parsing data records in a binary file via usage of a text from a description file, in the example, a `desc.xml` file. The following demonstrates how to define a custom enum called `DataTypes` and use the `TypeCast` macro.

## Usage
### Define a custom enum
In this example define `DataTypes`, and use the cast attribute for each variant. 

```rust
#[derive(Clone, Copy, Debug, Serialize, Deserialize, TypeCast)]
pub enum DataTypes {
    #[cast(from_le_bytes => f32)]
    AnyCustomVariant,
    #[cast(from_le_bytes => [f32;2])]
    AnyCustomVariant2

}
```

This will automatically generate a DataTypesCast enum:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataTypesCast {
    AnyCustomVariant(f32),
    AnyCustomVariant2([f32;2]),
}
```

---

#### Note:
Any Enum that utilizes the TypeCast will simply have `Cast` appended to the name. This:
```rust
#[derive(TypeCast)]
enum ExampleEnum {
    //
}
```
will generate the following at compile time:
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
enum ExampleEnumCast {
    //
}
```
The attribute macro `#[derive(Clone, Debug, Serialize, Deserialize)]` is hardcoded above the `enum {}Cast` at the moment. If there is significant demand, the macro can be altered to include only those defined on the parent enum.

---

### Code Generation
Okay, but why not just define the DataTypesCast or ExampleEnumCast and skip the attribute nonsense?

Because the following are automatically generated:
```rust
impl DataTypes {
    pub fn parse(self, input: &mut &[u8]) -> DataTypesCast {
        match self {
            DataTypes::AnyCustomVariant => {
                DataTypesCast::AnyCustomVariant({
                    let (bytes, _) = input.split_at(std::mem::size_of::<f32>());
                    <f32>::from_le_bytes(bytes.try_into().unwrap())
                })
            }
            // This can be generated for any sized array supported by the standard library
            DataTypes::AnyCustomVariant2 => {
                DataTypesCast::AnyCustomVariant2({
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
        }
    }

    // The TryInto implementations are so the values can be used outside of match statements. Right now, its tedious to use, but it works. See the hashmapped_fields example for usage.
    impl std::convert::TryInto<f32> for DataTypesCast {
        type Error = String;
        fn try_into(self) -> Result<f32, Self::Error> {
            match self {
                DataTypesCast::AnyCustomVariant(val) => Ok(val),
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
    // allows 
    impl std::convert::TryInto<[f32; 2]> for DataTypesCast {
        type Error = String;
        fn try_into(self) -> Result<[f32; 2], Self::Error> {
            match self {
                DataTypesCast::AnyCustomVariant2(val) => Ok(val),
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
}

// Generated at compile time for FromStr trait:
impl std::str::FromStr for DataTypes {
    type Err = Box<dyn std::error::Error + Send + Sync>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AnyCustomVariant" => Ok(DataTypes::AnyCustomVariant),
            "AnyCustomVariant2" => Ok(DataTypes::AnyCustomVariant2),
            _ => Err("Invalid variant".into()),
        }
    }
}
```
If the parent enum has a large number of variants, this would be extremely tedious to type out. 

## Next Steps
Define your data structures and functions for parsing the binary data. 

In the hashmapped_fields example, `DataRecord` is a struct that contains a HashMap of field names and their parsed values. `RecordDescs` is a struct that deserializes description information from the `desc.xml` whose descriptions are then used to parse the `data.dat` file into a `DataRecord`. The example prints out the parsed data in the `DataTypesCast` variants and then extracts and prints the values output from a match statement and output individually using `try_into`.