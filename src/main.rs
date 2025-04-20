#![allow(unused)]
#![allow(non_snake_case)]

mod db;
mod config;
mod command;
mod constants;

use std::{env, collections::HashMap};
use crate::command::{Command, Opt};

fn main() {

    let args = env::args(); // args
    let args: Vec<String> = args.into_iter().collect(); // converting to string vector

    // Test


}

fn save(args: HashMap<String, String>) {

    println!("Save Command Called");

}

fn ls(args: HashMap<String, String>) {

    println!("LS command called");

}