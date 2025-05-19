use std::fs::File;
use std::io::{IsTerminal, Read, stdin};

pub fn get_data(path: &String) -> Result<String, std::io::Error> {
    let mut contents = String::new();

    let result = if stdin().is_terminal() {
        let file_open = File::open(path);

        let mut file = match file_open {
            Ok(file) => file,
            Err(e) => {
                return Err(e);
            }
        };

        file.read_to_string(&mut contents)
    } else {
        stdin().read_to_string(&mut contents)
    };

    match result {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}
