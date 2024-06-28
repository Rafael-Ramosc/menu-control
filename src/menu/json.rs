use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn json_data(json: &String) -> std::io::Result<()> {
    let dir_path = Path::new("data");
    let file_path = dir_path.join("user.json");

    fs::create_dir_all(dir_path)?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(&file_path)?;

    let mut json_content = String::new();
    file.read_to_string(&mut json_content)
        .expect("Error when trying to read json file!");

    if json_content.is_empty() {
        json!({ "users": []});
    } else {
        serde_json::from_str(&json_content)?;
    }

    //todo: acrescentar novo usuario no array users

    write!(file, "{},", json)?;

    Ok(())
}
