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

const MENU: &str = r#"Crossterm interactive test

Controls:

- 'q' - quit interactive test (or return to this menu)
- 'h' for heads 
- 't' for tails
- '1' for batting 
- '2' for bowling 
- '0-6' for runs
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
    
            w.flush()?;

            let toss_event = read()?;
            
            if toss_event == Event::Key(KeyCode::Char('h').into()) {
                queue!(w, style::Print("You choose heads"), cursor::MoveToNextLine(1))?;
                if CoinToss::guess("Heads".parse()?).is_correct() {
                    queue!(w, style::Print("You won the toss - Batting or Bowling"), cursor::MoveToNextLine(1))?;
                    w.flush()?;

                    let toss_decision = read()?;

                    if toss_decision == Event::Key(KeyCode::Char('1').into()) {
                        CricketGame::new(TossWonBy::Human, PlayerStatus::Batting);
                    }
                    else if toss_decision == Event::Key(KeyCode::Char('2').into()) {
                        CricketGame::new(TossWonBy::Human, PlayerStatus::Bowling);
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

            if toss_event == Event::Key(KeyCode::Char('t').into()) {
                queue!(w, style::Print("You choose tails"), cursor::MoveToNextLine(1))?;
                w.flush()?;
                if CoinToss::guess("Heads".parse()?).is_correct() {
                    queue!(w, style::Print("You won the toss - Batting or Bowling"), cursor::MoveToNextLine(1))?;
                    w.flush()?;

                    let toss_decision = read()?;

                    if toss_decision == Event::Key(KeyCode::Char('1').into()) {
                        CricketGame::new(TossWonBy::Human, PlayerStatus::Batting);
                    }
                    else if toss_decision == Event::Key(KeyCode::Char('2').into()) {
                        CricketGame::new(TossWonBy::Human, PlayerStatus::Bowling);
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

            if toss_event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
            else {
                continue;
            }

        }

        queue!(w, style::Print("Hope you enjoyed the game, Quitting now !!!"), cursor::MoveToNextLine(1))?;

        w.flush()?;
                    
        execute!(
                w,
                style::ResetColor,
                cursor::Show,
                terminal::LeaveAlternateScreen
        )?;
        
        terminal::disable_raw_mode()
     }
