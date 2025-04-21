mod option;

use std::collections::{HashMap, HashSet};
use option::Opt;
use crate::error::Error;

pub struct Args {
    options: HashMap<String, Opt>,
    notations: HashMap<String, String>,
    argCount: (i32, i32),
}

impl Args {
    pub fn new() -> Self {
        Self{
            options: HashMap::new(),
            notations: HashMap::new(),
            argCount: (0,0)
        }

    }

    pub fn addOption(&mut self, option: Opt) -> &mut Self {

        let name = option.name.clone();
        let notation = option.notation.clone();

        if name.len() == 0 || name.starts_with("-") {
            panic!("Invalid name for option");
        }

        if self.options.contains_key(&name) {
            panic!("Option already configured");
        }

        if let Some(n) = notation {
            if n.len() != 1 || n.starts_with("-"){
                panic!("Invalid notation");
            }

            self.notations.insert(n, name.clone());
            
        }

        self.options.insert(name, option);

        self
    }

    pub fn addArgCount(&mut self, min: i32, max: i32) -> &mut Self {
    
        self.argCount = (min, max);

        self
    }
}


pub struct Command {
    subCommands: HashMap<String, Command>,
    args: Option<Args>,
    callBack: Option<fn(HashMap<String, Option<String>>, Vec<String>)>
}

impl Command {

    pub fn new() -> Self {
        Self {
            subCommands: HashMap::new(),
            args: None,
            callBack: None
        }
    }


    fn addSubCommand(&mut self, name: String, subCommand: Command) -> &mut Self {

        if name.len() == 0 || name.starts_with("-") {
            panic!("Invalid name for command");
        }

        if self.subCommands.contains_key(&name) {
            panic!("Subcommand already configured");
        }

        self.subCommands.insert(name, subCommand);

        self
    }

    fn addArgs(&mut self) -> &mut Args {
        
        if let Some(_) = self.args {
            panic!("Args already configured");
        }

        self.args = Some(Args::new());

        return match &mut self.args {
            Some(a) => {
                a
            }
            None => {
                panic!("Something went wrong adding argument");
            }
        }

    }

    fn setCallBack(&mut self, callBack: fn(HashMap<String, Option<String>>, Vec<String>)) -> &mut Self {

        self.callBack = Some(callBack);

        self

    }

}

impl Command {

    fn isOption( arg: &str) -> Option<&str> {
        let chars:Vec<char> = arg.chars().collect();

        if (arg.len() > 2) {

            if (chars[0] == '-' && chars[1] == '-') {
                return Some(&arg[2..]);
            }

            return None
        }

        if (arg.len() == 2) {

            if (chars[0] == '-') {
                return Some(&arg[1..]);
            }

        }

        None

    }

    fn checkRequired(options: &HashMap<String, Opt>, availOptions: &HashMap<String, Option<String>>) -> Option<String> {

        for (name, opt) in options {
            if opt.required && !availOptions.contains_key(&opt.name){
                return Some(name.to_string());
            }
        }

        None

    }

    fn runUtil(command: &Command, tokens: &Vec<String>, ind: usize) -> Result<(), Error> {

        if ind >= tokens.len() {

            if command.callBack.is_none() {
                return Err(Error{}); // TODO: Return too few arguments error
            }

            let map: HashMap<String, Option<String>> = HashMap::new();
            let vc: Vec<String> = Vec::new();

            let callBack = command.callBack.unwrap();

            if let Some(args) = &command.args {
                if let Some(Opt) = Self::checkRequired(&args.options, &HashMap::new()) {
                    return Err(Error {  }); // TODO: return error particular options is required
                }
            }

            callBack(HashMap::new(), Vec::new());

            return Ok(());
        }

    

        let option = Self::isOption(&tokens[ind]);

        if let None = option {
            if command.subCommands.contains_key(&tokens[ind]) {

                return Self::runUtil(command.subCommands.get(&tokens[ind]).unwrap(), tokens, ind+1);

            }
        }


        // if there are still tokens left and the args is none meaning there are extra unnecessary tokens
        if command.args.is_none() { 
            return Err(Error{}); // TODO: return too many arguments
        }

        let args = command.args.as_ref().unwrap();

        let mut a: Vec<String> = Vec::new();
        let mut o: HashMap<String, Option<String>> = HashMap::new();

        let mut i = ind-1;
        while i < tokens.len() {
            i += 1;

            let token = &tokens[i];
            
            if let Some(mut option) = Self::isOption(token)  {

                // check if the option is in notation form
                match args.notations.get(option) {
                    Some(opt) => {
                        option = opt;
                    }

                    None => {

                    }
                }


                match args.options.get(option) {
                    Some(opt) => {   
                        
                        if opt.takesValue {
                            if i+1 >= tokens.len() {
                                return Err(Error{}); // TODO: return error that no value for particular option
                            }

                            i += 1;
                            o.insert(option.to_string(), Some(tokens[i].clone()));
                        }

                        else {
                            o.insert(option.to_string(), None);
                        }
                    }

                    None => {
                        return Err(Error {  }) // TODO: return unknown options error
                    }
                }

            }

            else {

                a.push(token.to_string());

                if (a.len() as i32) > args.argCount.1 {
                    return Err(Error {}) // TODO: return error too many arguments
                }

            }

        }

        if (a.len() as i32) < args.argCount.0 {
            return Err(Error {  }) // TODO: return error too few arguments
        }

        if let Some(callBack) = command.callBack {
            if let Some(opt) = Self::checkRequired(&args.options, &o) {
                return Err(Error {  }); // TODO: return error particular argument is required
            }

            callBack(o, a);
        }


        Ok(())

    }

    pub fn run(&mut self, tokens: Vec<String>) -> Result<(), Error> {

        return Self::runUtil(self, &tokens, 0);

    }

}