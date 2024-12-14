#[allow(unused)]
// #[allow(unused_imports)]

mod game;
use game::HangmanGame;



fn main() {
    let phrase = "Roo? And Ginger?!".to_string();
    let mut game = HangmanGame::new(&phrase);

    let _ = game.play();
}
