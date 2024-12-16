use crossterm::event::{
    read,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
    KeyModifiers,
};
use std::ops::Range;
use std::fmt;
use std::io::Error;

mod board;
use board::{
    GameBoard,
    Position,
};

// bit ranges for capital and lowercase alphanumeric characters with *NO*
// diacritics. Note since `Range` is a half-open interval, the second-to-last
// element of each range corresponds to Z and z, respectively
const CAPITAL: Range<u8> = 65..91;
const LOWERCASE: Range<u8> = 97..123;

// fixed locations in game region
const PROMPT_LOCATION: Position = Position { row: 2, col: 1 };
const PHRASE_LOCATION: Position = Position { row: 4, col: 1 };
const GUESS_LOCATION: Position = Position { row: 8, col: 1 };



// A custom error type to be thrown when something
// inappropriate happens with a user's guess
#[derive(Debug, Clone)]
struct GuessError;

impl fmt::Display for GuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unacceptable guess")
    }
}


// An iterator to read in multiple key presses from the user
struct GuessEvent;

impl Iterator for GuessEvent {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(event) = read() {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) = event {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => None,
                    (KeyCode::Char(_), _) => Some(event),
                    (KeyCode::Enter, _) => None,
                    _ => None,
                }
            } else {
                None
            }
        } else {
            // if we can't read properly; just exit iteration
            None
        }
    }
}


pub struct HangmanGame {
    phrase: String,
    lives: u8,
    guesses: Vec<char>,
    should_quit: bool,
    board: GameBoard,
}

impl HangmanGame {

