#[macro_use]
mod utils;

mod errors;
mod game;
mod player;
mod toss;
mod umpire;


use std::io::{self};

pub use crossterm::Result;

fn main() -> Result<()> {
        let mut stderr = io::stdout();
        umpire::toss(&mut stderr)
}
