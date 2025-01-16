use std::{env, fs, str::FromStr};

pub fn read_file(path: &str) -> Result<String, String> {
    let mut base_path = match env::current_exe() {
        Ok(path) => path,
        Err(code) => return Err(code.to_string()),
    };

    base_path.pop();

    let base_path_str = match base_path.to_str() {
        Some(ok) => ok,
        None => return Err(String::from_str("Failed to convert PathBuf to str").unwrap()),
    };

    let full_path = format!("{}/{}", base_path_str, path);

    let content = match fs::read_to_string(full_path) {
        Ok(content) => content,
        Err(code) => return Err(code.to_string()),
    };

    Ok(content)
}

pub fn read_file_full_path(path: &str) -> Result<String, String> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(code) => return Err(code.to_string()),
    };

    Ok(content)
}
