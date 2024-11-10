use std::path::PathBuf;
use std::{env, fs};

pub fn dataDir() -> Result<PathBuf, String> {
    let username = env::var("USER").map_err(|_| "username not found".to_string())?;

    let dir = format!("/home/{}/.tele/", username);

    let dirPath = PathBuf::from(&dir);

    if !dirPath.exists() {
        let success = fs::create_dir(&dir);
        if let Err(_) = success {
            return Err(String::from("Cannot create database"));
        }

    }

    Ok(dirPath)

}

