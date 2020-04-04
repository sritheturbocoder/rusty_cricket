extern crate ansi_term;
use ansi_term::Colour::*;
use crate::players::*;
use crate::game;
use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum TossWonBy {
    Human,
    Genie
}

pub struct CricketGame{
    pub innings : u8,
    pub max_overs : u8,
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
                        innings : 0,
                        max_overs : 10,
                        toss_won_by : toss_won_by,
                        toss_decision : toss_decision,
                        human_player : human::Human {
                            overs : 10,
                            runs : 0,
                            status : utils::PlayerStatus::Batting,
                            wickets : 11,
                        },
                        genie_player : genie::Genie{
                            overs : 10,
                            runs : 0,
                            status : utils::PlayerStatus::Bowling,
                            wickets : 11,
                        }
                    },
                    utils::PlayerStatus::Bowling => CricketGame{
                        innings : 0,
                        max_overs : 10,
                        toss_won_by : toss_won_by,
                        toss_decision : toss_decision,
                        human_player : human::Human {
                            overs : 10,
                            runs : 0,
                            status : utils::PlayerStatus::Bowling,
                            wickets : 11,
                        },
                        genie_player : genie::Genie{
                            overs : 10,
                            runs : 0,
                            status : utils::PlayerStatus::Batting,
                            wickets : 11,
                        }
                    },
                }
            },
            game::TossWonBy::Genie => {
                match toss_decision {
                    utils::PlayerStatus::Batting => CricketGame{
                        innings : 0,
                        max_overs : 10,
                        toss_won_by : toss_won_by,
                        toss_decision : toss_decision,
                        human_player : human::Human {
                            overs : 10,
                            runs : 0,
                            status : utils::PlayerStatus::Bowling,
                            wickets : 11,
                        },
                        genie_player : genie::Genie{
                            overs : 10,
                            runs : 0,
                            status : utils::PlayerStatus::Batting,
                            wickets : 11,
                        }
                    },
                    utils::PlayerStatus::Bowling => CricketGame{
                        innings : 0,
                        max_overs : 10,
                        toss_won_by : toss_won_by,
                        toss_decision : toss_decision,
                        human_player : human::Human {
                            overs : 10,
                            runs : 0,
                            status : utils::PlayerStatus::Batting,
                            wickets : 11,
                        },
                        genie_player : genie::Genie{
                            overs : 10,
                            runs : 0,
                            status : utils::PlayerStatus::Bowling,
                            wickets : 11,
                        }
                    },
                }
            }
        }
    }

    pub fn start(&self) {

        let bat = "🏏"; 
        let ball = "🏮";
        let four = "🎯";
        let six = "Maximum !!! 🎳";
        let hundred = "💯";

        let mut duration_remaining = rand::thread_rng().gen_range(2,10);
        while duration_remaining > 0 {
            self.countdown_one_second_from(&duration_remaining, true);
            duration_remaining -= 1;
        }

        print!("{}", Red.bold().paint(bat));
        

        duration_remaining = 1;
        while duration_remaining > 0 {
            self.countdown_one_second_from(&duration_remaining, false);
            duration_remaining -= 1;
        }

        println!("\n");

        println!("Next ball coming through !!!");
    }

    fn countdown_one_second_from(&self, start_second: &usize, showball : bool) {
        let quarter_of_second = Duration::from_millis(250);
        let ball = "🏮";
        for _ in 0..3 {
            if showball { print!("{}", Red.bold().paint(ball)) }
            io::stdout().flush().unwrap();
            sleep(quarter_of_second);
        }
    }
}