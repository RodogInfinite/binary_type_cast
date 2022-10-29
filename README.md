# binary_type_cast
Crate for testing the type_cast_derive proc_macro

## Under heavy development!
TODO: make the filter for primitives and arrays more robust and flexible than looking for "Arr" at the end. Should work no matter what the given enum variant names are.

```Rust
#[derive(DataCast)]
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
    IEEE754LSBDoubleArr,
    // [4 bytes, 4 bytes]
    #[cast([f32;2] => from_be_bytes)]
    IEEE754MSBSingleArr,
    // [8 bytes, 8 bytes]
    #[cast([f64;2] => from_be_bytes)]
    IEEE754MSBDoubleArr,
}
```

## Expands to:

```rust
#[derive(Clone, Copy, Debug)]
enum DataTypesCast {
    IEEE754LSBSingle(f32),
    IEEE754LSBDouble(f64),
    IEEE754LSBSingleArr([f32; 2]),
    IEEE754LSBDoubleArr([f64; 2]),
    IEEE754MSBSingleArr([f32; 2]),
    IEEE754MSBDoubleArr([f64; 2]),
}

impl DataTypes {
    fn parse(self, input: &mut &[u8]) -> DataTypesCast {
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
            DataTypes::IEEE754LSBSingleArr => {
                DataTypesCast::IEEE754LSBSingleArr({
                    let (bytes, rest) = input.split_at(std::mem::size_of::<f32>());
                    [
                        <f32>::from_le_bytes(bytes.try_into().unwrap()),
                        <f32>::from_le_bytes(rest.try_into().unwrap()),
                    ]
                })
            }
            DataTypes::IEEE754LSBDoubleArr => {
                DataTypesCast::IEEE754LSBDoubleArr({
                    let (bytes, rest) = input.split_at(std::mem::size_of::<f64>());
                    [
                        <f64>::from_le_bytes(bytes.try_into().unwrap()),
                        <f64>::from_le_bytes(rest.try_into().unwrap()),
                    ]
                })
            }
            DataTypes::IEEE754MSBSingleArr => {
                DataTypesCast::IEEE754MSBSingleArr({
                    let (bytes, rest) = input.split_at(std::mem::size_of::<f32>());
                    [
                        <f32>::from_be_bytes(bytes.try_into().unwrap()),
                        <f32>::from_be_bytes(rest.try_into().unwrap()),
                    ]
                })
            }
            DataTypes::IEEE754MSBDoubleArr => {
                DataTypesCast::IEEE754MSBDoubleArr({
                    let (bytes, rest) = input.split_at(std::mem::size_of::<f64>());
                    [
                        <f64>::from_be_bytes(bytes.try_into().unwrap()),
                        <f64>::from_be_bytes(rest.try_into().unwrap()),
                    ]
                })
            }
        }
    }
}

```


## Main
```rust
fn main() {
    let mut data: &[u8] = &[172, 152, 111, 195];
    let mut data2: &[u8] = &[172, 152, 111, 195, 117, 93, 133, 192];
    let mut data3: &[u8] = &[
        172, 152, 111, 195, 117, 93, 133, 192, 172, 152, 111, 195, 117, 93, 133, 192,
    ];
    println!(
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
        DataTypes::IEEE754LSBSingle.parse(&mut data),
        DataTypes::IEEE754LSBDouble.parse(&mut data2),
        DataTypes::IEEE754LSBSingleArr.parse(&mut data2),
        DataTypes::IEEE754LSBDoubleArr.parse(&mut data3),
        DataTypes::IEEE754MSBSingleArr.parse(&mut data2),
        DataTypes::IEEE754MSBDoubleArr.parse(&mut data3),
    )
}
```

## Output
```rust
IEEE754LSBSingle(-239.59637)
IEEE754LSBDouble(-683.6825016706912)
IEEE754LSBSingleArr([-239.59637, -4.1676583])
IEEE754LSBDoubleArr([-683.6825016706912, -683.6825016706912])
IEEE754MSBSingleArr([-4.332508e-12, 2.8081308e32])
IEEE754MSBDoubleArr([-7.321865025863303e-94, -7.321865025863303e-94])
```