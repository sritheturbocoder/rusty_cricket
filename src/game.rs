use crate::players::{human, genie};

pub struct CricketGame{
    pub innings : u8,
    pub human_player : human::Human,
    pub genie_player : genie::Genie,
    pub max_overs : u8,
}

impl CricketGame{
    pub fn start_game(self){
        println!("Human status {} overs {} runs {} wickets ", 
        self.human_player.overs, 
        self.human_player.runs, 
        self.human_player.wickets);

        println!("\n");

        println!("Genie status {} overs {} runs {} wickets ", 
        self.genie_player.overs, 
        self.genie_player.runs, 
        self.genie_player.wickets);

        println!("\n");
    }
}

