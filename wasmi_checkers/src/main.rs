extern crate wasmi;

mod checkersgame;
mod imports;
mod runtime;

use checkersgame::CheckersGame;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut game = CheckersGame::new("../checkers/checkers.wasm");
    game.display_board();
    game.move_piece(&(0, 5), &(0, 4))?;
    game.move_piece(&(0, 1), &(0, 2))?;
    game.move_piece(&(0, 4), &(0, 3))?;
    Ok(())
}