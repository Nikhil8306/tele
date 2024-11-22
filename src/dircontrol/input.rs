use std::io::{stdin, Read};

#[derive(Debug)]
pub enum Command {
    Quit,
    Up,
    Down,
    Return,
    Delete,
    Query(Vec<u8>)
}

pub struct Input {
    buffer: Vec<u8>
}

impl Input {
    pub fn new() -> Self {
        Self {
            buffer: vec![]
        }
    }
}

impl Input {
    pub fn read(&mut self) -> Option<Command> {

        let mut inpBuffer = [0_u8;5];
        stdin().read(&mut inpBuffer);

        
        if inpBuffer[0] == 113 || inpBuffer[0] == 81 {
            self.clearBuffer();
            return Some(Command::Quit);
        }
        
        if inpBuffer[0] == 27 && inpBuffer[2] == 65 {
            self.clearBuffer();
            return Some(Command::Up);
        }
        
        if inpBuffer[0] == 27 && inpBuffer[2] == 66 {
            self.clearBuffer();
            return Some(Command::Down);
        }
        
        if inpBuffer[0] == 13 {
            self.clearBuffer();
            return Some(Command::Return);
        }

        if inpBuffer[0] == 127 {
            self.clearBuffer();
            return Some(Command::Delete);
        }
        

        if (Self::isBufferAllowed(inpBuffer)) {
            self.buffer.push(inpBuffer[0]);
        }

        return Some(Command::Query(self.buffer.clone()));
    }

    pub fn clearBuffer(&mut self) {
        self.buffer.clear();
    }

    pub fn isBufferAllowed(buffer: [u8; 5]) -> bool {

        if buffer[0] < 32 {
            return false;
        }

        if buffer[0] == 47 {
            return false;
        }

        if buffer[0] > 126 {
            return false;;
        }

        return true;

    }
}