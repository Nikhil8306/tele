use std::{fmt::write, io::stdout, path::{self, PathBuf}, vec};

pub struct Dir {
    path:PathBuf,
    dirs:Vec<(String, usize, bool)>
}

impl Dir {
    pub fn new() -> Result<Self, ()> {
        let path = path::absolute(".").map_err(|_| ())?;
        let mut dirs = Self::getDirsUtil(&path).map_err(|_| ())?;



        return Ok(
            Self { path: path, dirs: dirs }
        );

    }

    fn getDirsUtil(path: &PathBuf) -> Result<Vec<(String, usize, bool)>, ()> {

        let readDir = path.read_dir().map_err(|_| ())?;
        let mut dirs: Vec<(String, usize, bool)> = vec![];

        for readDir in readDir {
            let currDir = readDir.map_err(|_| ())?;
            let fileName = currDir.file_name().into_string().map_err(|_| ())?;
            let mut dirPath = path.clone();
            dirPath.push(&fileName);
            dirs.push((fileName, 0, dirPath.is_dir()));
        }
        
        dirs.sort();
        dirs.insert(0, (String::from("../"), 0, false));
        let mut ind = 0;
        for dir in &mut dirs{
            dir.1 = ind;

            ind += 1;
        }
        

        Ok(dirs)
    }
}

impl Dir {
    pub fn getDirs(&self) -> Result<(Vec<(String, usize, bool)>), ()> {

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

    pub fn query(&self, exp: &String) -> Result<Vec<(String, usize, bool)>, ()> {
        let mut queryRes: Vec<(String, usize, bool)> = Vec::new();

        for (dir, indx, isDir) in &self.dirs {
            if dir.contains(exp) {
                queryRes.push((dir.clone(), *indx, *isDir));
            }
        }

        return Ok(queryRes);
    }
 }
