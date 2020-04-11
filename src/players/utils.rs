use std::fmt;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
};

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerStatus {
    Batting,
    Bowling
}

#[derive(Clone, Copy)]
pub enum GameStatus {
    InProgress,
    NextOver,
    Won,
    Loss,
    Draw,
    GameOver,
}

impl fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let printable = match *self{
            PlayerStatus::Batting => "Batting",
            PlayerStatus::Bowling => "Bowling",
        };

        write!(f, "{}", printable)
    }
}