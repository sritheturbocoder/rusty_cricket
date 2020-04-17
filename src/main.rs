#[macro_use]
mod macros;

mod errors;
mod toss;
mod players;
mod game;
mod umpire;


use std::io::{self};
pub use crossterm::{Result,};
    
fn main() -> Result<()> {
        let mut stderr = io::stdout();
        umpire::toss(&mut stderr)
}