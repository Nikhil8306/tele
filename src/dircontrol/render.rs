use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType}, style::{SetForegroundColor, Color, ResetColor}, execute, cursor::{MoveUp}};
use std::{fmt::write,io::{Stdout, Write}};
use std::write;

use super::dir::Dir;

pub struct Screen {
    buffer: Vec<(String, bool)>,
    cursorPos: usize,
    output:Stdout
}

impl Screen {
    // associate functions
    pub fn new(buffer: &Vec<(String, bool)>) -> Self {
        let mut newScreen = Self {
            buffer: buffer.clone(),
            cursorPos:0,
            output:std::io::stdout()
        };

        newScreen.render();
        return newScreen;
    }

}


impl Screen {
    // methods
    pub fn reset(&mut self, buffer: Vec<(String, bool)>) {

        self.clearScreenBuffer();
        self.cursorPos = 0;
        self.buffer = buffer;
        self.render();

    }

    pub fn cursorDown(&mut self) {

        self.cursorPos = (self.cursorPos+1)%self.buffer.len();
        self.clearScreenBuffer();
        self.render();

    }
    
    pub fn cursorUp(&mut self) {

        if self.cursorPos > 0 {
            self.cursorPos -= 1;
        }
        else {
            self.cursorPos = self.buffer.len()-1;
        }
        self.clearScreenBuffer();
        self.render();
    }

    pub fn getCursorPos(&self) -> usize {
        return self.cursorPos;
    }

    pub fn render(&mut self) {
        let mut ind = 0;
        for (dir, isDir) in &(self.buffer) {
            if *isDir {
                execute!(self.output, SetForegroundColor(Color::Blue));
            }
            let mut sign = " ";
            if ind == self.cursorPos {
                sign = ">";
            }
            write!(self.output, " {}   {}\r\n", sign, dir);
            if *isDir {
                execute!(self.output, SetForegroundColor(Color::Reset));
            }

            ind += 1;
        }
    }

    pub fn clearScreenBuffer(&mut self) {
        for elem in &(self.buffer) {
            execute!(self.output, MoveUp(1));
            execute!(self.output, Clear(ClearType::CurrentLine));
        }
    }
}



