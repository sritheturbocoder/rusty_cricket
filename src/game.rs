use crate::players::{human, genie,utils};

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

        match self.human_player.status{
            utils::PlayerStatus::Batting => println!("Game starts..Start batting now"),
            utils::PlayerStatus::Bowling => println!("Game starts..Start bowling now")
        }
    }
}

