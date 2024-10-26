#![allow(unused)]

use std::fmt::format;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::fs;
use std::env;
use std::env::Args;

fn contains_key(file: &mut std::fs::File, key:&str) -> bool {
    file.seek(SeekFrom::Start(0));
    let mut buffer = std::io::BufReader::new(& *file);

    for line in buffer.lines(){
        // println!("Hello");
        let line = line.unwrap();
        let mut a = line.split("=");
        let curr_key = a.nth(0).unwrap();

        if key == curr_key {
            return true;
        }
    }

    return false;
}

fn save_key_path(file: &mut std::fs::File, args:&mut Args) -> Result<(), String>{
    let key_opt = args.next();
    if key_opt.is_none() {
        return Err(String::from("Please provide key and value"));
    }
    let key =  key_opt.unwrap();

    let path_opt = args.next();
    let mut path = String::from(".");
    if path_opt.is_some() {
        path = path_opt.unwrap();
    }

    let abs_path = std::path::absolute(path).unwrap();

    if contains_key(file, &key) {
        return Err(String::from("Key not available"));
    }
    file.seek(SeekFrom::End(0));
    let buf = format!("{}={}\n", key, abs_path.to_str().unwrap());
    file.write(buf.as_bytes());
    Ok(())
}

fn get_path(file: &mut std::fs::File, key : &str) -> Result<String, String>{
    file.seek(SeekFrom::Start(0));
    let buffer = std::io::BufReader::new(& *file);

    for line in buffer.lines() {
        let line = line.unwrap_or(String::from(""));
        let mut split = line.split("=");
        let curr_key = split.nth(0).unwrap();
        if key == curr_key {
            return Ok(split.nth(0).unwrap().to_string());
        }
    }

    return Err(String::from("Value not found"));
}

fn run_code(file: &mut std::fs::File, args: &mut std::env::Args) -> Result<(), String> {
    let key_res = args.next();
    if key_res.is_none() {
        return Err(String::from("Provide key"));
    }

    let path_res = get_path(file, &key_res.unwrap()); 

    if path_res.is_err() {
        return Err(String::from("No such key saved!!"));
    }

    // let command = format!("code {}", path_res.unwrap());
    std::process::Command::new("code").arg(path_res.unwrap()).spawn();

    Ok(())
}

pub fn main(){
    let username_res = env::var("USER");
    if username_res.is_err() {
        println!("Error reading username ");
        return;
    }
    let username = username_res.unwrap();

    let mut tele_path = format!("/home/{}/.tele", username);
    let file_dir = Path::new(&tele_path);

    if !file_dir.exists() {
        let created = fs::create_dir(file_dir);
        if created.is_err() {
            println!("{:?}", created.unwrap_err());
            return;
        }
    }

    tele_path.push_str("/telepath");
    let file_path = Path::new(&tele_path);

    if !file_path.exists() {
        let created = fs::File::create(file_path);
        if created.is_err() {
            println!("Error creating data file");
            return;
        }
    }

    let mut file = std::fs::OpenOptions::new().read(true).append(true).open(file_path).unwrap();

    
    
    // args
    let mut args = std::env::args();
    args.next();

    let flag = args.next().unwrap();

    if flag == "-s" || flag == "--save" {
        
        let res = save_key_path(&mut file, &mut args);
        if res.is_err() {
            println!("{}", res.unwrap_err());
        }
    }   

    else if flag == "-c" || flag == "--code" {
        let res = run_code(&mut file, &mut args);
        if res.is_err() {
            println!("{}", res.unwrap_err());
        }
    }
    
}



/*

    Flags   
    -c / --code = open the following key's path in vs code
    -s / --save = Save the following path under the key 

*/