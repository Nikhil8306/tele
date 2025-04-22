#![allow(unused)]
#![allow(non_snake_case)]

mod db;
mod config;
mod command;
mod constants;
mod error;

use std::{collections::HashMap, env, hash::Hash};
use command::Args;

use crate::command::{Command, option::Opt};

fn main() {

    let args = env::args(); // args
    let args: Vec<String> = args.into_iter().collect(); // converting to string vector

    // Test

    let command = 
        Command::new()
        .addSubCommand("push", Command::new().setCallBack(pushCallBack))
        .addSubCommand("init", Command::new() 
            .addArgs(Args::new().setArgCount(0, 2).addOption(Opt::new("temp").notation("t").required(true).takesValue(true)))
            .setCallBack(initCallBack)
        );

    command.run(vec![String::from("tele"), String::from("init"), String::from("somethign"), String::from("-t"), String::from("tmepvalue"), String::from("Something12")]);
}

fn pushCallBack(options: HashMap<String, Option<String>>, args: Vec<String>) {
    println!("THis is a push command");

    println!("{:?}, {:?}", options, args);
}

fn initCallBack(options: HashMap<String, Option<String>>, args: Vec<String>) {
    println!("This is a init command");

    println!("{:?}, {:?}", options, args);
}