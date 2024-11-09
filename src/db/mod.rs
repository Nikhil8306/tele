// Database to store key value pair

use std::{env, path::PathBuf, fs};
use crate::config::dataDir;

#[derive(Debug)]
pub enum Type{ 
    Pair,
    File
}

#[derive(Debug)]
pub struct DB{
    name:String,
    path:PathBuf,
    storageType:Type
} 

impl DB {
    pub fn new(name: &str, storageType: Type) -> Result<Self, String> {
        if isInvalidName(name) {
            return Err(String::from("Invalid DB name"));
        }
        let mut dbPath = dataDir()?;

        let files = dbPath.read_dir().map_err(|_| String::from("Error reading the DB directory"))?;

        for file in files {
            let file = file.map_err(|_| String::from("Error reading the DB directory"))?;
            
            if file.file_name().to_str().unwrap_or("") == name {
                return Err(String::from("DB already exists"));
            }
        }
        
        dbPath.push(name);
        
        match &storageType {
            Type::Pair => {
                let created = fs::File::create(&dbPath);
                if created.is_err() {
                    return Err(String::from("Error creating the DB"));
                }
            },
            Type::File => {
                let created = fs::create_dir(&dbPath);
                if created.is_err() {
                    return Err(String::from("Error creating the DB"));
                }
            }
        }

        return Ok(DB {
            name:name.to_string(), 
            path:dbPath,
            storageType
        });
        
    }


}


fn isInvalidName(name:&str) -> bool {
    return false;
    todo!();
}