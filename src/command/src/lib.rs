pub mod option;
pub mod error;

use std::collections::{HashMap, HashSet};
use option::Opt;
use error::Error;

pub struct Args {
    options: HashMap<String, Box<Opt>>,
    notations: HashMap<String, String>,
    argCount: (i32, i32),
}

impl Args {
    pub fn new() -> Box<Self> {
        Box::new(Self{
            options: HashMap::new(),
            notations: HashMap::new(),
            argCount: (0,0)
        })

    }

    pub fn addOption(mut self: Box<Self>, option: Box<Opt>) -> Box<Self> {

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

    pub fn setArgCount(mut self: Box<Self>, min: i32, max: i32) -> Box<Self>{
    
        self.argCount = (min, max);

        self
    }
}


pub struct Command {
    subCommands: HashMap<String, Box<Command>>,
    args: Option<Box<Args>>,
    callBack: Option<fn(HashMap<String, Option<String>>, Vec<String>)>
}

impl Command {

    pub fn new() -> Box<Self> {
        Box::new(Self {
            subCommands: HashMap::new(),
            args: None,
            callBack: None
        })
    }


    pub fn addSubCommand(mut self: Box<Self>, name: &str, subCommand: Box<Command>) -> Box<Command> {

        if name.len() == 0 || name.starts_with("-") {
            panic!("Invalid name for command");
        }

        if self.subCommands.contains_key(name) {
            panic!("Subcommand already configured");
        }

        self.subCommands.insert(name.to_string(), subCommand);

        self
    }

    pub fn addArgs(mut self: Box<Self>, arg: Box<Args>) -> Box<Command> {
        
        if let Some(_) = self.args {
            panic!("Args already configured");
        }

        self.args = Some(arg);

        self

    }

    pub fn setCallBack(mut self: Box<Self>, callBack: fn(HashMap<String, Option<String>>, Vec<String>)) -> Box<Self> {

        self.callBack = Some(callBack);

        self

    }

}

enum OptionType <'a> {
    FullName(&'a str),
    Notation(&'a str)
}

impl Command {

    fn isOption( arg: &str) -> Option<OptionType> {
        let chars:Vec<char> = arg.chars().collect();

        if (arg.len() > 2) {

            if (chars[0] == '-' && chars[1] == '-') {
                return Some(OptionType::FullName(&arg[2..]));
            }

            return None
        }

        if (arg.len() == 2) {

            if (chars[0] == '-') {
                return Some(OptionType::Notation(&arg[1..]));
            }

        }

        None

    }

    fn missingRequired(options: &HashMap<String, Box<Opt>>, availOptions: &HashMap<String, Option<String>>) -> Option<String> {

        for (name, opt) in options {
            if opt.required && !availOptions.contains_key(&opt.name){
                return Some(name.to_string());
            }
        }

        None

    }

    fn runUtil(mut command: Box<Command>, tokens: &Vec<String>, ind: usize) -> Result<(), Error> {

        if ind >= tokens.len() {

            if command.callBack.is_none() {
                return Err(Error::FewArgs);
            }

            let map: HashMap<String, Option<String>> = HashMap::new();
            let vc: Vec<String> = Vec::new();

            let callBack = command.callBack.unwrap();

            if let Some(args) = command.args {
                if let Some(Opt) = Self::missingRequired(&args.options, &HashMap::new()) {
                    return Err(Error::MissingOption(Opt));
                }
            }

            callBack(HashMap::new(), Vec::new());

            return Ok(());
        }

    

        let option = Self::isOption(&tokens[ind]);

        if let None = option {
            if command.subCommands.contains_key(&tokens[ind]) {

                let nextCommand = command.subCommands.remove(&tokens[ind]).unwrap();
                return Self::runUtil(nextCommand, tokens, ind+1);

            }
        }


        // if there are still tokens left and the args is none meaning there are extra unnecessary tokens
        if command.args.is_none() { 
            return Err(Error::TooManyArgs);
        }

        let args = command.args.unwrap();

        let mut a: Vec<String> = Vec::new();
        let mut o: HashMap<String, Option<String>> = HashMap::new();

        let mut i = ind-1;
        while i < tokens.len()-1 {
            i += 1;

            let token = &tokens[i];
            
            if let Some(option) = Self::isOption(token)  {

                // check if the option is in notation form
                let optionName = match option {
                    OptionType::FullName(name) => {

                        name.to_string()
                    }

                    OptionType::Notation(name) => {
                        if !args.notations.contains_key(name) {
                            return Err(Error::UnknownOption(name.to_string()));
                        }

                        args.notations.get(name).unwrap().to_string()
                    }
                };


                match args.options.get(&optionName) {
                    Some(opt) => {   
                        
                        if opt.takesValue {
                            if i+1 >= tokens.len() {
                                return Err(Error::MissingValue(optionName));
                            }

                            i += 1;
                            o.insert(optionName.clone(), Some(tokens[i].clone()));
                        }

                        else {
                            o.insert(optionName.clone(), None);
                        }
                    }

                    None => {
                        return Err(Error::UnknownOption(optionName))
                    }
                }

            }

            else {

                a.push(token.to_string());

                if (a.len() as i32) > args.argCount.1 {
                    return Err(Error::TooManyArgs)
                }

            }

        }

        if (a.len() as i32) < args.argCount.0 {
            return Err(Error::FewArgs)
        }

        if let Some(callBack) = command.callBack {
            if let Some(opt) = Self::missingRequired(&args.options, &o) {
                return Err(Error::MissingOption(opt));
            }

            callBack(o, a);
        }


        Ok(())

    }


    pub fn run(mut self: Box<Self>, tokens: Vec<String>) -> Result<(), Error> {

        return Self::runUtil(self, &tokens, 1);

    }

}