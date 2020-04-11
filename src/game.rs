use crate::players::utils::{GameStatus, PlayerStatus};
use crate::players::{genie, human};
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    queue, style,
    terminal::{self, ClearType},
    Result,
};
use rand::Rng;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Copy)]
pub enum TossWonBy {
    Human,
    Genie,
}

pub struct ScoreBoard {
    pub innings: u8,
    pub max_overs: u16,
}

pub struct CricketGame {
    pub toss_won_by: TossWonBy,
    pub toss_decision: PlayerStatus,
    pub human_player: human::Human,
    pub genie_player: genie::Genie,
    pub score_board: ScoreBoard,
}

impl CricketGame {
    pub fn new(toss_won_by: TossWonBy, toss_decision: PlayerStatus) -> CricketGame {
        match toss_won_by {
            TossWonBy::Human => {
                match toss_decision {
                    PlayerStatus::Batting => {
                        return CricketGame {
                            toss_won_by: toss_won_by,
                            toss_decision: toss_decision,
                            human_player: human::Human {
                                runs: 0,
                                status: PlayerStatus::Batting,
                                wickets: 0,
                                bowled_balls: 0,
                                required_runrate: 0.0,
                                current_runrate: 0.0,
                                won_game: GameStatus::InProgress,
                            },
                            genie_player: genie::Genie {
                                runs: 0,
                                status: PlayerStatus::Bowling,
                                wickets: 0,
                                bowled_balls: 0,
                                required_runrate: 0.0,
                                current_runrate: 0.0,
                                won_game: GameStatus::InProgress,
                            },
                            score_board: ScoreBoard {
                                innings: 1,
                                max_overs: 10,
                            },
                        }
                    }
                    PlayerStatus::Bowling => {
                        return CricketGame {
                            toss_won_by: toss_won_by,
                            toss_decision: toss_decision,
                            human_player: human::Human {
                                runs: 0,
                                status: PlayerStatus::Bowling,
                                wickets: 0,
                                bowled_balls: 0,
                                required_runrate: 0.0,
                                current_runrate: 0.0,
                                won_game: GameStatus::InProgress,
                            },
                            genie_player: genie::Genie {
                                runs: 0,
                                status: PlayerStatus::Batting,
                                wickets: 0,
                                bowled_balls: 0,
                                required_runrate: 0.0,
                                current_runrate: 0.0,
                                won_game: GameStatus::InProgress,
                            },
                            score_board: ScoreBoard {
                                innings: 1,
                                max_overs: 10,
                            },
                        }
                    }
                };
            }

            TossWonBy::Genie => {
                match toss_decision {
                    PlayerStatus::Batting => {
                        return CricketGame {
                            toss_won_by: toss_won_by,
                            toss_decision: toss_decision,
                            human_player: human::Human {
                                runs: 0,
                                status: PlayerStatus::Bowling,
                                wickets: 0,
                                bowled_balls: 0,
                                required_runrate: 0.0,
                                current_runrate: 0.0,
                                won_game: GameStatus::InProgress,
                            },
                            genie_player: genie::Genie {
                                runs: 0,
                                status: PlayerStatus::Batting,
                                wickets: 0,
                                bowled_balls: 0,
                                required_runrate: 0.0,
                                current_runrate: 0.0,
                                won_game: GameStatus::InProgress,
                            },
                            score_board: ScoreBoard {
                                innings: 1,
                                max_overs: 10,
                            },
                        }
                    }
                    PlayerStatus::Bowling => {
                        return CricketGame {
                            toss_won_by: toss_won_by,
                            toss_decision: toss_decision,
                            human_player: human::Human {
                                runs: 0,
                                status: PlayerStatus::Batting,
                                wickets: 0,
                                bowled_balls: 0,
                                required_runrate: 0.0,
                                current_runrate: 0.0,
                                won_game: GameStatus::InProgress,
                            },
                            genie_player: genie::Genie {
                                runs: 0,
                                status: PlayerStatus::Bowling,
                                wickets: 0,
                                bowled_balls: 0,
                                required_runrate: 0.0,
                                current_runrate: 0.0,
                                won_game: GameStatus::InProgress,
                            },
                            score_board: ScoreBoard {
                                innings: 1,
                                max_overs: 10,
                            },
                        }
                    }
                };
            }
        }
    }

