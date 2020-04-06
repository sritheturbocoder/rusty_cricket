use crate::game::CricketGame;
use players::utils::PlayerStatus;

/// Base Error code for GAMe (5XXX)
pub const BASE_GAME_ERROR_CODE: i32 = 6000;

pub const BASE_GAME_OVER_CODE: i32 = 6001;

pub struct ScoreBoard {
    pub innings : u8,
    pub max_overs : u8,
    pub balls : u16,
    pub innings : u8
}

pub enum GameError {
    #[fail(display = "Game Over")]
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
        type Err = scoreboard::GameError;
        match self.game.human_player.status {
            PlayerStatus::Batting => {
                if human_score == genie_score cricket_game.human_player.wickets = cricket_game.human_player.wickets + 1;
                if human_score > genie_score cricket_game.human_player.runs += human_score;
            },
            PlayerStatus::Bowling => {
                if genie_score == human_score cricket_game.genie_player.wickets = cricket_game.genie_player.wickets + 1;
                if genie_score > human_score cricket_game.genie_player.runs += genie_score;
            }
        }
        self.balls += 1;
        
        if max_overs * 6 == self.balls && self.innings == 2 { 
            declare_winner(&mut cricket_game);
        }

        declare_winner(&mut cricket_game);
    }

    fn declare_winner(&mut cricket_game : CricketGame) -> Result<Self, Self::Err> {

        if cricket_game.human_player.runs > cricket_game.genie_player.runs {
            cricket_game.human_player.won_game = players::utils::GameStatus::Won;
            cricket_game.genie_player.won_game = players::utils::GameStatus::Loss;
            Err(BASE_GAME_OVER_CODE);
        }

        if cricket_game.genie_player.runs > cricket_game.human_player.runs {
            cricket_game.genie_player.won_game = players::utils::GameStatus::Won;
            cricket_game.human_player.won_game = players::utils::GameStatus::Loss;
            Err(BASE_GAME_OVER_CODE);
        }

        if (cricket_game.genie_player.runs == cricket_game.human_player.runs) {
            cricket_game.genie_player.won_game = players::utils::GameStatus::Draw;
            cricket_game.human_player.won_game = players::utils::GameStatus::Draw;
            Err(BASE_GAME_OVER_CODE);
        }

        Ok(());
    }
}