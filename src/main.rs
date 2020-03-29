mod errors;
mod toss;
mod players;
mod game;

use std::io::{stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;

use crate::toss::coin_toss::{CoinError, CoinToss};
use players::genie::{Genie};
use players::human::{Human};
use players::utils::{PlayerStatus};
use game::CricketGame;

fn main() -> Result<(), CoinError> {
        let stdin = stdin();
        println!("fun with rusty cricket");
        println!("ESC or q to exit. Let's toss ******** Heads or Tails ********");

        let mut genie_player : Option<Genie> = None;
        let mut human_player : Option<Human> = None;

        for c in stdin.keys() {
                match c.unwrap() {
                        Key::Char('q') => break,
                        Key::Esc => break,
                        Key::Char('h') => {
                                if CoinToss::guess("Heads".parse()?).is_correct() {
                                        println!("Lucky !!, You won the toss");
                                        
                                        genie_player = Some(Genie {
                                                wontoss : false,
                                                status : PlayerStatus::Bowling,
                                                overs : 0,
                                                runs : 0,
                                                wickets : 0
                                        });

                                        human_player = Some(Human {
                                                wontoss : true,
                                                status : PlayerStatus::Batting,
                                                overs : 0,
                                                runs : 0,
                                                wickets : 0
                                        });
                                } else {
                                        println!("Haha!!, I won the toss");
                                        
                                        genie_player = Some(Genie {
                                                wontoss : true,
                                                status : PlayerStatus::Batting,
                                                overs : 0,
                                                runs : 0,
                                                wickets : 0
                                        });

                                        human_player = Some(Human {
                                                wontoss : false,
                                                status : PlayerStatus::Bowling,
                                                overs : 0,
                                                runs : 0,
                                                wickets : 0
                                        });
                                }
                                break;
                        }
                        Key::Char('t') => {
                                if CoinToss::guess("Tails".parse()?).is_correct() {
                                        println!("Lucky !!, You won the toss");

                                        genie_player = Some(Genie {
                                                wontoss : false,
                                                status : PlayerStatus::Bowling,
                                                overs : 0,
                                                runs : 0,
                                                wickets : 0
                                        });

                                        human_player = Some(Human {
                                                wontoss : true,
                                                status : PlayerStatus::Batting,
                                                overs : 0,
                                                runs : 0,
                                                wickets : 0
                                        });

                                } else {
                                        println!("Haha!!, I won the toss");

                                        genie_player = Some(Genie {
                                                wontoss : true,
                                                status : PlayerStatus::Batting,
                                                overs : 0,
                                                runs : 0,
                                                wickets : 0
                                        });

                                        human_player = Some(Human {
                                                wontoss : false,
                                                status : PlayerStatus::Bowling,
                                                overs : 0,
                                                runs : 0,
                                                wickets : 0
                                        });

                                }
                                break;
                        }
                        _ => {
                                println!("Invalid choice, Try Again Head or Tail ?");
                                continue;
                        }
                }
        }

        if let Some(human_player) = human_player
        {
                if let Some(genie_player) = genie_player
                {
                        let game : CricketGame = CricketGame {
                                innings : 1,
                                human_player : human_player,
                                genie_player : genie_player,
                                max_overs : 10
                        };
                
                        game.start_game();

                }
        }

        Ok(())
}
