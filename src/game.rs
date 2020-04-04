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
}