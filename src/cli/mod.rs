
use std::f32::consts::E;
use std::fs;
use std::process::Command;
use std::path::{self, PathBuf};
use std::str::FromStr;
use crate::db::{self, DB, Type};
use crate::constants::{DB_NAME, EDITOR_DB};

enum CMD {
    Save,
    Remove,
    Open,
    Help,
    Show,
    Init
}

pub struct CLI {
    cmd: CMD,
    args: Vec<String>
}

impl CLI {
    
    pub fn new(cli: Vec<String>) -> Result<Self, String> {

        if cli.len() <= 1 {
            return Err(String::from("Too few arguments"));
        }

        let newCLI = match cli[1].as_str() {
            "save" => {
                CLI {
                    cmd: CMD::Save,
                    args: cli[2..cli.len()].to_vec(),
                }
            },

            "rm" | "remove" => {
                CLI {
                    cmd: CMD::Remove,
                    args: cli[2..cli.len()].to_vec()
                }
            },

            "--help" | "help" => {
                CLI {
                    cmd: CMD::Help,
                    args: cli[2..cli.len()].to_vec()
                }
            },

            "ls" => {
                CLI {
                    cmd: CMD::Show,
                    args: vec![]
                }
            },
            
            "init" => {
                CLI {
                    cmd: CMD::Init,
                    args: vec![]
                }
            }

            _ => {
                CLI {
                    cmd: CMD::Open,
                    args: cli[1..cli.len()].to_vec()
                }
            }
        };

        return Ok(newCLI);
        
            
    }


    pub fn run (&self) -> Result<(), String> {

        match self.cmd {

            CMD::Help => {
                println!("Figure it out yourself");
            },

            CMD::Open => {
                self.open()?;
            },

            CMD::Save => {  
                self.save()?;
            },

            CMD::Remove => {
                self.remove()?;
            },

            CMD::Show => {
                self.show()?;
            },

            CMD::Init => {
                self.init()?;
            }

        }


        Ok(())

    }

    fn init(&self) -> Result<(), String> {

        DB::new(DB_NAME, Type::Atomic);
        DB::new(EDITOR_DB, Type::Atomic);
        
        Ok(())

    }


    fn open(&self) -> Result<(), String>{
        
        let args = &self.args;
        let editorDB = DB::open(EDITOR_DB)?;
        let db = DB::open(DB_NAME)?;

        
        if args.len() == 0 {
            return Err(String::from("Very few arguments"));
        }

        let name = &args[0];
        let path = db.get(&name)?;
        if path.is_none() {
            return Err(String::from("No such key saved"));
        }
        let path = path.unwrap();


        let mut editorOpt= editorDB.get(&name)?;

        if editorOpt.is_none() {
            editorDB.add(&name, "code");
            editorOpt = Some(String::from("code"));
        }


        let mut editor = editorOpt.unwrap();
        let mut oldEditor = editor.clone();

        if args.len() > 1 {
            editor = args[1].clone();
        }

        let res = Command::new(&editor).arg(path).spawn();

        if res.is_err() {
            return Err(String::from("Editor not found"));
        }

        if oldEditor != editor {
            editorDB.remove(&name);
            editorDB.add(&name, &editor );
        }

        Ok(())


    }


    fn save(&self) -> Result<(), String> {
        let db = DB::open(DB_NAME)?;
        let editorDB = DB::open(EDITOR_DB)?;

        
        let args = &self.args;
        if args.len() == 0 {
            return Err(String::from("Very few arguments"));
        }

        let name = &args[0];
        let mut path = path::absolute(".").map_err(|_| String::from("Error reading the current path"))?;
                                                                                                                 
        if args.len() > 1 {
            path = path::absolute(args[1].as_str()).map_err(|_| String::from("Invalid path"))?;
            if !path.exists() {
                return Err(String::from("Invalid path"));
            }
        }

        let value = path.to_str();
        if value.is_none() {
            return Err(String::from("Invalid Path"));
        }
        
        db.add(&name, value.unwrap())?;
        editorDB.add(&name, "code")?;


        Ok(())
    }

    fn remove(&self) -> Result<(), String> {
        
        let args = &self.args;

        if args.len() == 0 {
            return Err(String::from("Too few arguments"));
        } 

        let key = &args[0];

        let db = DB::open(DB_NAME)?;
        let editorDB = DB::open(EDITOR_DB)?;

        db.remove(key)?;
        editorDB.remove(key)?;

        Ok(())
    }

    fn show(&self) -> Result<(), String> {
        
        let db = DB::open(DB_NAME)?;
        let map = db.getDB()?;

        println!();
        for pair in map {
            println!("{}  ->  {}", pair.0, pair.1);
        }
        println!();

        Ok(())
    }

}