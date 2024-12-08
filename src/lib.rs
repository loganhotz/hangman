use std::env;
use std::io::{
    Error,
    ErrorKind,
    stdin,
    stdout,
    Write, // required for the `stdout().flush()` method
};

pub struct HangmanGame {
    phrase: String,
    guesses: Vec<String>,
    guesses_remaining: u8,
    should_quit: bool,
}



fn load_command_line() -> Result<String, Error> {
    // `args()` returns an iterable; `.collect()` transforms that iterable
    // into a vector of `String`s. from The Book:
    // "Although we very rarely need to annotate types in Rust, `collect` is
    // one function you do often need to annotate because Rust isn’t able to
    // infer the kind of collection you want."
    let mut args: Vec<String> = env::args().collect();

    // the first command line argument is always the target directory; we
    // need to strip that before constructing the SecretPhrase
    _ = args.remove(0);

    let phrase = args.join(" ");
    if phrase.is_empty() {
        return Err(Error::from(ErrorKind::NotFound))
    }

    Ok(phrase)
}

fn prompt_command_line() -> String {
    println!("It looks like you forgot to give a secret phrase");
    print!("when starting the game. Please provide one here: ");
    let _ = stdout().flush();

    let mut s = String::new();
    let _ = stdin().read_line(&mut s);

    s
}

pub fn read_command_line() -> String {
    if let Ok(phrase) = load_command_line() {
        return phrase;
    }

    prompt_command_line()
}


const QUIT: &str = "quit";

impl HangmanGame {

    pub fn new() -> Result<Self, Error> {

        let phrase = read_command_line();
        let guesses: Vec<String> = Vec::new();
        let guesses_remaining: u8 = 7;
        let should_quit: bool = false;

        Ok(Self { phrase, guesses, guesses_remaining, should_quit })
    }

    // region: Gameplay and scheduling

    fn end(&mut self) {
        if self.guesses_remaining > 0 {
            println!("\nCongrats on winning the Rusty Hangman!");
        } else {
            print!("\nThank you for playing Rusty Hangman!\n");
            print!("\nThe secret phrase was:\n{}\n",
                self.phrase);
        }

    }

    pub fn play(&mut self) {
        self.start();

        let mut s = String::new();

        loop {
            if self.should_quit {
                break;
            }

            // `trim` strips the newline character that always follows
            // `stdin().read_line()` outputs
            stdin().read_line(&mut s).unwrap();
            self.evaluate_guess(s.trim());
            s.clear();
        }

        self.end();
    }

    fn start(&self) {
        // command characters to clear terminal and move cursor to (1, 1)
        Self::clear_terminal();
        self.display_welcome_message();
    }

    // endregion

    // region: Things relating to individual guesses

    fn add_guess(&mut self, guess: &char) {

        // we log the secret phrase and guesses as lowercase
        let clean_guess = guess.to_lowercase().to_string();
        self.check_guess(&clean_guess);

        // don't need to re-add previous guesses
        if !self.guesses.contains(&guess.to_string()) {
            self.guesses.push(clean_guess);
        }

        if self.construct_hidden_phrase() == self.phrase {
            self.should_quit = true;
        }
    }

    fn check_guess(&mut self, ch: &str) {
        if self.guesses.contains(&ch.to_string()) {
            println!("{:?} has already been guessed, goofball", ch);
        } else if !self.phrase.contains(ch) {
            self.guesses_remaining -= 1;

            if self.guesses_remaining == 0 {
                self.should_quit = true;
            }
        }
    }

    fn evaluate_guess(&mut self, guess: &str) {
        if guess == QUIT {
            self.should_quit = true;
            return;
        }

        // we can use `if let` as a shorthand for single-case matches where
        // the other option is `_ => ()`
        if let Some(c) = guess.chars().nth(0) { self.add_guess(&c) };
        self.update_display();

    }

    // endregion

    // region: Things relating to the whole phrase

    fn construct_hidden_phrase(&self) -> String {
        let mut hidden = String::new();

        for char in self.phrase.chars() {
            if self.guesses.contains(&char.to_string().to_lowercase())
                    || char.is_whitespace() {
                hidden.push(char);
            } else {
                hidden.push('_');
            }
        }

        hidden
    }

    // region: printing to the terminal

    fn clear_terminal() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn display_game_area(&self) {
        println!("\n\nThe phrase is:\n\n{}\nGuesses remaining: {}\nPast Guesses: {}",
            self.construct_hidden_phrase(),
            self.guesses_remaining,
            self.guesses.join(", "));
    }

    fn display_rules() {
        let rules = "The rules of Rusty Hangman are:
        1. To submit your guess for each round, type in a character and press
            the `Enter` key. If more than one character is given prior to
            pressing `Enter`, only the first character is accepted as a guess.
        2. The game does not treat the secret phrase as case-sensitive.
        3. If at any time you want to quit the game, submit `quit` as a guess.";
        print!("{rules}");
    }

    fn display_welcome_message(&self) {
        print!("Welcome to the Rusty Hangman!\r\n\n");
        Self::display_rules();
        print!("\n\r\nThe secret word is:\r\n{}\n\n",
            self.construct_hidden_phrase());
    }

    fn update_display(&self) {
        Self::clear_terminal();
        Self::display_rules();
        self.display_game_area();
    }

    // endregion

}
