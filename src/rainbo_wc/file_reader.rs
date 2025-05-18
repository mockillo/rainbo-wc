use std::fs::File;
use std::io::Read;

pub fn get_data(path: &String) -> Result<String, std::io::Error> {
    let file_open = File::open(path);

    let mut file = match file_open {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let mut contents = String::new();

    let result = file.read_to_string(&mut contents);

    match result {
        Ok(_) => Ok(contents),
        Err(e) => panic!("Error reading from file: {}", e),
    }
}
