use crate::utils::utils::{PlayerStatus, GameStatus};
use crossterm::{Result};

pub type Score = u16;

pub trait Bat {
    fn bat(&self) -> Result<Score>;
}

pub trait Bowl {
    fn bowl(&self) -> Result<Score>;
}

pub struct Human {
}

pub struct Computer {
}

impl Bat for Human {
    fn bat(&self) -> Result<Score> {
        unimplemented!();
    }
}

impl Bowl for Human {
    fn bowl(&self) -> Result<Score> {
        unimplemented!();
    }
}

impl Bat for Computer {
    fn bat(&self) -> Result<Score> {
        unimplemented!();
    }
}

impl Bowl for Computer {
    fn bowl(&self) -> Result<Score> {
        unimplemented!();
    }
}

pub struct Player<T> 
{
    pub runs :u16,
    pub wickets :u16,
    pub bowled_balls :u16,
    pub required_runrate :f32,
    pub current_runrate :f32,
    pub status : PlayerStatus,
    pub won_game : GameStatus,
    pub player : T,
}