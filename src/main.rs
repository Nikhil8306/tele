#![allow(unused)]
#![allow(non_snake_case)]

mod db;
mod config;
mod cli;
mod constants;

use std::env;
use crate::cli::CLI;


fn main() {

    let args = env::args();
    let args: Vec<String> = args.into_iter().collect();

    let cli = CLI::new(args).unwrap();
    let res = cli.run();
    if res.is_err() {
        println!("{}", res.unwrap_err());
    }

}   
 