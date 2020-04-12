extern crate crossterm;

use crate::toss::coin_toss::{CoinToss};
use crate::players::utils::*;
use crate::game::{CricketGame, TossWonBy};
use std::io::{Write};

use crossterm::{
    cursor,
    execute, queue, style,
    terminal::{self, ClearType},
    event::{read, Event, KeyCode},
    Result,
};

use rand::Rng;

const MENU: &str = r#"fun with rusty cricket 

Instructions:

- 'q' - quit interactive test (or return to this menu)
- '$' for heads 
- '@' for tails
- '1' for batting 
- '2' for bowling 
- '0-6' for runs
"#;

const TOSSCHOICE: &str = r#"You won the toss

Instructions:

- 'q' - quit interactive test (or return to this menu)
- '1' for batting 
- '2' for bowling 
"#;

pub fn toss<W>(w: &mut W) -> Result<()>
    where
    W: Write,
{
        execute!(w, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
    
        loop {
            queue!(
                w,
                style::ResetColor,
                terminal::Clear(ClearType::All),
                cursor::Hide,
                cursor::MoveTo(1, 1)
            )?;
    
            for line in MENU.split('\n') {
                queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
            }
    
            queue!(w, style::Print("Let's Toss $ (or) @"), cursor::MoveToNextLine(1))?;
            w.flush()?;

            let toss_event = read()?;
            
            if toss_event == Event::Key(KeyCode::Char('$').into()) {
                queue!(w, style::Print("You choose heads"), cursor::MoveToNextLine(1))?;
                w.flush()?;
                if CoinToss::guess("Heads".parse()?).is_correct() {
                    
                    for line in TOSSCHOICE.split('\n') {
                        queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
                    }

                    w.flush()?;

                    let toss_decision = read()?;

                    if toss_decision == Event::Key(KeyCode::Char('1').into()) {
                        match CricketGame::new(TossWonBy::Human, PlayerStatus::Batting).start(w){
                            Ok(()) => {
                                break;
                            },
                            Err(e) => {
                                panic!(e);
                            }
                        }
                    }
                    else if toss_decision == Event::Key(KeyCode::Char('2').into()) {
                        match CricketGame::new(TossWonBy::Human, PlayerStatus::Bowling).start(w){
                            Ok(()) => {
                                break;
                            },
                            Err(e) => {
                                panic!(e);
                            }
                        }
                    }
                    else if toss_decision == Event::Key(KeyCode::Char('q').into()) {
                        break;
                    }
                    else {
                        queue!(w, style::Print("Invaid input, Choose h or t"), cursor::MoveToNextLine(1))?;
                        w.flush()?;
                        continue;
                    }
                }
                else {
                    queue!(w, style::Print("You lost toss"), cursor::MoveToNextLine(1))?;
                    w.flush()?;
                    match rand::thread_rng().gen::<bool>() {
                        true => {
                            CricketGame::new(TossWonBy::Genie, PlayerStatus::Batting);
                        },
                        _ => {
                            CricketGame::new(TossWonBy::Genie, PlayerStatus::Bowling);
                        }
                    }
                }
            }

            if toss_event == Event::Key(KeyCode::Char('@').into()) {
                queue!(w, style::Print("You choose tails"), cursor::MoveToNextLine(1))?;
                w.flush()?;
                if CoinToss::guess("Heads".parse()?).is_correct() {
                    
                    for line in TOSSCHOICE.split('\n') {
                        queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
                    }

                    w.flush()?;

                    let toss_decision = read()?;

                    if toss_decision == Event::Key(KeyCode::Char('1').into()) {
                        match CricketGame::new(TossWonBy::Human, PlayerStatus::Batting).start(w) {
                            Ok(()) => {
                                break;
                            },
                            Err(e) => {
                                panic!(e);
                            }
                        }
                    }
                    else if toss_decision == Event::Key(KeyCode::Char('2').into()) {
                        match CricketGame::new(TossWonBy::Human, PlayerStatus::Bowling).start(w) {
                                Ok(()) => {
                                    break;
                                },
                                Err(e) => {
                                    panic!(e);
                                }
                        }
                    }
                    else if toss_decision == Event::Key(KeyCode::Char('q').into()) {
                        break;
                    }
                    else {
                        queue!(w, style::Print("Invaid input, Choose h or t"), cursor::MoveToNextLine(1))?;
                        w.flush()?;
                        continue;
                    }
                }
                else {
                    queue!(w, style::Print("You lost toss"), cursor::MoveToNextLine(1))?;
                    w.flush()?;
                    match rand::thread_rng().gen::<bool>() {
                        true => {
                            match CricketGame::new(TossWonBy::Genie, PlayerStatus::Batting).start(w) {
                                Ok(()) => {
                                    break;
                                },
                                Err(e) => {
                                    panic!(e);
                                }
                            }
                        },
                        _ => {
                            match CricketGame::new(TossWonBy::Genie, PlayerStatus::Bowling).start(w) {
                                Ok(()) => {
                                    break;
                                },
                                Err(e) => {
                                    panic!(e);
                                }
                            }
                        }
                    }
                }
            }

            if toss_event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
            else {
                continue;
            }

        }

        queue!(w, style::Print("Hope you enjoyed the game, Press any key to return to terminal !!!"), cursor::MoveToNextLine(1))?;
        w.flush()?;

        read()?;
        execute!(
                w,
                style::ResetColor,
                cursor::Show,
                terminal::LeaveAlternateScreen
        )?;
        
        terminal::disable_raw_mode()
     }
