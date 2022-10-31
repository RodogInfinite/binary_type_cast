# binary_type_cast
Crate for testing the type_cast_derive proc_macro

## Under heavy development!
TODO: 
- Create Tests
- Better Error Handling

```Rust
#[derive(TypeCast)]
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
}
```

## Expands to:

```rust
#[derive(Clone, Copy, Debug)]
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
    IEEE754LSBSingleArr3Elem,
    // [8 bytes, 8 bytes]
    #[cast(from_le_bytes => [f64;3])]
    IEEE754LSBDoubleArr3Elem,
    #[cast(String)]
    ASCIIString,
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
                    let mut tmp_vec = std::vec::Vec::new();
                    for _ in 0..2 {
                        let (bytes, rest) = input.split_at(std::mem::size_of::<f32>());
                        let converted = <f32>::from_le_bytes(bytes.try_into().unwrap());
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
            DataTypes::IEEE754LSBDoubleArr => {
                DataTypesCast::IEEE754LSBDoubleArr({
                    let mut tmp_vec = std::vec::Vec::new();
                    for _ in 0..2 {
                        let (bytes, rest) = input.split_at(std::mem::size_of::<f64>());
                        let converted = <f64>::from_le_bytes(bytes.try_into().unwrap());
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
            DataTypes::IEEE754MSBSingleArr => {
                DataTypesCast::IEEE754MSBSingleArr({
                    let mut tmp_vec = std::vec::Vec::new();
                    for _ in 0..2 {
                        let (bytes, rest) = input.split_at(std::mem::size_of::<f32>());
                        let converted = <f32>::from_le_bytes(bytes.try_into().unwrap());
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
            DataTypes::IEEE754MSBDoubleArr => {
                DataTypesCast::IEEE754MSBDoubleArr({
                    let mut tmp_vec = std::vec::Vec::new();
                    for _ in 0..2 {
                        let (bytes, rest) = input.split_at(std::mem::size_of::<f64>());
                        let converted = <f64>::from_le_bytes(bytes.try_into().unwrap());
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
            DataTypes::IEEE754LSBSingleArr3Elem => {
                DataTypesCast::IEEE754LSBSingleArr3Elem({
                    let mut tmp_vec = std::vec::Vec::new();
                    for _ in 0..3 {
                        let (bytes, rest) = input.split_at(std::mem::size_of::<f32>());
                        let converted = <f32>::from_le_bytes(bytes.try_into().unwrap());
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
            DataTypes::IEEE754LSBDoubleArr3Elem => {
                DataTypesCast::IEEE754LSBDoubleArr3Elem({
                    let mut tmp_vec = std::vec::Vec::new();
                    for _ in 0..3 {
                        let (bytes, rest) = input.split_at(std::mem::size_of::<f64>());
                        let converted = <f64>::from_le_bytes(bytes.try_into().unwrap());
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
                DataTypesCast::ASCIIString(String::from_utf8(input.to_vec()).unwrap())
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
        "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
        DataTypes::IEEE754LSBSingle.parse(&mut data),
        DataTypes::IEEE754LSBDouble.parse(&mut data2),
        DataTypes::IEEE754LSBSingleArr.parse(&mut data2),
        DataTypes::IEEE754LSBDoubleArr.parse(&mut data3),
        DataTypes::IEEE754MSBSingleArr.parse(&mut data4),
        DataTypes::IEEE754MSBDoubleArr.parse(&mut data5),
        DataTypes::IEEE754LSBSingleArr3Elem.parse(&mut data6),
        DataTypes::IEEE754LSBDoubleArr3Elem.parse(&mut data7),
        DataTypes::ASCIIString.parse(&mut "Hello".as_bytes()),
        DataTypes::ASCIIString.parse(&mut "GoodBye".as_bytes()),
    )
}
```

## Output
```rust
IEEE754LSBSingle(-239.59637)
IEEE754LSBDouble(-683.6825016706912)
IEEE754LSBSingleArr([-239.59637, -4.1676583])
IEEE754LSBDoubleArr([-683.6825016706912, -683.6825016706912])
IEEE754MSBSingleArr([-239.59637, -4.1676583])
IEEE754MSBDoubleArr([-683.6825016706912, -683.6825016706912])
IEEE754LSBSingleArr3Elem([-239.59637, -4.1676583, -239.59637])
IEEE754LSBDoubleArr3Elem([-683.6825016706912, -683.6825016706912, -7.11487364695832e16])
ASCIIString("Hello")
ASCIIString("GoodBye")
```