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
