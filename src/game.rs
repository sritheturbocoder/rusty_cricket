use crate::players::*;
use std::time::Duration;
use std::thread::sleep;
use std::io::{Write};
use rand::Rng;
use std::{
    fmt::{self, Display, Formatter},
    io,
};


use crossterm::{
    cursor,
    execute, queue, style,
    terminal::{self, ClearType},
    event::{read, Event, KeyCode},
    Result,
};

/// Base Error code for GAMe (5XXX)
pub const BASE_GAME_ERROR_CODE: i32 = 6000;

pub const BASE_GAME_OVER_CODE: i32 = 6001;

#[derive(Clone, Copy)]
pub enum TossWonBy {
    Human,
    Genie
}

pub struct ScoreBoard {
    pub innings : u8,
    pub max_overs : u8,
    pub balls : u16,
}

pub struct CricketGame{
    pub toss_won_by : TossWonBy,
    pub toss_decision : utils::PlayerStatus,
    pub human_player : human::Human,
    pub genie_player : genie::Genie,
    pub score_board : ScoreBoard,
}

impl CricketGame{

    pub fn new(toss_won_by : TossWonBy, toss_decision : utils::PlayerStatus) ->CricketGame {

        match toss_won_by {
            TossWonBy::Human => {
                match toss_decision {
                   utils::PlayerStatus::Batting => CricketGame{
                        toss_won_by : toss_won_by,
                        toss_decision : toss_decision,
                        human_player : human::Human {
                            runs : 0,
                            status : utils::PlayerStatus::Batting,
                            wickets : 11,
                            won_game : utils::GameStatus::InProgress
                        },
                        genie_player : genie::Genie{
                            runs : 0,
                            status : utils::PlayerStatus::Bowling,
                            wickets : 11,
                            won_game : utils::GameStatus::InProgress
                        }
                    },
                    utils::PlayerStatus::Bowling => CricketGame{
                        toss_won_by : toss_won_by,
                        toss_decision : toss_decision,
                        human_player : human::Human {
                            runs : 0,
                            status : utils::PlayerStatus::Bowling,
                            wickets : 11,
                            won_game : utils::GameStatus::InProgress
                        },
                        genie_player : genie::Genie{
                            runs : 0,
                            status : utils::PlayerStatus::Batting,
                            wickets : 11,
                            won_game : utils::GameStatus::InProgress
                        }
                    },
                }
            },
            TossWonBy::Genie => {
                match toss_decision {
                    utils::PlayerStatus::Batting => CricketGame{
                        toss_won_by : toss_won_by,
                        toss_decision : toss_decision,
                        human_player : human::Human {
                            runs : 0,
                            status : utils::PlayerStatus::Bowling,
                            wickets : 11,
                            won_game : utils::GameStatus::InProgress
                        },
                        genie_player : genie::Genie{
                            runs : 0,
                            status : utils::PlayerStatus::Batting,
                            wickets : 11,
                            won_game : utils::GameStatus::InProgress
                        }
                    },
                    utils::PlayerStatus::Bowling => CricketGame{
                        toss_won_by : toss_won_by,
                        toss_decision : toss_decision,
                        human_player : human::Human {
                            runs : 0,
                            status : utils::PlayerStatus::Batting,
                            wickets : 11,
                            won_game : utils::GameStatus::InProgress
                        },
                        genie_player : genie::Genie{
                            runs : 0,
                            status : utils::PlayerStatus::Bowling,
                            wickets : 11,
                            won_game : utils::GameStatus::InProgress
                        }
                    },
                }
            }
        }
    }

    pub fn start<W>(&self, w: &mut W) -> Result<()> 
    where
    W: Write,{
        let bat = "🏏"; 
        let four = "🎯";
        let six = "Maximum !!! 🎳";
        let hundred = "💯";
        let wicket = "OUT !!! ☝";
        let mut human_prediction : u8;
        let mut genie_prediction : u8;

        let score_board = ScoreBoard {
            innings : 0,
            max_overs : 10,
            balls : 0,
        };

        loop {

            match self.human_player.status {

                utils::PlayerStatus::Bowling => {
                    queue!(w, style::Print("Guess Batsman score (0-6) "), cursor::MoveToNextLine(1))?;
                    w.flush();
                    let runs_predict = read()?;
                    if runs_predict == Event::Key(KeyCode::Char('0').into()) {
                        human_prediction = 0;
                    }
                    else
                    if runs_predict == Event::Key(KeyCode::Char('1').into()) {
                        human_prediction = 1;
                    }
                    else
                    if runs_predict == Event::Key(KeyCode::Char('2').into()) {
                        human_prediction = 2;
                    }
                    else
                    if runs_predict == Event::Key(KeyCode::Char('3').into()) {
                        human_prediction = 3;
                    }
                    else
                    if runs_predict == Event::Key(KeyCode::Char('4').into()) {
                        human_prediction = 4;
                    }
                    else
                    if runs_predict == Event::Key(KeyCode::Char('5').into()) {
                        human_prediction = 5;
                    }
                    else
                    if runs_predict == Event::Key(KeyCode::Char('6').into()) {
                        human_prediction = 6;
                    }
                },

                utils::PlayerStatus::Batting => {
                    queue!(w, style::Print("Bowling now... "), cursor::MoveToNextLine(1))?;
                    w.flush();
                    genie_prediction = rand::thread_rng().gen_range(0,6);
                }
            }

            match CricketGame::bowl(&self,w) {
                Ok(()) => {
                    queue!(w, style::Print(bat), cursor::MoveToNextLine(1))?;   
                    w.flush()?;
                },
                Err(e) => {
                    panic!(e)
                }
            }
    
            CricketGame::bat(&self);
            break;

        }

        
        Ok(())
    }

