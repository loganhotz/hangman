// I normally play hangman on a whiteboard or chalkboard, hence the name
// for the module related to rendering the game. Thought about calling it
// `gallows.rs` but I don't have that strong of a sense of gallows humor

use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode,
    size,
    Clear, ClearType,
    EnterAlternateScreen, LeaveAlternateScreen,
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
pub struct Size {
    height: usize,
    width: usize,
}

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
        // enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::reset_caret()?;
        Self::execute()?;

        let sz = Self::size()?;
        // println!("{sz:?}");
        // let _z = sz.width * sz.height;

        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        // Self::leave_alternate_screen()?;
        Self::execute()?;
        // disable_raw_mode()?;

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

    fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
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

    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated
    /// to `usize` if `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        // clippy::as_conversions: See doc above
        // #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        // clippy::as_conversions: See doc above
        // #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;

        Ok(Size { height, width })
    }

}