    pub fn start<W>(&mut self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        let bat = "🏏";
        let mut human_prediction: u16 = 0;
        let mut genie_prediction: u16 = 0;

        loop {
            queue!(w, terminal::Clear(ClearType::All))?;
            queue!(w, cursor::MoveTo(1, 1))?;
            w.flush().ok();
            match self.human_player.status {
                PlayerStatus::Bowling => {
                    queue!(
                        w,
                        style::Print("Guess Batsman score (0-6) "),
                        cursor::MoveToNextLine(1)
                    )?;
                    w.flush().ok();
                    let runs_predict = read()?;
                    if runs_predict == Event::Key(KeyCode::Char('0').into()) {
                        human_prediction = 0;
                    } else if runs_predict == Event::Key(KeyCode::Char('1').into()) {
                        human_prediction = 1;
                    } else if runs_predict == Event::Key(KeyCode::Char('2').into()) {
                        human_prediction = 2;
                    } else if runs_predict == Event::Key(KeyCode::Char('3').into()) {
                        human_prediction = 3;
                    } else if runs_predict == Event::Key(KeyCode::Char('4').into()) {
                        human_prediction = 4;
                    } else if runs_predict == Event::Key(KeyCode::Char('5').into()) {
                        human_prediction = 5;
                    } else if runs_predict == Event::Key(KeyCode::Char('6').into()) {
                        human_prediction = 6;
                    }
                }

                PlayerStatus::Batting => {
                    queue!(
                        w,
                        style::Print("Bowling now... "),
                        cursor::MoveToNextLine(1)
                    )?;
                    w.flush().ok();
                    genie_prediction = rand::thread_rng().gen_range(0, 6);
                }
            }

            match self.bowl(w) {
                Ok(()) => {
                    queue!(w, style::Print(bat), cursor::MoveToNextLine(1))?;
                    w.flush()?;
                }
                Err(e) => panic!(e),
            }

            let runs_scored: u16;
            match CricketGame::bat(self.human_player.status).unwrap() {
                0 => {
                    runs_scored = 0;
                }
                1 => {
                    runs_scored = 1;
                }
                2 => {
                    runs_scored = 2;
                }
                3 => {
                    runs_scored = 3;
                }
                4 => {
                    runs_scored = 4;
                }
                5 => {
                    runs_scored = 5;
                }
                6 => {
                    runs_scored = 6;
                }
                _ => {
                    runs_scored = 0;
                }
            }

            match self.human_player.status {
                PlayerStatus::Batting => {
                    match ScoreBoard::score(runs_scored, genie_prediction, self) {
                        Ok(GameStatus::InProgress) => {
                            continue;
                        }
                        Ok(GameStatus::NextOver) => {
                            ScoreBoard::print_score(w, self).ok();
                            queue!(
                                w,
                                style::Print("Press any key for next over..."),
                                cursor::MoveToNextLine(1)
                            )?;
                            w.flush().ok();
                            read()?;
                            continue;
                        }
                        Ok(GameStatus::NextInnings) => {
                            ScoreBoard::print_score(w, self).ok();
                            queue!(
                                w,
                                style::Print(
                                    "Get ready to Bowl. Press any key for next innings..."
                                ),
                                cursor::MoveToNextLine(1)
                            )?;
                            w.flush().ok();
                            read()?;
                            continue;
                        }
                        Ok(GameStatus::GameOver)
                        | Ok(GameStatus::Won)
                        | Ok(GameStatus::Loss)
                        | Ok(GameStatus::Draw)
                        | Err(_) => {
                            ScoreBoard::print_score(w, self).ok();
                            queue!(
                                w,
                                style::Print("Game over :)...Press any key..."),
                                cursor::MoveToNextLine(1)
                            )?;
                            w.flush().ok();
                            read()?;
                            break;
                        }
                    }
                }
                PlayerStatus::Bowling => {
                    match ScoreBoard::score(human_prediction, runs_scored, self) {
                        Ok(GameStatus::InProgress) => {
                            ScoreBoard::print_score(w, self).ok();
                            continue;
                        }
                        Ok(GameStatus::NextOver) => {
                            ScoreBoard::print_score(w, self).ok();
                            queue!(
                                w,
                                style::Print("Press any key for next over..."),
                                cursor::MoveToNextLine(1)
                            )?;
                            w.flush().ok();
                            read()?;
                            continue;
                        }
                        Ok(GameStatus::NextInnings) => {
                            ScoreBoard::print_score(w, self).ok();
                            queue!(
                                w,
                                style::Print("Get ready to Bat. Press any key for next innings..."),
                                cursor::MoveToNextLine(1)
                            )?;
                            w.flush().ok();
                            read()?;
                            continue;
                        }
                        Ok(GameStatus::GameOver)
                        | Ok(GameStatus::Won)
                        | Ok(GameStatus::Loss)
                        | Ok(GameStatus::Draw)
                        | Err(_) => {
                            ScoreBoard::print_score(w, self).ok();
                            queue!(
                                w,
                                style::Print("Game over :)...Press any key..."),
                                cursor::MoveToNextLine(1)
                            )?;
                            w.flush().ok();
                            read()?;
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn bowl<W>(&self, w: &mut W) -> Result<()>
    where
        W: Write,
    {
        let mut duration_remaining = rand::thread_rng().gen_range(3, 6);
        while duration_remaining > 0 {
            CricketGame::countdown_one_second_from(w, &duration_remaining, true).ok();
            duration_remaining -= 1;
        }
        w.flush()?;
        Ok(())
    }

    pub fn bat(human_status: PlayerStatus) -> Result<u16> {
        match human_status {
            PlayerStatus::Batting => {
                let runs = read()?;
                if runs == Event::Key(KeyCode::Char('0').into()) {
                    return Ok(0);
                } else if runs == Event::Key(KeyCode::Char('1').into()) {
                    return Ok(1);
                } else if runs == Event::Key(KeyCode::Char('2').into()) {
                    return Ok(2);
                } else if runs == Event::Key(KeyCode::Char('3').into()) {
                    return Ok(3);
                } else if runs == Event::Key(KeyCode::Char('4').into()) {
                    return Ok(4);
                } else if runs == Event::Key(KeyCode::Char('5').into()) {
                    return Ok(5);
                } else if runs == Event::Key(KeyCode::Char('6').into()) {
                    return Ok(6);
                } else {
                    return Err(crossterm::ErrorKind::__Nonexhaustive);
                }
            }
            PlayerStatus::Bowling => {
                return Ok(rand::thread_rng().gen_range(0, 6));
            }
        }
    }

    pub fn countdown_one_second_from<W>(w: &mut W, start_second: &u8, showball: bool) -> Result<()>
    where
        W: Write,
    {
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

impl ScoreBoard {
    pub fn score(
        human_score: u16,
        genie_score: u16,
        cricket_game: &mut CricketGame,
    ) -> Result<GameStatus> {
        match cricket_game.human_player.status {
            PlayerStatus::Batting => {
                cricket_game.genie_player.bowled_balls += 1;
                if human_score == genie_score {
                    cricket_game.human_player.wickets = cricket_game.human_player.wickets + 1;
                }
                if human_score > genie_score {
                    cricket_game.human_player.runs += human_score;
                }
                if (cricket_game.score_board.max_overs * 6
                    == cricket_game.genie_player.bowled_balls
                    || cricket_game.human_player.wickets == 11)
                    && cricket_game.score_board.innings == 1
                {
                    cricket_game.score_board.innings = 2;
                    cricket_game.human_player.status = PlayerStatus::Bowling;
                    cricket_game.genie_player.status = PlayerStatus::Batting;
                    return Ok(GameStatus::NextInnings);
                }
            }
            PlayerStatus::Bowling => {
                cricket_game.human_player.bowled_balls += 1;
                if genie_score == human_score {
                    cricket_game.genie_player.wickets = cricket_game.genie_player.wickets + 1;
                }
                if genie_score > human_score {
                    cricket_game.genie_player.runs += genie_score;
                }
                if (cricket_game.score_board.max_overs * 6
                    == cricket_game.human_player.bowled_balls
                    || cricket_game.genie_player.wickets == 11)
                    && cricket_game.score_board.innings == 1
                {
                    cricket_game.score_board.innings = 2;
                    cricket_game.human_player.status = PlayerStatus::Batting;
                    cricket_game.genie_player.status = PlayerStatus::Bowling;
                    return Ok(GameStatus::NextInnings);
                }
            }
        }

        if cricket_game.human_player.status == PlayerStatus::Batting
            && cricket_game.score_board.innings == 2
        {
            let required_runs: f32 =
                cricket_game.genie_player.runs as f32 - cricket_game.human_player.runs as f32 ;
            let remaining_overs: f32 = (cricket_game.score_board.max_overs
                - (cricket_game.genie_player.bowled_balls / 6))
                as f32;
            cricket_game.human_player.required_runrate = required_runs / remaining_overs;

            let current_runs: f32 = cricket_game.human_player.runs as f32;
            let overs_bowled: f32 = (cricket_game.genie_player.bowled_balls / 6) as f32;
            cricket_game.human_player.current_runrate = current_runs / overs_bowled;
        }

        if cricket_game.genie_player.status == PlayerStatus::Batting
            && cricket_game.score_board.innings == 2
        {
            let required_runs: f32 =
                cricket_game.human_player.runs as f32 - cricket_game.genie_player.runs as f32;
            let remaining_overs: f32 = (cricket_game.score_board.max_overs
                - (cricket_game.human_player.bowled_balls / 6))
                as f32;
            cricket_game.genie_player.required_runrate = required_runs / remaining_overs;

            let current_runs: f32 = cricket_game.genie_player.runs as f32;
            let overs_bowled: f32 = (cricket_game.human_player.bowled_balls / 6) as f32;
            cricket_game.genie_player.current_runrate = current_runs / overs_bowled;
        }

        ScoreBoard::check_game_status(cricket_game)
    }

    fn check_game_status(cricket_game: &mut CricketGame) -> Result<GameStatus> {
        if cricket_game.score_board.innings == 2 {
            if cricket_game.human_player.runs > cricket_game.genie_player.runs
                && cricket_game.human_player.status == PlayerStatus::Batting
            {
                cricket_game.human_player.won_game = GameStatus::Won;
                cricket_game.genie_player.won_game = GameStatus::Loss;
                return Ok(GameStatus::GameOver);
            }

            if cricket_game.genie_player.runs > cricket_game.human_player.runs
                && cricket_game.genie_player.status == PlayerStatus::Batting
            {
                cricket_game.genie_player.won_game = GameStatus::Won;
                cricket_game.human_player.won_game = GameStatus::Loss;
                return Ok(GameStatus::GameOver);
            }

            if cricket_game.genie_player.runs == cricket_game.human_player.runs
                && cricket_game.human_player.bowled_balls == cricket_game.genie_player.bowled_balls
            {
                cricket_game.genie_player.won_game = GameStatus::Draw;
                cricket_game.human_player.won_game = GameStatus::Draw;
                return Ok(GameStatus::Draw);
            }

            if cricket_game.human_player.bowled_balls == cricket_game.genie_player.bowled_balls {
                if cricket_game.human_player.runs > cricket_game.genie_player.runs {
                    cricket_game.human_player.won_game = GameStatus::Won;
                    cricket_game.genie_player.won_game = GameStatus::Loss;
                    return Ok(GameStatus::GameOver);
                }

                if cricket_game.genie_player.runs > cricket_game.human_player.runs {
                    cricket_game.genie_player.won_game = GameStatus::Won;
                    cricket_game.human_player.won_game = GameStatus::Loss;
                    return Ok(GameStatus::GameOver);
                }
            }
        }

        if cricket_game.human_player.status == PlayerStatus::Bowling {
            if (cricket_game.human_player.bowled_balls % 6) as f32 == 0.0 {
                return Ok(GameStatus::NextOver);
            }
        }

        if cricket_game.genie_player.status == PlayerStatus::Bowling {
            if (cricket_game.genie_player.bowled_balls % 6) as f32 == 0.0 {
                return Ok(GameStatus::NextOver);
            }
        }

        Ok(GameStatus::InProgress)
    }

    pub fn print_score<W>(w: &mut W, cric_game: &mut CricketGame) -> Result<()>
    where
        W: Write,
    {
        queue!(w, terminal::Clear(ClearType::All), cursor::MoveTo(1, 1))?;
        queue!(
            w,
            style::Print("**************************************************"),
            cursor::MoveToNextLine(1)
        )?;
        queue!(
            w,
            style::Print(cric_game.score_board.innings),
            style::Print(" Innings "),
            cursor::MoveToNextLine(1)
        )?;
        w.flush().ok();

        match cric_game.human_player.status {
            PlayerStatus::Batting => {
                queue!(
                    w,
                    style::Print("You are Batting and score is "),
                    style::Print(cric_game.human_player.runs),
                    style::Print(" / "),
                    style::Print(cric_game.human_player.wickets),
                    style::Print(" wickets "),
                    cursor::MoveToNextLine(1)
                )?;

                queue!(
                    w,
                    style::Print("I am bowling "),
                    style::Print(cric_game.genie_player.bowled_balls / 6),
                    style::Print(" over(s) bowled "),
                    cursor::MoveToNextLine(1)
                )?;
            }
            PlayerStatus::Bowling => {
                queue!(
                    w,
                    style::Print("You are bowling "),
                    style::Print(cric_game.human_player.bowled_balls / 6),
                    style::Print(" over(s) bowled "),
                    cursor::MoveToNextLine(1)
                )?;
                queue!(
                    w,
                    style::Print("I Bat and score is "),
                    style::Print(cric_game.genie_player.runs),
                    style::Print(" / "),
                    style::Print(cric_game.genie_player.wickets),
                    style::Print(" wickets "),
                    cursor::MoveToNextLine(1)
                )?;
            }
        }

        if cric_game.score_board.innings == 2 {
            if cric_game.human_player.status == PlayerStatus::Batting {
                queue!(
                    w,
                    style::Print("Runs to win : "),
                    style::Print(cric_game.genie_player.runs - cric_game.human_player.runs),
                    cursor::MoveToNextLine(1),
                )?;
                queue!(
                    w,
                    style::Print("Required run rate : "),
                    style::Print(cric_game.human_player.required_runrate),
                    cursor::MoveToNextLine(1),
                )?;
                queue!(
                    w,
                    style::Print("Current run rate : "),
                    style::Print(cric_game.human_player.current_runrate),
                    cursor::MoveToNextLine(1),
                )?;
            } else {
                queue!(
                    w,
                    style::Print("Runs to win : "),
                    style::Print(cric_game.human_player.runs - cric_game.genie_player.runs),
                    cursor::MoveToNextLine(1),
                )?;
                queue!(
                    w,
                    style::Print("Required run rate : "),
                    style::Print(cric_game.genie_player.required_runrate),
                    cursor::MoveToNextLine(1),
                )?;
                queue!(
                    w,
                    style::Print("Current run rate : "),
                    style::Print(cric_game.genie_player.current_runrate),
                    cursor::MoveToNextLine(1),
                )?;
            }
        }

        queue!(
            w,
            style::Print("**************************************************"),
            cursor::MoveToNextLine(1)
        )?;

        w.flush().ok();
        Ok(())
    }
}
