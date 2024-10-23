#![allow(unused)]

use crossterm::{cursor, style, terminal, execute, terminal::ClearType};
use std::{fs::read_dir, io::{Read, Write}};


// Indentation before directory names
const IND_UNSELECTED:&str = "     ";
const IND_SELECTED:&str = " >   ";



fn main() {
    // Variables
    // Count of number of lines currently printed and current line pointer is pointing to
    let mut line_count = 0;
    let mut curr_line = 0;

    // For storing the current working directory
    let mut path = vec!["."];

    // For storing the names of the current directory
    let mut curr_directories:Vec<String> = Vec::new();

    // Flag, when to update the display
    let mut update_display = true;

    // Enabling raw mode
    terminal::enable_raw_mode();
    
        'control:loop {

            if update_display {
                update_display = false;
                
                let dir = display_dir(&path);
                if dir.is_err() {
                    display_err(dir.unwrap_err());
                    break 'control;
                }

                curr_directories = dir.unwrap();
                line_count = curr_directories.len();
                curr_line = line_count;
                // print!("{curr_line}");
                curr_line = move_cur(line_count, curr_line, 5);
                // print!("{curr_line}");

                move_cur(line_count, curr_line, 0);
            }

            let mut inp:[u8; 5] = [0;5];
            std::io::stdin().read(&mut inp).unwrap();
            
            if inp[0] == 81 || inp[0] == 113 {
                break 'control;
            }
            
            if inp[0] == 27 {
                if inp[2] == 65 {
                    update_mark(1);
                    // execute!(std::io::stdout(), cursor::MoveUp(1));
                }
                else if inp[2] == 66 {
                    update_mark(0);
                    // execute!(std::io::stdout(), cursor::MoveDown(1));
                }
            }
        }
    
    
    


    // Disabling raw mode
    terminal::disable_raw_mode();
}


// Functions

// To display directory content
fn display_dir(path: &Vec<&str>) -> Result<Vec<String>, String> {
    let entries = read_dir(path.join("/"));
    if (entries.is_err()) {
        return Err(String::from("Cannot read the directory"));
    }
    
    let read_dir = entries.unwrap();
    let mut curr_directories: Vec<String> = Vec::new();

    print!("{}../\r\n", IND_SELECTED);
    for dir in read_dir {
        if dir.is_err() {
            return Err(String::from("Error reading a specific directory"));
        }

        let curr_dir = dir.unwrap();
        let curr_dir_name = curr_dir.file_name().into_string().unwrap();
        print!("{}{}\r\n", IND_UNSELECTED, curr_dir_name);
        curr_directories.push(curr_dir_name);
    }
    execute!(std::io::stdout(), cursor::MoveUp(1));
    Ok(curr_directories)
}


// To update the mark
fn update_mark(dir: u8) {
    execute!(std::io::stdout(), cursor::MoveToColumn(0));
    unmark_dir();
    
    if dir == 1 {
        execute!(std::io::stdout(), cursor::MoveUp(1));
        cursor::MoveToColumn(0);
    }
    else {
        execute!(std::io::stdout(), cursor::MoveDown(1));
        cursor::MoveToColumn(0);
    }

    mark_dir();
    execute!(std::io::stdout(), cursor::MoveToColumn(0));
}

// To unmark the directory name
fn unmark_dir() {
    print!("{}\r", IND_UNSELECTED);
}

// To mark the directory name
fn mark_dir() {
    print!("{}\r", IND_SELECTED);
}

// To display error message
fn display_err(err:String) {
    style::SetForegroundColor(style::Color::Red);
    print!("{err}\r\n");
    style::SetForegroundColor(style::Color::Reset);
}

// To move cursor to some row
fn move_cur(lines:usize, mut curr_line:usize, target_line:usize) -> usize{
    if curr_line < 0 || curr_line > lines {
        return curr_line;
    }

    while curr_line != target_line {
        if curr_line < target_line {
            execute!(std::io::stdout(), cursor::MoveDown(1));
            curr_line+=1;
        }
        else {
            execute!(std::io::stdout(), cursor::MoveUp(1));
            curr_line-=1;
        }
    }

    curr_line
}