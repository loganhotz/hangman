use hangman::HangmanGame;

fn main() {

    let Ok(mut game) = HangmanGame::new() else {
        panic!("oh no!")
    };

    game.play();
}
