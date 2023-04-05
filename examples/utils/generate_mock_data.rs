// generate_mock_data.rs
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

fn read_expected_data() -> std::io::Result<Vec<u8>> {
    let mut expected_data = Vec::new();

    // Single
    let single: f32 = 123.456;
    expected_data.extend_from_slice(&single.to_le_bytes());

    // Double
    let double: f64 = 654.321;
    expected_data.extend_from_slice(&double.to_le_bytes());

    // SingleArr
    let single_arr: [f32; 2] = [1.234, 5.678];
    for &value in single_arr.iter() {
        expected_data.extend_from_slice(&value.to_le_bytes());
    }

    // DoubleArr
    let double_arr: [f64; 2] = [9.876, 5.432];
    for &value in double_arr.iter() {
        expected_data.extend_from_slice(&value.to_le_bytes());
    }

    // MSBSingleArr
    let msb_single_arr: [f32; 2] = [12.34, 56.78];
    for &value in msb_single_arr.iter() {
        expected_data.extend_from_slice(&value.to_be_bytes());
    }

    // MSBDoubleArr
    let msb_double_arr: [f64; 2] = [98.76, 54.32];
    for &value in msb_double_arr.iter() {
        expected_data.extend_from_slice(&value.to_be_bytes());
    }

    // LSBSingleArr3Elem
    let lsb_single_arr_3_elem: [f32; 3] = [12.3, 45.6, 78.9];
    for &value in lsb_single_arr_3_elem.iter() {
        expected_data.extend_from_slice(&value.to_le_bytes());
    }

    // LSBDoubleArr3Elem
    let lsb_double_arr_3_elem: [f64; 3] = [98.7, 65.4, 32.1];
    for &value in lsb_double_arr_3_elem.iter() {
        expected_data.extend_from_slice(&value.to_le_bytes());
    }

    // ASCIIString
    let ascii_string = "HelloWorld";
    expected_data.extend_from_slice(ascii_string.as_bytes());

    Ok(expected_data)
}

fn should_overwrite_existing_file(file_path: &Path) -> bool {
    if let Ok(expected_data) = read_expected_data() {
        if let Ok(mut file) = File::open(file_path) {
            let mut file_data = Vec::new();
            file.read_to_end(&mut file_data)
                .expect("Unable to read file contents");

            if file_data != expected_data {
                println!("File contents are different from expected data. Do you want to overwrite the file? (y/N)");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                input.trim().to_lowercase() == "y"
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
pub fn generate_mock_data_file(directory: &str, file: &str) {
    let data_file_path = format!("{}/{}", directory, file);
    let file_path = Path::new(&data_file_path);

    if !file_path.exists() || should_overwrite_existing_file(&file_path) {
        let mut file = File::create(&data_file_path).expect("Unable to create the file");
        // Single
        let single: f32 = 123.456;
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(&single.to_le_bytes()).unwrap();

        // Double
        let double: f64 = 654.321;
        file.seek(SeekFrom::Start(4)).unwrap();
        file.write_all(&double.to_le_bytes()).unwrap();

        // SingleArr
        let single_arr: [f32; 2] = [1.234, 5.678];
        file.seek(SeekFrom::Start(12)).unwrap();
        for &value in single_arr.iter() {
            file.write_all(&value.to_le_bytes()).unwrap();
        }

        // DoubleArr
        let double_arr: [f64; 2] = [9.876, 5.432];
        file.seek(SeekFrom::Start(20)).unwrap();
        for &value in double_arr.iter() {
            file.write_all(&value.to_le_bytes()).unwrap();
        }

        // MSBSingleArr
        let msb_single_arr: [f32; 2] = [12.34, 56.78];
        file.seek(SeekFrom::Start(36)).unwrap();
        for &value in msb_single_arr.iter() {
            file.write_all(&value.to_be_bytes()).unwrap();
        }

        // MSBDoubleArr
        let msb_double_arr: [f64; 2] = [98.76, 54.32];
        file.seek(SeekFrom::Start(44)).unwrap();
        for &value in msb_double_arr.iter() {
            file.write_all(&value.to_be_bytes()).unwrap();
        }

        // LSBSingleArr3Elem
        let lsb_single_arr_3_elem: [f32; 3] = [12.3, 45.6, 78.9];
        file.seek(SeekFrom::Start(60)).unwrap();
        for &value in lsb_single_arr_3_elem.iter() {
            file.write_all(&value.to_le_bytes()).unwrap();
        }

        // LSBDoubleArr3Elem
        let lsb_double_arr_3_elem: [f64; 3] = [98.7, 65.4, 32.1];
        file.seek(SeekFrom::Start(72)).unwrap();
        for &value in lsb_double_arr_3_elem.iter() {
            file.write_all(&value.to_le_bytes()).unwrap();
        }

        // ASCIIString
        let ascii_string = "HelloWorld";
        file.seek(SeekFrom::Start(96)).unwrap();
        file.write_all(ascii_string.as_bytes()).unwrap();
    }
}
