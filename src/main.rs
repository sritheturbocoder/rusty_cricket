pub mod utils;

use termion::{color, clear, style, cursor};
use std::io;

fn main() {
    println!("{clear} {goto}", 
            clear = clear::All,
            goto = cursor::Goto(1,1)
            );
    println!("{bold}{red}fun {blue}with {green}rusty {yellow}cricket {reset}",
            bold  = style::Bold,
            red   = color::Fg(color::Red),
            blue  = color::Fg(color::Blue),
            green = color::Fg(color::Green),
            yellow = color::Fg(color::LightYellow),
            reset = color::Fg(color::Reset)
            );
    println!("{bold} {lightcyan} Choose Odd / Even ? (O/E)",
            bold = style::Bold,
            lightcyan = color::Fg(color::LightMagenta));

    let toss : String = get_player_input()
            .unwrap_or_else(|e| utils::exit_err(&e, e.raw_os_error().unwrap_or(-1)))
            .trim()
            .parse()
            .unwrap_or_else(|e| utils::exit_err(&e, 2));
    
    println!("Enter your toss number ");
    let user_toss_number :u16 = get_player_input()
            .unwrap_or_else(|e| utils::exit_err(&e, e.raw_os_error().unwrap_or(-1)))
            .trim()
            .parse()
            .unwrap_or_else(|e| utils::exit_err(&e, 2));

    let my_number : u16 = process_user_input(&toss, user_toss_number);
    println!("My number is {}", my_number);

    let addnumbers : u16 = user_toss_number + my_number;
    println!("Added number is {} ", addnumbers);
    let toss_number_type : String;
    
    match addnumbers %2 {
        0 => {
               toss_number_type = String::from("E");
               println!("Toss number is even")
        },
        1 => {
              toss_number_type = String::from("O");
              println!("Toss number is odd")
        }
        _ => panic!("Some new problem in mathematics !!!")
    }

    if toss.to_lowercase() == toss_number_type.to_lowercase()
    {
        println!("You won the toss (Choose Bat or Bowl) !");
        let _player_choice : String = get_player_input()
            .unwrap_or_else(|e| utils::exit_err(&e, e.raw_os_error().unwrap_or(-1)))
            .trim()
            .parse()
            .unwrap_or_else(|e| utils::exit_err(&e, 2));
    }
    else
    {
        let rnd_number : u16 = utils::generate_toss();
        match rnd_number {
            0 => println!("I won the toss and will choose to bowl !"),
            1 => println!("I won the toss and will choose to bat !"),
            _ => panic!("A new bug found in random number generator !")
        }
    }
    let timeout = 4;
}

fn get_player_input() -> io::Result<String> {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_n) => {},
        Err(error) => println!("error: {}", error),
    }
    Ok(buf)
}

fn process_user_input(toss : &String, toss_number: u16) -> u16 {
    match toss.chars().next().unwrap()  {
        'o' | 'O' => {
            match toss_number % 2 {
                0 => panic!("You promised to enter odd..I am smart you can't cheat me !!"),
                _ => utils::generate_even_number(), // user selected odd so I select some even
            }
        },
        'e' | 'E' => {
            match toss_number % 2 {
                0 => utils::generate_odd_number(), // user selected even so I select some odd
                _ => panic!("You promised to enter even but you cheated !!"),
            }
        },
        _ => {
            panic!("Invalid input...please enter O or E")
        }
    }
}

