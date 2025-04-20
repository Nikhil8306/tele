use std::collections::HashMap;

pub struct Opt {
    name: String,
    takeValue: bool,
    required: bool,
}

impl Opt {
    fn new(name: String) -> Self{
        return Self {
            name: name,
            takeValue: false,
            required: false
        };
    }

    fn takeValue(&mut self, shouldTake: bool) -> &mut Self {

        self.takeValue = shouldTake;
        
        self
    }

    fn required(&mut self, isRequired: bool) -> &mut Self {

        self. required = isRequired;

        self
    } 
}


pub struct Command {
    name: String,
    subCommand: HashMap<String, Command>,
    options: HashMap<String, Opt>,
    argCount : Option<(i32, i32)>
}

impl Command {

    fn new(app: String) -> Self{
        return Self{
            name: app,
            subCommand: HashMap::new(),
            options: HashMap::new(),
            argCount : None
        }
    }

    fn addSubCommand(&mut self, name: &String) -> &mut Command{

        // If Options or arguments are there, don't add subcommands
        if (self.options.len() > 0 || self.argCount != None) {
            panic!("Cannot add subcommand after arguments!!");
        }

        if (self.subCommand.contains_key(name)) {
            panic!("Key already present");
        }

        self.subCommand.insert(name.clone(), Command{
            name: name.clone(),
            subCommand: HashMap::new(),
            options: HashMap::new(),
            argCount: None
        });

        return self.subCommand.get_mut(name).unwrap();
    }

    fn addOptions(&mut self, options: Vec<Opt>) -> &mut Command {

        for option in options.into_iter(){
            self.options.insert(option.name.clone(), option);
        }

        return self;
        
    }

    fn addArgs(&mut self, start: i32, end: i32) -> &mut Command{

        self.argCount = Some((start, end));

        return self;

    }

}


impl Command {

    fn matches(&mut self, args: Vec<String>, ind: usize) -> bool{

        todo!("Function to match args");

    }

}