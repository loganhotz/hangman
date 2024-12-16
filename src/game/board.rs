// I normally play hangman on a whiteboard or chalkboard, hence the name
// for the module related to rendering the game. Thought about calling it
// `gallows.rs` but I don't have that strong of a sense of gallows humor

use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::{
    Clear, ClearType,
};
use crossterm::{
    queue,
    Command,
};
use std::io::{
    stdout,
    Write,
    Error
};


#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

const GALLOWS: &str =
"
  +---+
  |   |
  |
  |
  |
  |
==========
";


pub struct GameBoard {
    pub padding: usize,
}

impl GameBoard {

    pub fn initialize() -> Result<(), Error> {
        Self::clear_screen()?;
        Self::reset_caret()?;
        Self::execute()?;


        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;

        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    pub fn reset_caret() -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row: 0 })?;
        Ok(())
    }


    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    // region: Gallows

    pub fn print_gallows(&self) -> Result<(), Error> {
        // let Position { col, .. } = at;

        for (i, gallow) in GALLOWS.lines().enumerate() {
            Self::move_caret_to(Position { col: self.padding + 3, row: i } )?;
            Self::print(gallow)?;
        }
        Ok(())
    }

    pub fn print_head(&self) -> Result<(), Error> {
        let position = Position { col: self.padding + 9, row: 3 };
        Self::move_caret_to(position)?;
        Self::print("O")?;
        
        Ok(())
    }

    pub fn print_torso(&self) -> Result<(), Error> {
        let position = Position { col: self.padding + 9, row: 4 };
        Self::move_caret_to(position)?;
        Self::print("|")?;
        
        Ok(())
    }

    pub fn print_left_arm(&self) -> Result<(), Error> {
        let position = Position { col: self.padding + 8, row: 4 };
        Self::move_caret_to(position)?;
        Self::print("/")?;
        
        Ok(())
    }

    pub fn print_right_arm(&self) -> Result<(), Error> {
        let position = Position { col: self.padding + 10, row: 4 };
        Self::move_caret_to(position)?;
        Self::print("\\")?;
        
        Ok(())
    }

    pub fn print_left_leg(&self) -> Result<(), Error> {
        let position = Position { col: self.padding + 8, row: 5 };
        Self::move_caret_to(position)?;
        Self::print("/")?;
        
        Ok(())
    }

    pub fn print_right_leg(&self) -> Result<(), Error> {
        let position = Position { col: self.padding + 10, row: 5 };
        Self::move_caret_to(position)?;
        Self::print("\\")?;
        
        Ok(())
    }

    // endregion

    // region: Command execution

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    // endregion

}
