extern crate ansi_term;

use ansi_term::Colour::*;
use crate::players::{human, genie,utils};
use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};
use rand::Rng;

pub struct CricketGame{
    pub innings : u8,
    pub human_player : human::Human,
    pub genie_player : genie::Genie,
    pub max_overs : u8,
}

impl CricketGame{
    pub fn start_game(self){

        if self.human_player.wontoss { println!("Lucky !!, You won the toss !!!") } else { println!("Haha!!, I won the toss") }

        println!("*** Human *** {} overs {} runs {} wickets {} ", 
        self.human_player.overs, 
        self.human_player.runs, 
        self.human_player.wickets,
        self.human_player.status);

        println!("*** Genie *** {} overs {} runs {} wickets {} ", 
        self.genie_player.overs, 
        self.genie_player.runs, 
        self.genie_player.wickets,
        self.genie_player.status);

        println!("\n");

        let bat = "🏏"; 
        let ball = "🏮";
        let four = "🎯";
        let six = "Maximum !!! 🎳";
        let hundred = "💯";

        match self.human_player.status{
            utils::PlayerStatus::Batting => println!("Game starts..Start batting now"),
            utils::PlayerStatus::Bowling => println!("Game starts..Start bowling now")
        }

        self.start();
    }

    pub fn start(&self) {
        let mut duration_remaining = rand::thread_rng().gen_range(2,10);
        let bat = "🏏"; 
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

        println!("{}", "Next ball coming through !!!");
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

