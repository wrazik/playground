use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn read_file() -> Result<String, Box<Error>> {
    let mut file = File::open("/proc/cpuinfo")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    match read_file() {
        Ok(content) => {
            println!("Current file content is: {}\n", content.len());
        }
        Err(error) => {
            println!("Error reading file: {}\n", error.to_string());
        }
    }
}
