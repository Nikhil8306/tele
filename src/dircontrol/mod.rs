use crossterm::{terminal::enable_raw_mode, terminal::disable_raw_mode, cursor, execute};
use std::io::{stdin, stdout, Read, Write};
use std::path::{self, PathBuf};
use std::write;
use input::{Command, Input};
use dir::Dir;
use render::Screen;

mod render;
mod input;
mod dir;

pub fn main() {

    enable_raw_mode();

    execute!(stdout(), cursor::Hide);
    {
        
        let mut dir = Dir::new().unwrap();
        let mut input = Input::new();
        let mut screen = Screen::new(&dir.getDirs().unwrap());
        
        
        loop {
            
            let inp = input.read().unwrap();
            
            match inp {
                Command::Down => {
                    screen.cursorDown();
                },
                
                Command::Up => {
                    screen.cursorUp();
                },
                
                Command::Quit => {
                    screen.clearScreenBuffer();
                    break;
                },
                
                Command::Return => {
                    let cursorPos = screen.getCursorPos();
                    dir.goTo(cursorPos);
                    screen.reset(dir.getDirs().unwrap());
                },
                
                _ => {
                    
                }
            }
            
        }
        
    }
    
    
    execute!(stdout(), cursor::Show);
    disable_raw_mode();


}
