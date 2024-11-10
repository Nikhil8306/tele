use crate::db::{DB, Type};
use std::io::{Read, Seek, SeekFrom, Write};
use std::fs::{self, OpenOptions };

impl DB {

    pub fn remove(&self, key: &str) -> Result<(), String> {

        let path = self.path.clone();

        match &self.storageType {
            
            Type::Atomic => {

                // 

                let file = fs::read_to_string(&path).map_err(|_| String::from("Error reading DB"))?;

                let mut lines:Vec<&str> = file.split("\n").into_iter().collect();

                let mut ind = 0;
                let mut  skip = 0;
                let mut contentSize = 0;
                while ind < lines.len()-1 {
                    if lines[ind] == key {
                        contentSize = lines[ind].len() + lines[ind+1].len() + 2;
                        break;
                    }
                    skip += lines[ind].len()+1;
                    ind += 1;
                }
                

                if contentSize > 0 {
                    // what have i done 
                    let mut file = OpenOptions::new().read(true).append(true).open(&path).map_err(|_| String::from("fkng err reaading db"))?;

                    file.seek(SeekFrom::Start((contentSize+skip) as u64));
                    let mut buf = Vec::new();

                    file.read_to_end(&mut buf);
                    file.set_len(skip as u64);
                    
                    file.write_all(&buf);
                
                    return Ok(());
                }
            },

            Type::Composite => {
                
                let dirs = fs::read_dir(path).map_err(|_| String::from("Eaaa reererlerlkj"))?;


                for dir in dirs {
                    let dir = dir.map_err(|_| String::from("Error reading DB"))?;

                    if dir.file_name() == key {
                        let res = fs::remove_file(dir.path());
                        if res.is_err() {
                            return Err(String::from("Error removing key from DB"));
                        }
                        
                        return Ok(()); 
                    }
                }

            }

        }

        Err(String::from("Not such key present"))

    }

}   