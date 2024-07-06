use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub fn json_data(new_profile_json: &str) -> std::io::Result<()> {
    let dir_path = Path::new("data");
    let file_path = dir_path.join("profile.json");

    fs::create_dir_all(dir_path)?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)?;

    let mut existing_content = String::new();
    file.read_to_string(&mut existing_content)?;

    let mut json_value: Value = if existing_content.is_empty() {
        json!({ "profiles": [] })
    } else {
        serde_json::from_str(&existing_content)?
    };

    let new_profile: Value = serde_json::from_str(new_profile_json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    if let Some(profiles) = json_value["profiles"].as_array_mut() {
        profiles.push(new_profile);
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "JSON não contém um array 'profiles'",
        ));
    }

    let formatted_json = serde_json::to_string_pretty(&json_value)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;
    file.write_all(formatted_json.as_bytes())?;

    Ok(())
}
