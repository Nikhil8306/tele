use std::{collections::HashMap, hash::Hash};

pub struct Opt {
    name: String,
    notation: String,
    takeValue: bool,
    required: bool,
}

impl Opt {
    pub fn new(name: String, notation: String) -> Self{
        return Self {
            name,
            notation,
            takeValue: false,
            required: false
        };
    }

    pub fn takeValue(&mut self, shouldTake: bool) -> &mut Self {

        self.takeValue = shouldTake;
        
        self
    }

    pub fn required(&mut self, isRequired: bool) -> &mut Self {

        self. required = isRequired;

        self
    } 
}
struct Args{
    options: HashMap<String, Opt>,
    argCount: Option<(i32, i32)>,
    callBack: Option<fn(HashMap<String, String>, Vec<String>)>
}

impl Args {
    fn new() -> Self{
        return Self {
            options: HashMap::new(),
            argCount: None,
            callBack: None
        }
    }
}

pub enum Command {
    SubCommand (String, Vec<Command>),
    Args (Args),
}


impl Command {

    pub fn new(app: String) -> Self{
        
        return Command::SubCommand(app, Vec::new());

    }

    pub fn addSubCommand(&mut self, mut subcommand: Command) -> &mut Command{

        return match self{
            Self::Args(_) => {
                panic!("Cannot add subcommand after arguments");
            }

            Self::SubCommand(name, commands) => {
                commands.push(subcommand);
                let ind = commands.len()-1;

                commands.get_mut(ind).unwrap()
            }
        };

    }

    fn addOptionUtil(arg: &mut Command, option: Opt) {
        if let Command::Args(arg) = arg {
            arg.options.insert(option.name.clone(), option);
        }
    }

    pub fn addOption(&mut self, option: Opt) -> &mut Command {

        let c =  match self {

            Command::SubCommand(_, commands) => {
                
                let mut arg = Args::new();

                commands.push(Command::Args(arg));
                let ind = commands.len()-1;

                commands.get_mut(ind).unwrap()
            }

            Command::Args(arg) => {

                self
            }

        };

        Self::addOptionUtil(c, option);

        return c;
    }

    fn addArgsUtil(arg: &mut Command, start: i32, end: i32) {
        if let Command::Args(arg) = arg {
            arg.argCount = Some((start, end));
        }
    } 

    pub fn addArgs(&mut self, min: i32, max: i32) -> &mut Command{

        if min > max  {
            panic!("Min count is greater than Max Count");
        }

        let c =  match self {

            Command::SubCommand(_, commands) => {
                
                let mut arg = Args::new();

                commands.push(Command::Args(arg));
                let ind = commands.len()-1;

                commands.get_mut(ind).unwrap()
            }

            Command::Args(arg) => {

                self
            }

        };

        Self::addArgsUtil(c, min, max);

        return c;

    }

    fn setCallBackUtil(arg: &mut Command, callback: fn(HashMap<String, String>, Vec<String>)) {
        if let Command::Args(arg) = arg {
            arg.callBack = Some(callback);
        }
    }

    pub fn setCallBack(&mut self, callBack : fn(HashMap<String, String>, Vec<String>)) {

        let c =  match self {

            Command::SubCommand(_, commands) => {
                
                let mut arg = Args::new();

                commands.push(Command::Args(arg));
                let ind = commands.len()-1;

                commands.get_mut(ind).unwrap()
            }

            Command::Args(arg) => {

                self
            }

        };

        Self::setCallBackUtil(c, callBack);

    }

}   

struct Err {
    // Temp
}

impl Command {

    fn isOption( arg: &str) -> Option<&str> {
        let chars:Vec<char> = arg.chars().collect();

        if (arg.len() > 2) {

            if (chars[0] == '-' && chars[1] == '-') {
                return Some(&arg[2..]);
            }

        }

        if (arg.len() > 1) {

            if (chars[0] == '-') {
                return Some(&arg[1..]);
            }

        }

        None

    }

    fn runUtil(command: &Command, args: &Vec<String>, ind: usize) -> Result<(HashMap<String, Option<String>>, Vec<String>), Err> {

        todo!("");
        
    }

    pub fn run(&mut self, args: Vec<String>) {
        


    }

}