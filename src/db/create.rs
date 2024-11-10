use crate::db::{self, DB, Type};
use std::{fmt::format, fs::{self, OpenOptions}, io::Write, path::PathBuf};

impl DB {
    pub fn add(&self, key: &str, value: &str) -> Result<(), String> {

        let path = self.path.clone();

        self.isValidKey(key)?;
        self.isValidValue(value)?;
        
        match self.storageType {
            Type::Atomic => {
                let haveKey = Self::keyExist(&path, key)?;

                if haveKey {
                    return Err(String::from("Key already present"));
                }   

                let mut file = OpenOptions::new().write(true).append(true).open(path).map_err(|_| String::from("Error adding new field"))?;
                
                let res = file.write((format!("{}\n{}\n", key, value).as_bytes()));
                if res.is_err() {
                    return Err(String::from("Error adding new field"));
                }
                
            }
            Type::Composite => {
                let haveKey = Self::fileExist(&path, key)?;
                if haveKey {
                    return Err(String::from("Key already present"));
                }

                let mut filePath = path.clone();
                filePath.push(key);

                let mut file = fs::File::create(filePath).map_err(|_| String::from("Error adding new field"))?;
                
                let res = file.write(value.as_bytes());
                if res.is_err() {
                    return Err(String::from("Error adding new field"));
                }
            },  
        }

        Ok(())
    }

    fn keyExist (path: &PathBuf, key: &str) -> Result<bool, String> {
        let file = fs::read_to_string(path);

        match file {
            Ok(file) => {
                let mut ind = -1;
                for line in file.lines() {
                    ind += 1;
                    if ind % 2 != 0 {
                        continue;
                    }

                    if line == key {
                        return Ok(true);
                    }
                }
            },

            Err(_) => {
                return Err(String::from("Error reading the DB"));
            }
        }


        Ok(false)
    }

    fn fileExist(path: &PathBuf, key: &str) -> Result<bool, String> {

        let dirs = path.read_dir().map_err(|_| String::from("Error reading DB"))?;
        
        for dir in dirs {
            if dir.is_err() {
                return Err(String::from("Error reading DB"));
            }

            let dir = dir.unwrap();

            if dir.file_name() == key {
                return Ok(true);
            }
        }

        Ok(false)

    }

    
}