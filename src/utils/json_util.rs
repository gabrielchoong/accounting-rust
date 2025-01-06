// json utilities

use serde::de::DeserializeOwned;
use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};

pub fn append_to_file(file_path: &str, data: &String) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    writeln!(file, "{},", data)?;

    Ok(())
}

pub fn retrieve_json_data<T: DeserializeOwned>(file_path: &str) -> Result<(T, String)> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let data: T = serde_json::from_str(&content)?;

    Ok((data, content))
}
