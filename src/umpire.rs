extern crate crossterm;

use std::io::{Write};
use crate::toss::coin_toss::*;
use crate::game::*;
use crate::utils::utils::*;
use rand::Rng;

use crossterm::{
    cursor,
    execute, queue, style,
    terminal::{self, ClearType},
    event::{read, Event, KeyCode},
    Result,
};

const MENU: &str = r#"fun with rusty cricket 

Instructions:

- 'q' - quit interactive test (or return to this menu)
- 'h' for heads 
- 't' for tails
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

macro_rules! start_game {
    ($a:expr,$b:expr, $c:ident) => {
            match CricketGame::new($a, $b).start($c) {
                    Ok(()) => {
                            break;
                    }
                    Err(e) => {
                            panic!(e);
                    }
            }
    };
}

macro_rules! toss {
    ($a:expr, $w:ident) => {
            queue!($w, style::Print("You choose "))?;
            queue!($w, style::Print($a), cursor::MoveToNextLine(1))?;
            $w.flush()?;
            if CoinToss::guess($a.parse()?).is_correct() {
                    for line in TOSSCHOICE.split('\n') {
                            queue!($w, style::Print(line), cursor::MoveToNextLine(1))?;
                    }
                    $w.flush()?;

                    let toss_decision = read()?;

                    if toss_decision == Event::Key(KeyCode::Char('1').into()) {
                            start_game!(TossWonBy::Human, PlayerStatus::Batting, $w);
                    } else if toss_decision == Event::Key(KeyCode::Char('2').into()) {
                            start_game!(TossWonBy::Human, PlayerStatus::Bowling, $w);
                    } else if toss_decision == Event::Key(KeyCode::Char('q').into()) {
                            break;
                    } else {
                            queue!(
                                    $w,
                                    style::Print("Invaid input, Choose h or t"),
                                    cursor::MoveToNextLine(1)
                            )?;
                            $w.flush()?;
                            continue;
                    }
            } else {
                    queue!($w, style::Print("You lost toss"), cursor::MoveToNextLine(1))?;
                    $w.flush()?;
                    match rand::thread_rng().gen::<bool>() {
                            true => {
                                    start_game!(TossWonBy::Genie, PlayerStatus::Batting, $w);
                            }
                            _ => {
                                    start_game!(TossWonBy::Genie, PlayerStatus::Bowling, $w);
                            }
                    }
            }
    };
}

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
    
            queue!(w, style::Print("Choose your Toss h (or) t"), cursor::MoveToNextLine(1))?;
            w.flush()?;

            let toss_event = read()?;
            
            if toss_event == Event::Key(KeyCode::Char('h').into()) {
                toss!("Heads", w);
            }
            else if toss_event == Event::Key(KeyCode::Char('t').into()) {
                toss!("Tails", w);
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
