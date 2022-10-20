# binary_type_cast
Crate for testing the type_cast_derive proc_macro

## Under heavy development and not currently working!


## Description is incomplete!


```Rust
#[derive(Clone, Copy, Debug)]
pub enum DataTypes {
    // 4 bytes
    #[cast(f32 => from_le_bytes)] // f32=> from_be_bytes should also work 
    IEEE754LSBSingle,
    // 8 bytes
    #[cast(f64=> from_le_bytes)] // f64 => from_be_bytes should also work 
    IEEE754LSBDouble,
    // 4 bytes
    #[cast([f32;2] => from_le_bytes)] //[f32;2]=> from_be_bytes should also work 
    IEEE754LSBSingleArr,
    // 8 bytes
    #[cast([f64;2] => from_le_bytes)] // [f64;2] => from_be_bytes should also work 
    IEEE754MSBDoubleArr,

}

//expand to:
#[derive(Clone, Copy, Debug)]
enum DataKind {
    IEEE754LSBSingle(f32),
    IEEE754LSBDouble(f64),
    IEEE754LSBSingleArr([f32;2]),
    IEEE754MSBDoubleArr([f64;2]),
}

impl DataKind {
    fn parse(&self, input &mut &[u8]) -> DataKind {
        match self {
            DataTypes::IEEE754LSBSingle => DataKind::IEEE754LSBSingle({
                let (bytes, _) = input.split_at(std::mem::size_of::<f32>());
                f32::from_le_bytes(bytes.try_into().unwrap())
            }),
            DataTypes::IEEE754LSBDouble => DataKind::IEEE754LSBDouble({
                let (bytes, _) = input.split_at(std::mem::size_of::<f64>());
                f64::from_le_bytes(bytes.try_into().unwrap())
            }),
            DataTypes::IEEE754LSBSingleArr => DataKind::IEEE754LSBSingleArr({
                let (bytes, rest) = input.split_at(std::mem::size_of::<f32>());
                [
                    f32::from_le_bytes(bytes.try_into().unwrap())
                    f32::from_le_bytes(rest.try_into().unwrap())
                ]
                
            }),
            DataTypes::IEEE754LSBDoubleArr => DataKind::IEEE754LSDoubleArr({
                let (bytes, rest) = input.split_at(std::mem::size_of::<f64>());
                [
                    f64::from_le_bytes(bytes.try_into().unwrap())
                    f64::from_le_bytes(rest.try_into().unwrap())
                ]
            }),
        }
    }

    // generate one of these for each of the types in parse? or can it be generalized?
    pub fn unwrap_le_f32(self) -> f32 {
        match self {
            DataTypes::IEEE754LSBSingle(val) => val,
            _ => panic!(), // Better error handling here
        }
    }
}
```