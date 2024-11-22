use std::{path::{self, PathBuf}, vec};

pub struct Dir {
    path:PathBuf,
    dirs:Vec<(String, bool)>
}

impl Dir {
    pub fn new() -> Result<Self, ()> {
        let path = path::absolute(".").map_err(|_| ())?;
        let mut dirs = Self::getDirsUtil(&path).map_err(|_| ())?;



        return Ok(
            Self { path: path, dirs: dirs }
        );

    }

    fn getDirsUtil(path: &PathBuf) -> Result<Vec<(String, bool)>, ()> {

        let readDir = path.read_dir().map_err(|_| ())?;
        let mut dirs: Vec<(String, bool)> = vec![];

        for readDir in readDir {
            let currDir = readDir.map_err(|_| ())?;
            let fileName = currDir.file_name().into_string().map_err(|_| ())?;
            let mut dirPath = path.clone();
            dirPath.push(&fileName);
            dirs.push((fileName, dirPath.is_dir()));
        }
        
        dirs.sort();
        dirs.insert(0, (String::from("../"), false));

        Ok(dirs)
    }
}

impl Dir {
    pub fn getDirs(&self) -> Result<(Vec<(String,bool)>), ()> {

        return Ok(
            self.dirs.clone()
        );

    } 

    pub fn goTo(&mut self, ind: usize) -> Result<(), ()> {
        if ind == 0 {
            self.goBack();
            return Ok(());
        }
        let mut newPath = self.path.clone();
        newPath.push(&self.dirs[ind].0);

        if !newPath.is_dir() {
            return Err(());
        }

        self.path.push(&self.dirs[ind].0);
        self.dirs = Self::getDirsUtil(&self.path).unwrap();
        return Ok(());
    }
    
    pub fn goBack(&mut self) -> Result<(), ()> {
        
        if self.path.pop() {
            self.dirs = Self::getDirsUtil(&self.path).unwrap();
            return Ok(());
        }

        return Err(());
    }

    pub fn query(&self, exp: &String) -> Result<Vec<(String, bool)>, ()> {
        todo!("Have to implement it for the querying");
    }
 }
