mod errors;
mod toss;
mod players;
mod game;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

use crate::toss::coin_toss::{CoinError, CoinToss};
use players::genie::{Genie};
use players::human::{Human};
use players::utils::{PlayerStatus};
use game::CricketGame;

fn main() -> Result<(), CoinError> {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        
        write!(
                stdout,
                "{}{}{}{bold}{red}fun {blue}with {green}rusty {yellow}cricket",
                clear::All,
                cursor::Goto(1,1),
                cursor::Hide,
                bold = style::Bold,
                red = color::Fg(color::Red),
                blue = color::Fg(color::Blue),
                green = color::Fg(color::Green),
                yellow = color::Fg(color::LightYellow)
        )
        .unwrap();

        writeln!(
                stdout,
                "{}",
                cursor::Goto(1,2)
        )
        .unwrap();

        writeln!(
                stdout,
                "ESC or q to exit. Let's toss ******** Heads or Tails ********"
        )
        .unwrap();

        stdout.flush().unwrap();

        let mut lineno : u16 = 2;
        let mut genie_player : Option<Genie> = None;
        let mut human_player : Option<Human> = None;

        for c in stdin.keys() {
                lineno = lineno + 1;
                write!(
                        stdout,
                        "{}{}",
                        cursor::Goto(1, lineno),
                        clear::CurrentLine
                )
                .unwrap();
                lineno = lineno + 1;
                match c.unwrap() {
                        Key::Char('q') => break,
                        Key::Esc => break,
                        Key::Char('h') => {
                                if CoinToss::guess("Heads".parse()?).is_correct() {
                                        write!(
                                                stdout,
                                                "{}{}{}",
                                                cursor::Goto(1, lineno),
                                                clear::CurrentLine,
                                                "Fate loves you"
                                        ).unwrap();
                                        
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
                                        write!(
                                                stdout,
                                                "{}{}{}",
                                                cursor::Goto(1, lineno),
                                                clear::CurrentLine,
                                                "Not so lucky"
                                        ).unwrap();
                                        
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
                                        write!(
                                                stdout,
                                                "{}{}{}",
                                                cursor::Goto(1, lineno),
                                                clear::CurrentLine,
                                                "Fate loves you"
                                        ).unwrap();

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
                                        write!(
                                                stdout,
                                                "{}{}{}",
                                                cursor::Goto(1, lineno),
                                                clear::CurrentLine,
                                                "Not so lucky"
                                        ).unwrap();

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
                                write!(
                                        stdout,
                                        "{}{}{}",
                                        cursor::Goto(1, lineno),
                                        clear::CurrentLine,
                                        "Invalid choice, Try Again Head or Tail ?"
                                ).unwrap();
                                lineno = lineno + 1;
                                continue;
                        }
                }
        }

        stdout.flush().unwrap();
        // Show the cursor again before we exit.
        write!(stdout, "{}", termion::cursor::Show).unwrap();
        
        println!("{}{}{}{}",
                clear::All,
                cursor::Goto(1,1),
                cursor::Show,
                color::Fg(color::Reset)
        );

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