    fn bowl<W>(&self, w : &mut W) -> Result<()> 
    where
    W: Write,{
        let ball = "🏮";
        let mut duration_remaining = rand::thread_rng().gen_range(3,6);
        while duration_remaining > 0 {
            CricketGame::countdown_one_second_from(w, &duration_remaining, true).ok();
            duration_remaining -= 1;
        }
        w.flush()?;
        Ok(())
    }

    fn bat(&self) -> Result<Event> 
    {
        match self.human_player.status {
            utils::PlayerStatus::Batting => {
                Ok(read()?)
            },

            utils::PlayerStatus::Bowling => {
                match rand::thread_rng().gen_range(0,6) {
                    0 => Ok(Event::Key(KeyCode::Char('0').into())),
                    1 => Ok(Event::Key(KeyCode::Char('1').into())),
                    2 => Ok(Event::Key(KeyCode::Char('2').into())),
                    3 => Ok(Event::Key(KeyCode::Char('3').into())),
                    4 => Ok(Event::Key(KeyCode::Char('4').into())),
                    5 => Ok(Event::Key(KeyCode::Char('5').into())),
                    6 => Ok(Event::Key(KeyCode::Char('6').into())),
                    _ => Err(crossterm::ErrorKind::__Nonexhaustive),
                }
            }
        }
    }

    pub fn countdown_one_second_from<W>(w: &mut W, start_second: &u8, showball : bool) -> Result<()> 
    where
    W: Write,{
        let quarter_of_second = Duration::from_millis(250);
        let ball = "🏮";
        for _ in 0..*start_second as i64 {
            if showball {
                queue!(w, style::Print(ball))?;
                w.flush()?;
            }
            sleep(quarter_of_second);
        }
        w.flush()?;
        Ok(())
    }
}


#[derive(::failure::Fail)]
pub enum GameError {
    GameOver,
}

impl ErrorCode for GameError {
    fn error_code(&self) -> i32 {
        BASE_GAME_ERROR_CODE
            + match *self {
                GameError::GameOver => 1
            }
    }
}

impl ScoreBoard {
    pub fn score(self, human_score : u16 , genie_score : u16, &mut cricket_game : CricketGame) -> Result<Self, Self::Err> {
        type Err = GameError;
        match self.game.human_player.status {
            utils::PlayerStatus::Batting => {
                if human_score == genie_score {
                    cricket_game.human_player.wickets = cricket_game.human_player.wickets + 1;
                }
                if human_score > genie_score {
                    cricket_game.human_player.runs += human_score;
                }
            },
            utils::PlayerStatus::Bowling => {
                if genie_score == human_score {
                    cricket_game.genie_player.wickets = cricket_game.genie_player.wickets + 1;
                }
                if genie_score > human_score {
                    cricket_game.genie_player.runs += genie_score;
                }
            }
        }
        self.balls += 1;
        
        if self.max_overs * 6 == self.balls && self.innings == 2 { 
            ScoreBoard::declare_winner(&mut cricket_game);
        }

        ScoreBoard::declare_winner(&mut cricket_game);
    }

    fn declare_winner(&mut cricket_game : CricketGame) -> Result<Self, Self::Err> {

        if cricket_game.human_player.runs > cricket_game.genie_player.runs {
            cricket_game.human_player.won_game = utils::GameStatus::Won;
            cricket_game.genie_player.won_game = utils::GameStatus::Loss;
            Err(BASE_GAME_OVER_CODE);
        }

        if cricket_game.genie_player.runs > cricket_game.human_player.runs {
            cricket_game.genie_player.won_game = utils::GameStatus::Won;
            cricket_game.human_player.won_game = utils::GameStatus::Loss;
            Err(BASE_GAME_OVER_CODE);
        }

        if cricket_game.genie_player.runs == cricket_game.human_player.runs {
            cricket_game.genie_player.won_game = utils::GameStatus::Draw;
            cricket_game.human_player.won_game = utils::GameStatus::Draw;
            Err(BASE_GAME_OVER_CODE);
        }

        Ok(());
    }

    pub fn print_score<W>(&self, w : &mut W ,cric_game : &CricketGame) -> Result<()> 
    where
    W: Write,{
        queue!(w, style::Print("Genie is "), style::Print(cric_game.genie_player.status), cursor::MoveToNextLine(1))?;
        w.flush();
    }
}