    pub fn new(phrase: &str) -> Self {

        validate_phrase(phrase);
        let lives: u8 = 6;
        let guesses: Vec<char> = Vec::new();
        let should_quit = false;
        let board = GameBoard { padding: phrase.len() };

        Self {
            phrase: phrase.to_string(),
            lives,
            guesses,
            should_quit,
            board,
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {

        GameBoard::initialize()?;

        while !self.should_quit {
            GameBoard::clear_screen()?;
            GameBoard::reset_caret()?;

            self.board.print_gallows()?;
            self.print_body()?;
            self.print_guess_list()?;

            self.issue_hidden_phrase()?;
            Self::prompt_guess()?;

            GameBoard::execute()?;

            let guess = self.read_guess();
            self.evaluate_guess(&guess);
        }

        self.issue_goodbye()?;

        GameBoard::terminate()?;
        
        Ok(())
    }

    // region: Handling Single Guesses

    fn read_guess(&self) -> String {
        let mut full_guess = String::new();

        for event in GuessEvent {
            if let Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) = event {
                full_guess.push(c);
            }
        }

        full_guess
    }

    fn evaluate_guess(&mut self, guess: &str) {
        if let Some(c) = guess.chars().nth(0) {
            // if `guess` is an empty string, `nth()` returns `None`
            // self.add_guess(c);
            self.update_guesses(c);
        }
    }

    fn update_guesses(&mut self, guess: char) {
        if let Ok(lower) = lowercase(guess) {
            self.add_guess(lower);
        } else {
            self.issue_invalid_guess(&guess);
        }

    }

    fn add_guess(&mut self, guess: char) {
        if self.guesses.contains(&guess) {
            self.issue_duplicate_guess(&guess);
        } else {
            self.guesses.push(guess);
            if self.phrase.contains(guess) {
                self.issue_correct_guess(&guess);
            } else {
                self.issue_incorrect_guess(&guess);

                self.lives -= 1;
                if self.lives == 0 {
                    self.should_quit = true;
                }
            }
        }
        
    }

    // endregion

    // region: Communicating with player

    fn construct_hidden_phrase(&self) -> String {
        let mut phrase = String::new();

        for (c, b) in self.phrase.chars().zip(
            self.phrase.as_bytes().iter()) {

            if is_alpha_utf(b) {
                if let Ok(lower) = lowercase(c) {
                    if self.guesses.contains(&lower) {
                        phrase.push(c);
                    } else {
                        phrase.push('_');
                    }
                }
            } else {
                phrase.push(c);
            }

        }

        phrase
    }

    fn issue_correct_guess(&mut self, _guess: &char) {
        // println!("`{}` is in the secret phrase!", guess);
        if self.construct_hidden_phrase() == self.phrase {
            self.should_quit = true;
        }
    }

    fn issue_duplicate_guess(&self, _guess: &char) {
        // println!("`{}` has already been guessed", guess);
    }

    fn issue_hidden_phrase(&self) -> Result<(), Error> {
        GameBoard::move_caret_to(PHRASE_LOCATION)?;
        GameBoard::print(&self.construct_hidden_phrase())?;

        Ok(())
    }

    fn issue_incorrect_guess(&self, _guess: &char) {
        // println!("`{}` is not in the secret phrase :(", guess);
    }

    fn issue_invalid_guess(&self, _guess: &char) {
        // println!("invalid guess: `{}`", guess);
    }

    fn issue_goodbye(&self) -> Result<(), Error> {
        self.issue_hidden_phrase()?;
        GameBoard::execute()?;

        if self.lives > 0 {
            GameBoard::move_caret_to(Position { row: 10, col: 1 })?;
            GameBoard::print("Congratulations, you won Rusty Hangman!")?;
        } else {
            // before exiting the game we add the last limb to the hangman,
            // and fill in the secret word
            self.print_body()?;
            GameBoard::move_caret_to(PHRASE_LOCATION)?;
            GameBoard::print(&self.phrase)?;

            GameBoard::move_caret_to(Position { row: 10, col: 1 })?;
            GameBoard::print("You were not able to figure out the secret phrase :(\n")?;
            GameBoard::print("\r\nThank you for playing Rusty Hangman.\n")?;
        }

        Ok(())
    }

    fn prompt_guess() -> Result<(), Error> {
        GameBoard::move_caret_to(PROMPT_LOCATION)?;
        GameBoard::print("Guess: ")?;

        let Position { row, col } = PROMPT_LOCATION;
        GameBoard::move_caret_to(Position { row, col: col + 7 })?;

        Ok(())
    }

    fn print_body(&self) -> Result<(), Error> {
        if self.lives == 6 {
            return Ok(());
        }

        if self.lives <= 5 { self.board.print_head()?; }
        if self.lives <= 4 { self.board.print_torso()?; }
        if self.lives <= 3 { self.board.print_left_arm()?; }
        if self.lives <= 2 { self.board.print_right_arm()?; }
        if self.lives <= 1 { self.board.print_left_leg()?; }
        if self.lives == 0 { self.board.print_right_leg()?; }

        Ok(())
    }

    fn print_guess_list(&self) -> Result<(), Error> {
        GameBoard::move_caret_to(GUESS_LOCATION)?;
        GameBoard::print("Guesses")?;

        let Position { col, row } = GUESS_LOCATION;
        GameBoard::move_caret_to( Position { col: col + 1, row: row + 1 })?;

        let guess_list = self.guesses
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        GameBoard::print(&guess_list)?;

        Ok(())
    }

    // endregion

}

/// const fn(byte: u8) -> bool
///
/// Check if the given byte corresponds to an alphabetic
/// character. This function is case-agnostic, and is
/// equivalent to checking if the byte's character representation
/// resides in the regex range [a-zA-Z]
fn is_alpha_utf(byte: &u8) -> bool {
    CAPITAL.contains(byte) || LOWERCASE.contains(byte)
}

/// const fn(ch: char) -> char
///
/// Ensure the given character is lowercase.
fn lowercase(ch: char) -> Result<char, GuessError> {
    let chu8 = ch as u8;

    if LOWERCASE.contains(&chu8) {
        Ok(ch)
    } else if CAPITAL.contains(&chu8) {
        Ok( (chu8 + 32) as char )
    } else {
        Err(GuessError)
    }
}


/// const fn() -> Result<String, Error>
///
/// Verify that all the characters in the secret phrase are
/// UTF-8 characters whose decimal representation is <128.
fn validate_phrase(phrase: &str) {
    let it = phrase.chars().zip(phrase.as_bytes().iter());

    for (i, (c, b)) in it.enumerate() {
        if *b >= 128 {
            panic!("Character `{}` at index `{}` is not valid", c, i);
        }
    }
}
