use std::{env, fs, str::FromStr};

pub fn read_file(path: &str) -> Result<(String, String), String> {
    let mut base_path = match env::current_exe() {
        Ok(path) => path,
        Err(code) => return Err(code.to_string()),
    };

    base_path.pop();

    base_path = base_path.join(path);

    let mut base_path_clone = base_path.clone();

    base_path_clone.set_extension("");

    let base_path_str = match base_path.to_str() {
        Some(ok) => ok,
        None => return Err(String::from_str("Failed to convert PathBuf to str").unwrap()),
    };

    let file_name = match base_path_clone.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => name,
            None => return Err(String::from_str("Failed to convert file name to str").unwrap()),
        },
        None => return Err(String::from_str("Failed to get file name").unwrap()),
    };

    let content = match fs::read_to_string(base_path_str) {
        Ok(content) => content,
        Err(code) => return Err(format!("{} -> {}", base_path_str, code.to_string())),
    };

    Ok((file_name.to_string(), content))
}

pub fn read_file_full_path(path: &str) -> Result<(String, String), String> {
    let base_path = match std::path::PathBuf::from_str(path) {
        Ok(path) => path,
        Err(code) => return Err(format!("{} -> {}", path, code.to_string())),
    };

    if !base_path.is_file() {
        return Err(format!("{} -> Isn't a file", path));
    }

    let mut base_path_clone = base_path.clone();
    base_path_clone.set_extension("");

    let file_name = match base_path_clone.file_name() {
        Some(name) => match name.to_str() {
            Some(name) => name,
            None => return Err(format!("{} -> Failed to convert file name to str", path)),
        },
        None => return Err(format!("{} -> Failed to get file name", path)),
    };

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(code) => return Err(format!("{} -> {}", path, code.to_string())),
    };

    Ok((file_name.to_string(), content))
}
