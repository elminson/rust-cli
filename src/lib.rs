use std::fs;

pub fn read_dir(path: &str) {
    let paths = fs::read_dir(path);
    match paths {
        Ok(paths) => {
            for path in paths {
                // check if path is a directory
                if path.as_ref().unwrap().path().is_dir() {
                   read_dir(path.unwrap().path().to_str().unwrap());
                } else {
                    println!("Name: {}", path.unwrap().path().display())
                }
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }

}