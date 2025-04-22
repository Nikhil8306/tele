#![allow(unused)]
#![allow(non_snake_case)]

mod db;
mod config;
mod constants;

use std::{collections::HashMap, env, hash::Hash};
use command::Args;

use command::{Command, option::Opt};


fn main() {

    let args = env::args(); // args
    let args: Vec<String> = args.into_iter().collect(); // converting to string vector

    // Test

    let command = Command::new()
        .addSubCommand("save",
        Command::new()
            .addArgs(Args::new()
                .addOption(Opt::new("editor")
                    .notation("e")
                    .takesValue(true)
                )
                .addOption(Opt::new("name")
                    .notation("n")
                    .takesValue(true)
                )
                .addOption(Opt::new("dir")
                    .notation("d")
                    .takesValue(true)
                )
            )
            .setCallBack(saveCallBack)
        )
        .addSubCommand("ls", 
            Command::new()
                .addArgs(Args::new()
                    .addOption(Opt::new("dir")
                        .takesValue(true)
                        .notation("d")
                    )
                    .addOption(Opt::new("name")
                        .takesValue(true)
                        .notation("n")
                    )
                )
                .setCallBack(lsCallback)
        )
        .addSubCommand("update", 
            Command::new()
                .addArgs(Args::new()
                    .addOption(Opt::new("dir")
                        .notation("d")
                        .takesValue(true)
                        .required(true)
                    )

                    .addOption(Opt::new("name")
                        .notation("n")
                        .takesValue(true)
                        .required(true)
                    )
                )
                .setCallBack(updateCallBack)
        )
        .addSubCommand("remove", 
            Command::new()
                .addArgs(Args::new().setArgCount(2, 2))
                .setCallBack(removeCallBack)
        )
        .addSubCommand("init", 
            Command::new()
                .setCallBack(initCallBack)
        );

        command.run(args);
}

fn saveCallBack(options: HashMap<String, Option<String>>, args: Vec<String>) {
    println!("Save {:?} {:?}", options, args);
}

fn updateCallBack(options: HashMap<String, Option<String>>, args: Vec<String>) {
    println!("Update {:?} {:?}", options, args);
}

fn removeCallBack(options: HashMap<String, Option<String>>, args: Vec<String>) {
    println!("Remove {:?} {:?}", options, args);
}

fn initCallBack(options: HashMap<String, Option<String>>, args: Vec<String>) {
    println!("Init {:?} {:?}", options, args);
}

fn lsCallback(options: HashMap<String, Option<String>>, args: Vec<String>) {
    println!("List {:?} {:?}", options, args);
}