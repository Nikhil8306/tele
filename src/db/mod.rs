// Database to store key value Atomic

use std::{env, path::PathBuf, fs};
use crate::config::{self, dataDir};

pub mod create;
pub mod delete;
pub mod read;


#[derive(Debug)]
pub enum Type{ 
    Atomic,
    Composite
}

#[derive(Debug)]
pub struct DB{
    name:String,
    path:PathBuf,
    storageType:Type
} 

impl DB {
    pub fn new(name: &str, storageType:Type) -> Result<Self, String> {

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
            Type::Atomic => {
                fs::File::create(&dbPath).map_err(|_| String::from("Error creating DB"))?;
            },
            Type::Composite => {
                let dir = fs::create_dir(&dbPath).map_err(|_| String::from("Error creating DB"))?;
                
            }
        }

        return Ok(DB {
            name:name.to_string(), 
            path:dbPath,
            storageType
        });
        
    }

    pub fn open(name: &str) -> Result<Self, String> {
        let mut dbDir = config::dataDir()?;

        dbDir.push(name);

        if !dbDir.exists() {
            return Err(String::from("DB doesn't exist"));
        }

        let storageType = match dbDir.as_path().is_file() {
            true => {
                let file = fs::File::open(&dbDir).map_err(|_| String::from("Error opening DB"))?;
                Type::Atomic
            },
            false => {
                let dir = fs::read_dir(&dbDir).map_err(|_| String::from("Error opening DB"))?;

                Type::Composite
            }
        };

        return Ok(Self {
            name: name.to_string(), 
            path:dbDir,
            storageType,
        });
    }



    // 
    const EXCLUDEDCHARS: [char;2] = ['\n', '\r'];
    fn isValidKey(&self, key: &str) -> Result<(), String>{
        
        if key.contains(Self::EXCLUDEDCHARS){
            return Err(String::from("The key contains invalid characters "));
        }

        Ok(())       
    }
    
    fn isValidValue(&self, key: &str) -> Result<(), String> {

        if let Type::Atomic = self.storageType {
            
            if key.contains(Self::EXCLUDEDCHARS){
                return Err(String::from("Value contains invalid characters"));
            }

        }

        Ok(())

    }

}


