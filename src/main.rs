use std::env;

mod game;
use game::HangmanGame;

fn play() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if let Some(phrase_parts) = args.get(1..args.len()) {
        let phrase = phrase_parts.join(" ");
        let mut game = HangmanGame::new(&phrase);

        game.play()?;
    }

    Ok(())
}



fn main() -> Result<(), std::io::Error> {
    play()?;
    Ok(())
}
