use std::collections::HashMap;

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


pub struct Command {
    name: String,
    subCommand: HashMap<String, Command>,
    options: HashMap<String, Opt>,
    argCount : Option<(i32, i32)>,
    callBack: Option<fn(HashMap<String, String>, Vec<String>)>
}

impl Command {

    pub fn new(app: String) -> Self{
        return Self{
            name: app,
            subCommand: HashMap::new(),
            options: HashMap::new(),
            argCount : None,
            callBack: None
        }
    }

    pub fn addSubCommand(&mut self, subcommand: Command) -> &mut Command{

        // If Options or arguments are there, don't add subcommands
        if (self.options.len() > 0 || self.argCount.is_some()) {
            panic!("Cannot add subcommand after arguments!!");
        }

        // There should be no need for this...
        if (self.callBack.is_some()) {
            panic!("Cannot add subcommand after callback");
        }

        if (self.subCommand.contains_key(&subcommand.name)) {
            panic!("Command already present");
        }

        let subCommandName = subcommand.name.clone(); 
        
        if subCommandName.len() == 0 {
            panic!("Provide a valid name");
        }

        if subCommandName.starts_with("-") {
            panic!("Cannot have '-' in the beginning of a command name");
        }


        self.subCommand.insert(subCommandName, subcommand);

        return self;
    }

    pub fn addOption(&mut self, option: Opt) -> &mut Command {

        if (self.subCommand.len() > 0) {
            panic!("SubCommands are there, cannot add options");
        }

        if (option.name.len() == 0) {
            panic!("Provide a vaild name");
        }

        if (option.name.starts_with("-")) {
            panic!("Cannot start name with '-'");
        }

        self.options.insert(option.name.clone(), option);

        return self;
        
    }

    pub fn addArgs(&mut self, start: i32, end: i32) -> &mut Command{

        if (self.subCommand.len() > 0) {
            panic!("SubCommands are there, cannot add arguments");
        }

        self.argCount = Some((start, end));

        return self;

    }

    pub fn setCallBack(&mut self, callBack : fn(HashMap<String, String>, Vec<String>)) {

        self.callBack = Some(callBack);

    }

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

    fn runUtil(&mut self, args: Vec<String>, ind: usize) -> bool {

        todo!("Matching the args");

    }

    pub fn run(&mut self, args: Vec<String>) {
        


    }

}