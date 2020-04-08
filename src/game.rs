extern crate ansi_term;
use crate::players::*;
use crate::game;
use std::time::Duration;
use std::thread::sleep;
use std::io::{Write};
use rand::Rng;

use crossterm::{
    cursor,
    execute, queue, style,
    terminal::{self, ClearType},
    event::{read, Event, KeyCode},
    Result,
};

#[derive(Clone, Copy)]
pub enum TossWonBy {
    Human,
    Genie
}

pub struct CricketGame{
    pub toss_won_by : TossWonBy,
    pub toss_decision : utils::PlayerStatus,
    pub human_player : human::Human,
    pub genie_player : genie::Genie,
}

impl CricketGame{

    pub fn new(toss_won_by : TossWonBy, toss_decision : utils::PlayerStatus) ->CricketGame {

        match toss_won_by {
            game::TossWonBy::Human => {
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
            game::TossWonBy::Genie => {
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
        let four = "🎯";
        let six = "Maximum !!! 🎳";
        let hundred = "💯";
        let wicket = "OUT !!! ☝";
        let mut human_prediction : u8;
        let mut genie_prediction : u8;

        loop {

            match self.human_player.status {

                utils::PlayerStatus::Bowling => {
                    queue!(w, style::Print("Guess Batsman score (0-6) "), cursor::MoveToNextLine(1))?;
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
                    genie_prediction = rand::thread_rng().gen_range(0,6);
                }
            }

            CricketGame::bowl(&self,w);
            break; // to bat

        }

        
        Ok(())
    }

    fn bowl<W>(&self, w : &mut W) -> Result<()> 
    where
    W: Write,{
        let ball = "🏮";
        let mut duration_remaining = rand::thread_rng().gen_range(3,6);
        while duration_remaining > 0 {
            game::CricketGame::countdown_one_second_from(w, &duration_remaining, true).ok();
            duration_remaining -= 1;
        }
        w.flush()?;
        Ok(())
    }

    fn bat<W>(&self, w: &mut W) -> Result<()>
    where W: Write{
        let bat = "🏏"; 
        queue!(w, style::Print(bat), cursor::MoveToNextLine(1))?;   
        w.flush()?;
        let mut duration_remaining : u8 = 2;
        while duration_remaining > 0 {
            game::CricketGame::countdown_one_second_from(w, &duration_remaining, false).ok();
            duration_remaining -= 1;
        }

        Ok(())
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