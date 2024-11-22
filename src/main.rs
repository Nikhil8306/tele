#![allow(unused)]
#![allow(non_snake_case)]


mod dircontrol;
mod teleport;
mod db;
mod config;
mod cli;
mod constants;

use std::env;
use crate::cli::CLI;



fn main(){
    // todo: Need Some error pass instead of string

    let args = env::args();
    let args: Vec<String> = args.into_iter().collect();

    if args.len() == 1 {
        dircontrol::main();
    }
    else {
        let cli = CLI::new(args).unwrap();

        let res = cli.run();

        if res.is_err() {
            println!("{}", res.unwrap_err());
        }
    }
}   